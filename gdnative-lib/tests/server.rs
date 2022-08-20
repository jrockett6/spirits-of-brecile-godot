use futures::prelude::*;
use gdnative::prelude::Vector3;
use gdnative_lib::{player::InputState, server::Connection};
use serde::{Deserialize, Serialize};
use tokio::{
    io,
    runtime::{Builder, Runtime},
};

use crate::test_client::TestClient;

mod test_client;

#[derive(Serialize, Deserialize, Debug)]
struct TestPlayer {
    x_pos: f32,
    y_pos: f32,
    health: u32,
}

struct Utils {
    runtime: Runtime,
}
impl Default for Utils {
    fn default() -> Self {
        Self {
            runtime: Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap(),
        }
    }
}
impl Utils {
    fn go<F: Future>(f: F) -> io::Result<()> {
        let rt = Utils::default().runtime;
        rt.block_on(f);

        Ok(())
    }
}

// let test_player = TestPlayer {
//     x_pos: 0.5,
//     y_pos: -1.5,
//     health: 76,
// };

// let serialized = serde_json::to_value(test_player).unwrap();

// println!("{}", serialized);

#[test]
fn client_send_full() -> io::Result<()> {
    // Create input data
    let input1 = InputState {
        direction: Vector3 {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let json = serde_json::to_value(&input1)?;

    let test = async {
        let mut conn: Connection = TestClient::connect().await;

        // Send input to server
        conn.send(json).await.unwrap();

        // Get updated position
        let next_pos = serde_json::from_value::<InputState>(
            conn.try_next().await.unwrap().unwrap(),
        )
        .unwrap();

        println!("{:?}", next_pos);
        // TestClient::roundtrip_full(conn).await.unwrap();
    };

    Utils::go(test)
}

// Simulate server process with a thread;
// thread::spawn(|| {
// let server_driver = async_driver::AsyncExecutorDriver::default();
// server_driver.execute(Router::listen);
// });

// thread::sleep(Duration::from_millis(500));

// let client_driver = async_driver::AsyncExecutorDriver::default();
// client_driver.execute(connect_and_send_full);

// thread::sleep(Duration::from_millis(500));

// let connection = connection::Connection::default();
// let client = TcpStream::connect("127.0.0.1:7654")?;
