use futures::prelude::*;
use serde_json::json;
use tokio::io;
use tokio::net::TcpStream;
use tokio_serde::formats::*;
use tokio_serde::Framed;
use tokio_serde::SymmetricallyFramed;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

// type LengthDelimited = FramedWrite<TcpStream, LengthDelimitedCodec>;
type LengthDelimited<T, J> = SymmetricallyFramed<
    FramedWrite<TcpStream, LengthDelimitedCodec>,
    T,
    SymmetricalJson<J>,
>;

// #[derive(Default)]
#[cfg(test)]
pub struct TestClient;
impl TestClient {
    async fn connect() -> TcpStream {
        TcpStream::connect("127.0.0.1:6142").await.unwrap()
    }

    async fn send(socket: TcpStream, item: serde_json::Value) {
        let length_delimited =
            FramedWrite::new(socket, LengthDelimitedCodec::new());

        // Serialize frames with JSON
        let mut serialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalJson::default(),
        );

        // Send the value
        serialized.send(item).await.unwrap();
    }
}

pub async fn connect_and_send_full() -> io::Result<()> {
    let socket = TestClient::connect().await;
    let json = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    TestClient::send(socket, json).await;

    Ok(())
}
