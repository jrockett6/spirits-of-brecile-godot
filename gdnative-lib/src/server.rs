use futures::prelude::*;
use gdnative::prelude::*;
use serde_json::Value;
use std::sync::Arc;
use tokio::{
    io,
    net::{TcpListener, TcpStream},
    runtime::{Builder, Runtime},
    sync::broadcast,
};
use tokio_serde::{formats::*, SymmetricallyFramed};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::player::{InputState, OutputState};
use crate::server::player_manager::{
    PlayerManager, PlayerUpdateCommand, PlayerUpdateNotification,
};

pub mod player_manager;

pub type Connection = SymmetricallyFramed<
    Framed<TcpStream, LengthDelimitedCodec>,
    Value,
    Json<Value, Value>,
>;

#[derive(Debug)]
enum ConnectionCommand {
    Send { pos: Vector3 },
    Recv { direction: Vector3 },
}

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
        godot_print!("SERVER!!");

        // Create state update command/notify channels
        let (tx_command, rx_command) =
            broadcast::channel::<PlayerUpdateCommand>(1024);
        let (tx_notification, _) =
            broadcast::channel::<PlayerUpdateNotification>(1024);
        let tx_notification = Arc::new(tx_notification);

        self.player_manager.tx_notification =
            Some(Arc::clone(&tx_notification));
        self.player_manager.rx_command = Some(rx_command);

        // Spawn connection task
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
        let listener = TcpListener::bind("127.0.0.1:6142").await?;
        let mut id = 1;

        // TODO: add maximum number of accepted clients here
        loop {
            let (socket, _) = listener.accept().await?;
            godot_print!("[Server::listen] New player {} connecting...", id);
            Server::connect_player(
                id,
                socket,
                tx_command.clone(),
                tx_notification.subscribe(),
            );

            id += 1;
        }
    }

    fn connect_player(
        id: i64,
        socket: TcpStream,
        tx_command: broadcast::Sender<PlayerUpdateCommand>,
        mut rx_notification: broadcast::Receiver<PlayerUpdateNotification>,
    ) {
        // Get framed and serialized sink/stream
        let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());
        let serialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalJson::<Value>::default(),
        );
        let (mut sink, mut stream) = serialized.split();

        // Spawn player on main thread
        tx_command
            .send(PlayerUpdateCommand::Create { id: id })
            .unwrap();

        // Recieve input commands, and send to main physics loop
        tokio::spawn(async move {
            while let Some(msg) = stream.try_next().await.unwrap() {
                let input = serde_json::from_value::<InputState>(msg).unwrap();
                tx_command
                    .send(PlayerUpdateCommand::Update {
                        id: id,
                        direction: input.direction,
                    })
                    .unwrap();
            }
        });

        // Recieve physics loop updates, send to client
        tokio::spawn(async move {
            while let Ok(update) = rx_notification.recv().await {
                match update {
                    PlayerUpdateNotification::Create { id } => {
                        godot_print!(
                            "[Server::connect_player] player {} created!!",
                            id
                        )
                    }
                    PlayerUpdateNotification::Destroy { id } => {
                        godot_print!(
                            "[Server::connect_player] player {} destroyed!!",
                            id
                        )
                    }
                    PlayerUpdateNotification::Update { id, output_state } => {
                        let result =
                            serde_json::to_value(output_state).unwrap();

                        sink.send(result).await.unwrap();
                    }
                };
            }
        });
        // TODO:
        //  - handshake?
        //  - broadcast this update to all player tasks listening
    }
}
