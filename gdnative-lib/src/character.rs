use bevy_ecs::prelude::*;
use gdnative::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(remote = "Vector3")]
pub struct Vector3Def {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputState {
    #[serde(with = "Vector3Def")]
    pub direction: Vector3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputState {
    #[serde(with = "Vector3Def")]
    pub next_pos: Vector3,
}

#[derive(Component)]
pub struct Body {
    pub body: Ref<KinematicBody>,
}

// #[derive(Bundle)]
// struct Character {
//     speed: Speed,
//     body: Body,
// }

// #[derive(Debug)]
// pub struct Character {
//     health: i32,
//     y_speed: f32,

//     //Constants
//     pub speed: f32,
//     pub fall_speed: f32,
// }

// impl Default for Character {
//     fn default() -> Self {
//         Character {
//             health: 100,
//             y_speed: 0.0,
//             speed: 20.0,
//             fall_speed: 25.0,
//         }
//     }
// }

// impl Character {
//     pub fn update_position(
//         &self,
//         body: Ref<KinematicBody, Unique>,
//         direction: Vector3,
//     ) -> Vector3 {
//         let normalized_direction = if direction == Vector3::ZERO {
//             direction
//         } else {
//             direction.normalized()
//         };

//         // godot_print!(
//         //     "[Player::update_position] Initial location: {:?}",
//         //     body.global_transform().origin
//         // );

//         let velocity = Vector3 {
//             x: normalized_direction.x * self.speed,
//             y: self.y_speed - self.fall_speed,
//             z: normalized_direction.z * self.speed,
//         };

//         let velocity = body.move_and_slide(
//             velocity,
//             Vector3::UP,
//             false,
//             4,
//             0.785398,
//             true,
//         );
// body.global_transform().origin

// godot_print!(
//     "[Player::update_position] Applied velocity: {:?}",
//     velocity
// );

// godot_print!(
//     "[Player::update_position] Updated location: {:?}",
//     body.global_transform().origin
// );

// self.y_speed = velocity.y;
// _owner.global_transform().origin
// godot_print!(
//     "before pos: {:?}",
//     player.assume_safe().transform().origin
// );

// unsafe {
// }

// godot_print!("Player pos:   {:?}", self.position);
// godot_print!(
//     "after pos:  {:?}",
//     player.assume_safe().transform().origin + vel
// );
//     }
// }
