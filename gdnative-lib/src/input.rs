use bevy_ecs::prelude::*;
use gdnative::prelude::*;

use crate::{character::Body, movement::Speed};

pub fn handle_input(query: Query<(&Body, &Speed)>) {
    godot_print!("handle_input here!!");

    for (body, speed) in &query {
        let mut direction = Vector3::ZERO;
        let basis = unsafe {
            body.body.assume_safe().global_transform().basis
        };

        if gdnative::prelude::Input::is_action_pressed(
            gdnative::prelude::Input::godot_singleton(),
            "move_forward",
            false,
        ) {
            direction += basis.c()
        }

        if gdnative::prelude::Input::is_action_pressed(
            gdnative::prelude::Input::godot_singleton(),
            "move_backward",
            false,
        ) {
            direction -= basis.c()
        }

        if gdnative::prelude::Input::is_action_pressed(
            gdnative::prelude::Input::godot_singleton(),
            "move_left",
            false,
        ) {
            direction += basis.a()
        }

        if gdnative::prelude::Input::is_action_pressed(
            gdnative::prelude::Input::godot_singleton(),
            "move_right",
            false,
        ) {
            direction -= basis.a()
        }

        crate::movement::handle_movement(
            unsafe { body.body.assume_unique() },
            direction,
            speed,
        );
    }

    // if Input.is_action_pressed("move_forward"):
    // 	direction += player.global_transform.basis.z
    // if Input.is_action_pressed("move_back"):
    // 	direction += -player.global_transform.basis.z
    // if Input.is_action_pressed("move_left"):
    // 	direction += player.global_transform.basis.x
    // if Input.is_action_pressed("move_right"):
    // 	direction += -player.global_transform.basis.x
}
