use crate::player::Player;
use futures::prelude::*;
use gdnative::prelude::*;
use serde_json::{json, Value};
use tokio::{
    io,
    net::{TcpListener, TcpStream},
    runtime::{Runtime, Builder},
    sync::oneshot,
};
use tokio_serde::{formats::*, SymmetricallyFramed};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

type Connection = SymmetricallyFramed<
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
    runtime: Runtime,
}

#[methods]
impl Server {
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("SERVER!!");

        self.runtime.spawn(Server::listen());

        // match Server::listen() {
        //     Ok(_) => println!("Server listening..."),
        //     Err(e) => println!("Error: {}", e.to_string()),
        // }
    }
}

impl Server {
    pub fn new(_owner: &Node) -> Self {
        Server {
            runtime: Builder::new_multi_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap(),
        }
    }

    pub async fn listen() -> io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:6142").await?;

        // TODO: add maximum number of accepted clients here
        loop {
            let (socket, _) = listener.accept().await?;
            println!("connecting...");
            Server::connect_player(socket);
        }
    }

    fn connect_player(socket: TcpStream) {
        let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());
        let serialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalJson::<Value>::default(),
        );

        let (sink, mut stream) = serialized.split();

        let body = KinematicBody::new();
        let player = Player::default();

        // player.update_position(&body, Vector3::default(), 0.1);

        tokio::spawn(async move {
            godot_print!("hi!!!");
            
            while let Some(msg) = stream.try_next().await.unwrap() {
                godot_print!("GOT: {:?}", msg);
            }
        });

        tokio::spawn(async move {
            godot_print!("hey");
            // sink.send();
        });
        // TODO: Handshake herego.

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
