use gdnative::prelude::*;

pub struct Player {
    y_speed: f32,

    //Constants
    pub speed: f32,
    pub fall_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            y_speed: 0.0,
            speed: 20.0,
            fall_speed: 70.0,
        }
    }
}

impl Player {
    fn new(_owner: &KinematicBody) -> Self {
        Self::default()
    }

    pub fn update_position(
        &self,
        body: &KinematicBody,
        direction: Vector3,
        delta: f32,
    ) {
        let normalized_direction = if direction == Vector3::ZERO {
            direction
        } else {
            direction.normalized()
        };

        let velocity = Vector3 {
            x: normalized_direction.x * self.speed,
            y: self.y_speed - self.fall_speed * delta,
            z: normalized_direction.z * self.speed,
        };

        let velocity = body.move_and_slide(
            velocity,
            Vector3::UP,
            false,
            4,
            0.785398,
            true,
        );

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
    }
}
