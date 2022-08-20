use std::{collections::HashMap, rc::Rc, sync::Arc};

use futures::prelude::*;
use gdnative::{api::PhysicsServer, prelude::*};
use serde_json::Value;
use tokio::{
    io,
    net::{TcpListener, TcpStream},
    runtime::{Builder, Runtime},
    sync::{
        broadcast, mpsc,
        oneshot::{self, error::RecvError},
        watch,
    },
    time,
};
use tokio_serde::{formats::*, SymmetricallyFramed};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::player::{InputState, OutputState, Player};

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

#[derive(Clone)]
pub enum PlayerUpdateCommand {
    Create { id: String },
    Destroy { id: String },
    Update { id: String, direction: Vector3 },
}

#[derive(Clone)] // We don't actually need a multi-producer channel that requires this.
pub enum PlayerUpdateNotification {
    Create { id: String },
    Destroy { id: String },
    Update { id: String, direction: Vector3 },
}

// Attach our player state with it's physics body
struct ServerPlayer {
    player: Player,
    body: Ref<KinematicBody, Unique>,
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Server {
    players: HashMap<String, ServerPlayer>,
    physics_runtime: Runtime,
    connection_runtime: Runtime,
    rx_command: Option<broadcast::Receiver<PlayerUpdateCommand>>,
    tx_notification: Option<Arc<broadcast::Sender<PlayerUpdateNotification>>>,
}

// Main thread impl to handle scene tree related calculations on dedicated runtime.
#[methods]
impl Server {
    pub fn new(_owner: &Node) -> Self {
        Server {
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
            players: HashMap::new(),
            rx_command: None,
            tx_notification: None,
            // tx_command: tx_command,
            // rx_notification: rx_notification,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        godot_print!("SERVER!!");

        // Create tick channels
        let frame_init = 0u32;
        let (tx_tick, rx_tick) = watch::channel(frame_init);

        // Create state update command/notify channels
        let (tx_command, rx_command) =
            broadcast::channel::<PlayerUpdateCommand>(1024);
        let (tx_notification, _) =
            broadcast::channel::<PlayerUpdateNotification>(1024);

        let tx_notification = Arc::new(tx_notification);

        self.tx_notification = Some(Arc::clone(&tx_notification));
        self.rx_command = Some(rx_command);

        self.connection_runtime.spawn(Server::listen(
            rx_tick,
            tx_command,
            Arc::clone(&tx_notification),
        ));

        // Server::spawn_player();

        // Start our server ticks
        // self.runtime.spawn(Server::tick(tx_tick, frame_init));

        // Listen for client connections
        // std::thread::spawn(move || {
        //     self.runtime.block_on(self.listen(rx_tick, _owner));
        // });

        // )
    }

    #[export]
    fn _physics_process(&self, owner: &Node, _delta: f32) {
        self.physics_runtime.block_on(async {
            let mut n_msgs = self.rx_command.unwrap().len();

            while n_msgs > 0 {
                match self.rx_command.unwrap().recv().await.unwrap() {
                    PlayerUpdateCommand::Create { id } => {
                        self.spawn_player(owner, id)
                    }
                    PlayerUpdateCommand::Update { id, direction } => {
                        self.update_player(id, direction)
                    }
                    PlayerUpdateCommand::Destroy { id } => {
                        self.disconnect_player(id)
                    }
                }

                n_msgs -= 1;
            }
        })
    }

    fn spawn_player(&self, owner: &Node, id: String) {
        godot_print!("player spawned!!");

        // Add player to server, and body to scene tree
        self.players.insert(
            id,
            ServerPlayer {
                player: Player::default(),
                body: KinematicBody::new(),
            },
        );
        owner.add_child(self.players.get(&id).unwrap().body, false);

        self.tx_notification
            .unwrap()
            .send(PlayerUpdateNotification::Create { id: id });
    }

    fn update_player(&self, id: String, direction: Vector3) {
        godot_print!("player updated!! direction: {:?}", direction);

        // Clean up this logic...
        let player = self.players.get(&id).unwrap();
        player.player.update_position(&player.body, direction);

        self.tx_notification
            .unwrap()
            .send(PlayerUpdateNotification::Update {
                id: id,
                direction: direction,
            });
    }

    fn disconnect_player(&self, id: String) {
        godot_print!("player disconnected!!");

        // Free physics body memory, and remove player entry
        self.players.get(&id).unwrap().body.free(); // This may panic. Does engine free for us?
        self.players.remove(&id);

        self.tx_notification
            .unwrap()
            .send(PlayerUpdateNotification::Destroy { id: id });
    }
}

// Async fn's to handle connections on multithreaded runtime.
impl Server {
    // fn spawn_player() {
    //     let server = unsafe { PhysicsServer::godot_singleton() };

