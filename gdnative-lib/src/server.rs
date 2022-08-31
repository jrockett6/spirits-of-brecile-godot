use futures::prelude::*;
use gdnative::prelude::*;
use serde_json::Value;
use std::sync::Arc;
use tokio::{
    io,
    net::{TcpListener, TcpStream},
    runtime::{Builder, Runtime},
    sync::{broadcast},
};
use tokio_serde::{formats::*, SymmetricallyFramed};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::{debug, info, warn, Level};

use crate::character::{InputState, OutputState};
use crate::server::player_manager::{
    PlayerManager, PlayerUpdateCommand, PlayerUpdateNotification,
};

pub mod player_manager;

pub type Connection = SymmetricallyFramed<
    Framed<TcpStream, LengthDelimitedCodec>,
    Value,
    Json<Value, Value>,
>;

pub type ServerSubscriber = tracing_subscriber::FmtSubscriber<
    tracing_subscriber::fmt::format::JsonFields,
    tracing_subscriber::fmt::format::Format<
        tracing_subscriber::fmt::format::Json,
        (),
    >,
    tracing::metadata::LevelFilter,
    tracing_appender::non_blocking::NonBlocking,
>;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Server {
    player_manager: PlayerManager,
    physics_runtime: Runtime,
    connection_runtime: Runtime,
}

// Main thread impl to handle scene tree related calculations on dedicated runtime.
#[methods]
impl Server {
    pub fn new(_owner: &Node) -> Self {
        Server {
            player_manager: PlayerManager::default(),
            physics_runtime: Builder::new_current_thread()
                .enable_io()
                // .enable_time()
                .build()
                .unwrap(),
            connection_runtime: Builder::new_multi_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        godot_print!("SERVER READY");
        godot_print!(
            "Thread: {:?}, {:?}",
            std::thread::current().id(),
            std::thread::current().name()
        );

        // Create state update command/notify channels
        // Command channel is broadcast because mpsc doesn't let you query for queue length.
        let (tx_command, rx_command) =
            broadcast::channel::<PlayerUpdateCommand>(1024);
        let (tx_notification, _) =
            broadcast::channel::<PlayerUpdateNotification>(1024);
        let tx_notification = Arc::new(tx_notification);

        self.player_manager.tx_notification =
            Some(Arc::clone(&tx_notification));
        self.player_manager.rx_command = Some(rx_command);

        self.connection_runtime
            .spawn(Server::listen(tx_command, Arc::clone(&tx_notification)));

        // Start our server ticks
        // let frame_init = 0u32;
        // let (tx_tick, rx_tick) = watch::channel(frame_init);
        // self.runtime.spawn(Server::tick(tx_tick, frame_init));
    }

    #[export]
    fn _physics_process(&mut self, owner: &Node, _delta: f32) {
        let player_manager = &mut self.player_manager;

        self.physics_runtime.block_on(async {
            let mut n_msgs = player_manager.rx_command.as_ref().unwrap().len();

            while n_msgs > 0 {
                match player_manager
                    .rx_command
                    .as_mut()
                    .unwrap()
                    .recv()
                    .await
                    .unwrap()
                {
                    PlayerUpdateCommand::Create { id } => {
                        player_manager.spawn_player(owner, id.clone())
                    }
                    PlayerUpdateCommand::Update { id, direction } => {
                        player_manager.update_player(
                            owner,
                            id.clone(),
                            direction.clone(),
                        )
                    }
                    PlayerUpdateCommand::Destroy { id } => {
                        player_manager.disconnect_player(id.clone())
                    }
                }

                n_msgs -= 1;
            }
        });
    }
}

// Async fn's to handle connections on multithreaded runtime.
impl Server {
    pub async fn listen(
        tx_command: broadcast::Sender<PlayerUpdateCommand>,
        tx_notification: Arc<broadcast::Sender<PlayerUpdateNotification>>,
    ) -> io::Result<()> {
        // Configure tracing. Currently done here to avoid having the subscriber dropped.
        let file_appender = tracing_appender::rolling::minutely(
            "/Users/jrockett/workspace/spirits-of-brecile/gdnative-lib/target/tmp",
            "server.log",
        );

        let (non_blocking, _guard) =
            tracing_appender::non_blocking(file_appender);

        let subscriber = tracing_subscriber::fmt()
            // .json()
            // .flatten_event(true)
            // .with_current_span(true)
            // .with_span_list(false)
            // .pretty()
            // .with_thread_names(true)
            .with_target(false)
            .with_thread_ids(true)
            .with_writer(non_blocking)
            .with_max_level(Level::DEBUG)
            .without_time()
            .finish();

        tracing::subscriber::set_global_default(subscriber).unwrap();

        // Listen for client connections
        let listener = TcpListener::bind("127.0.0.1:6142").await?;
        let mut id = 1;

        // TODO: add maximum number of accepted clients here
        loop {
            let (socket, _) = listener.accept().await?;
            Server::connect_player(
                id,
                socket,
                tx_command.clone(),
                tx_notification.subscribe(),
            );

            id += 1;
        }
    }

