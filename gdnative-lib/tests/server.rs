use std::thread;
use std::time::Duration;

use futures::prelude::*;
use gdnative::prelude::Vector3;
use gdnative_lib::{character::OutputState, server::Connection};

use crate::test_client::TestClient;
use crate::test_input::TestInput;
use crate::utils::Utils;

mod test_client;
mod test_input;
mod utils;

#[test]
fn full_update_once() {
    Utils::go(async {
        let input = TestInput::default();
        let mut conn: Connection = TestClient::connect().await;

        // Send/recieve input/updated state
        conn.send(input.input1).await.unwrap();
        let updated_state = serde_json::from_value::<OutputState>(
            conn.try_next().await.unwrap().unwrap(),
        )
        .unwrap();

        conn.close().await.unwrap();

        assert_eq!(
            updated_state.next_pos,
            Vector3 {
                x: 0.47140455,
                y: -0.077700004,
                z: 0.47140455
            }
        );
    });
}

#[test]
fn full_update_twice() {
    Utils::go(async {
        let input = TestInput::default();
        let mut conn: Connection = TestClient::connect().await;

        // Send/recieve input/updated state
        conn.send(input.input1.clone()).await.unwrap();
        let updated_state = serde_json::from_value::<OutputState>(
            conn.try_next().await.unwrap().unwrap(),
        )
        .unwrap();

        assert_eq!(
            updated_state.next_pos,
            Vector3 {
                x: 0.47140455,
                y: -0.077700004,
                z: 0.47140455
            }
        );

        // Send/recieve input/updated state
        conn.send(input.input1).await.unwrap();
        let updated_state = serde_json::from_value::<OutputState>(
            conn.try_next().await.unwrap().unwrap(),
        )
        .unwrap();

        assert_eq!(
            updated_state.next_pos,
            Vector3 {
                x: 0.9428091,
                y: -0.15540001,
                z: 0.9428091
            }
        );

        // Sleep to makes sure client sees response from server
        thread::sleep(Duration::from_millis(500));
    });
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