    //     server.body_create(2, false);
    //     server.body_mo

    //     godot_print!("spawned body")

    //     // PhysicsServer::body_create(&self, 2, init_sleeping)
    // }

    // Send frame updates on the tick watch channel.
    // async fn tick(tx_tick: watch::Sender<u32>, mut frame: u32) {
    //     let mut interval =
    //         time::interval(time::Duration::from_nanos(10u64.pow(9) / 30));

    //     loop {
    //         interval.tick().await;
    //         tx_tick.send(frame).unwrap();
    //         frame += 1;
    //     }
    // }

    pub async fn listen(
        rx_tick: watch::Receiver<u32>,
        tx_command: broadcast::Sender<PlayerUpdateCommand>,
        tx_notification: Arc<broadcast::Sender<PlayerUpdateNotification>>,
    ) -> io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:6142").await?;
        let mut id = 1;

        // TODO: add maximum number of accepted clients here
        loop {
            let (socket, _) = listener.accept().await?;
            println!("connecting...");
            Server::connect_player(
                id,
                socket,
                rx_tick.clone(),
                tx_command.clone(),
                tx_notification.subscribe(),
            );

            id += 1;
        }
    }

    fn connect_player(
        id: u32,
        socket: TcpStream,
        mut rx_tick: watch::Receiver<u32>,
        tx_command: broadcast::Sender<PlayerUpdateCommand>,
        rx_notification: broadcast::Receiver<PlayerUpdateNotification>,
    ) {
        // Get framed and serialized sink/stream
        let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());
        let serialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalJson::<Value>::default(),
        );
        let (sink, mut stream) = serialized.split();

        // Spawn player on main thread
        tx_command.send(PlayerUpdateCommand::Create { id: id.to_string() });

        // Create player state update channel
        // let (tx_state, mut rx_state) = mpsc::channel::<OutputState>(32);

        // Initialize the player
        // let body = KinematicBody::new();
        // let player = Player::default();

        // rx_notification.

        // Recieve input commands, and send to main physics loop
        tokio::spawn(async move {
            godot_print!("hi!!!");

            while let Some(msg) = stream.try_next().await.unwrap() {
                let input = serde_json::from_value::<InputState>(msg).unwrap();
                tx_command.send(PlayerUpdateCommand::Update {
                    id: id.to_string(),
                    direction: input.direction,
                });
                // let next_state = player.update_position(&*body, input.direction);

                // tx_state
                //     .send(OutputState {
                //         next_pos: next_state,
                //     })
                //     .await
                //     .unwrap();

                // Arc::clone(&sender).send(OutputState { next_pos: next_state });
                // if let Some(sender) = sender.take() {
                // sender.send(OutputState { next_pos: next_state }).unwrap();
                // }
            }
        });

        // Send loop
        tokio::spawn(async move {
            while rx_tick.changed().await.is_ok() {
                let next_state = rx_state.recv().await;

                godot_print!("recieved = {:?}", next_state)
            }
            // godot_print!("hey");
            Ok::<(), RecvError>(())
            // sink.send();
        });
        // TODO: Handshake here.

        // let self_mut = &mut self

        // self.players.push(Player::default());
        // Add player to player hashmap
        // Start recieving input from player
        //  - update player state
        //  - store this value in the hashmap
        //  - broadcast this update to all player tasks listening

        // async fn manage_connection(conn: Connection) {}
    }
}
