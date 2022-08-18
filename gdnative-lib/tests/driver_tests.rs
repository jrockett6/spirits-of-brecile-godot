use crate::test_client::{connect_and_send_full, TestClient};
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

mod test_client;

#[derive(Serialize, Deserialize, Debug)]
struct TestPlayer {
    x_pos: f32,
    y_pos: f32,
    health: u32,
}
// let test_player = TestPlayer {
//     x_pos: 0.5,
//     y_pos: -1.5,
//     health: 76,
// };

// let serialized = serde_json::to_value(test_player).unwrap();

// println!("{}", serialized);

#[test]
fn first_integration_test() -> std::io::Result<()> {
    // Simulate server process with a thread;
    // thread::spawn(|| {
    // let server_driver = async_driver::AsyncExecutorDriver::default();
    // server_driver.execute(Router::listen);
    // });

    thread::sleep(Duration::from_millis(500));

    // let client_driver = async_driver::AsyncExecutorDriver::default();
    // client_driver.execute(connect_and_send_full);

    thread::sleep(Duration::from_millis(500));

    // let connection = connection::Connection::default();
    // let client = TcpStream::connect("127.0.0.1:7654")?;

    Ok(())
}
