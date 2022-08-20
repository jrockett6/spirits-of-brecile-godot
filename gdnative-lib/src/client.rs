use futures::prelude::*;
use gdnative::prelude::Vector3;
use serde_json::{json, Value};
use tokio::{io, net::TcpStream};
use tokio_serde::formats::*;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::{player::InputState, server::Connection};

#[derive(Debug)]
struct Client;

impl Client {
    pub async fn connect() -> io::Result<()> {
        let socket = TcpStream::connect("127.0.0.1:6142").await.unwrap();

        // Delimit frames using a length header
        let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());

        // Serialize frames with JSON
        let mut serialized: Connection = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalJson::<Value>::default(),
        );

        let input1 = InputState {
            direction: Vector3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        };

        let json = serde_json::to_value(&input1)?;

        serialized.send(json);

        Ok(())
    }
}
