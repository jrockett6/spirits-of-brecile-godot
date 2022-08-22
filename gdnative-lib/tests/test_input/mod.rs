use gdnative::prelude::Vector3;
use gdnative_lib::player::InputState;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Clone)]
pub struct TestInput {
    pub input1: Value,
}

impl Default for TestInput {
    fn default() -> Self {
        Self {
            input1: serde_json::to_value(InputState {
                direction: Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
            })
            .unwrap(),
        }
    }
}
