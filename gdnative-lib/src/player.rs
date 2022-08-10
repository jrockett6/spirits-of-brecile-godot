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

pub struct PlayerHandler {
    pub player: Player,
}

impl PlayerHandler {
    pub fn update_position(
        &mut self,
        body: Ref<KinematicBody>,
        direction: Vector3,
        delta: f32,
    ) {
        let normalized_direction = if direction == Vector3::ZERO {
            direction
        } else {
            direction.normalized()
        };

        let input_velocity = Vector3 {
            x: normalized_direction.x * self.player.speed,
            y: self.player.y_speed - self.player.fall_speed * delta,
            z: normalized_direction.z * self.player.speed,
        };

        let actual_velocity: Vector3;

        // godot_print!(
        //     "before pos: {:?}",
        //     player.assume_safe().transform().origin
        // );

        unsafe {
            actual_velocity = body.assume_safe().move_and_slide(
                input_velocity,
                Vector3::UP,
                false,
                4,
                0.785398,
                true,
            );
        }

        self.player.y_speed = actual_velocity.y;
        
        // godot_print!("Player pos:   {:?}", self.position);
        // godot_print!(
        //     "after pos:  {:?}",
        //     player.assume_safe().transform().origin + vel
        // );
    }
}