    #[tracing::instrument(
        level = "debug"
        name = "Server::connect_player"
        skip(socket, tx_command, rx_notification),
    )]
    fn connect_player(
        id: i64,
        socket: TcpStream,
        tx_command: broadcast::Sender<PlayerUpdateCommand>,
        rx_notification: broadcast::Receiver<PlayerUpdateNotification>,
    ) {
        info!("player {} connected", id);

        // Get framed and serialized sink/stream
        let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());
        let serialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalJson::<Value>::default(),
        );
        let (sink, stream) = serialized.split();

        // Spawn player on main thread
        tx_command
            .send(PlayerUpdateCommand::Create { id: id })
            .unwrap();

        // Recieve input commands, and send to main physics loop
        tokio::spawn(Self::recv_task(id, stream, tx_command));

        // Recieve physics loop updates, send to client
        tokio::spawn(Self::send_task(id, sink, rx_notification));

        // TODO:
        //  - handshake?
        //  - write a multi-client test
    }

    #[tracing::instrument(
        level = "debug"
        name = "Server::recv_task"
        skip(stream, tx_command),
    )]
    async fn recv_task(
        id: i64,
        mut stream: stream::SplitStream<Connection>,
        tx_command: broadcast::Sender<PlayerUpdateCommand>,
    ) {
        while let Some(msg) = stream.try_next().await.unwrap() {
            let input = serde_json::from_value::<InputState>(msg).unwrap();
            debug!(
                "got player {} input: {:?}, sending to PlayerManager",
                id, input
            );

            tx_command
                .send(PlayerUpdateCommand::Update {
                    id: id,
                    direction: input.direction,
                })
                .unwrap();
        }
        info!("client closed channel, dropping recv_task");
        tx_command
            .send(PlayerUpdateCommand::Destroy { id: id })
            .unwrap();
    }

    #[tracing::instrument(
        level = "debug"
        name = "Server::send_task"
        skip(sink, rx_notification),
    )]
    async fn send_task(
        _id: i64,
        mut sink: stream::SplitSink<Connection, Value>,
        mut rx_notification: broadcast::Receiver<PlayerUpdateNotification>,
    ) {
        while let Ok(update) = rx_notification.recv().await {
            match update {
                PlayerUpdateNotification::Create { id } => {
                    debug!("created player {}", id)
                }
                PlayerUpdateNotification::Update { id, output_state } => {
                    debug!("updated player {}: {:?}", id, output_state);
                    let result = serde_json::to_value(output_state).unwrap();
                    match sink.send(result).await {
                        Ok(()) => (),
                        Err(e) => warn!("{}", e),
                    }
                }
                PlayerUpdateNotification::Destroy { id } => {
                    debug!("destroyed player {}", id);
                    if id == _id {
                        break;
                    }
                }
            };
        }
        info!("player was destroyed (or got RecvError), dropping send_task");
    }
}
