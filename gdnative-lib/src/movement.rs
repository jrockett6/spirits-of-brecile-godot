use bevy_ecs::prelude::*;
use gdnative::prelude::*;

#[derive(Component)]
pub struct Speed {
    y_speed: f32,

    //Constants
    pub speed: f32,
    pub fall_speed: f32,
}

impl Default for Speed {
    fn default() -> Self {
        Speed {
            y_speed: 0.0,
            speed: 20.0,
            fall_speed: 25.0,
        }
    }
}

pub fn handle_movement(
    body: Ref<KinematicBody, Unique>,
    direction: Vector3,
    speed: &Speed,
) {
    let normalized_direction = if direction == Vector3::ZERO {
        direction
    } else {
        direction.normalized()
    };

    // godot_print!(
    //     "[Player::update_position] Initial location: {:?}",
    //     body.global_transform().origin
    // );

    let velocity = Vector3 {
        x: normalized_direction.x * speed.speed,
        y: speed.y_speed - speed.fall_speed,
        z: normalized_direction.z * speed.speed,
    };

    let velocity = body.move_and_slide(
        velocity,
        Vector3::UP,
        false,
        4,
        0.785398,
        true,
    );

    // body.global_transform().origin
}
