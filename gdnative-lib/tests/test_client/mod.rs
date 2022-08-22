use gdnative_lib::server::Connection;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_serde::formats::*;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[derive(Debug)]
pub struct TestClient;
impl TestClient {
    pub async fn connect() -> Connection {
        let socket = TcpStream::connect("127.0.0.1:6142").await.unwrap();

        // Delimit frames using a length header
        let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());

        // Serialize frames with JSON
        tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalJson::<Value>::default(),
        )
    }
}
