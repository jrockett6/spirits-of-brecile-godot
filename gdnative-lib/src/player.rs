use bevy_ecs::prelude::*;
use gdnative::prelude::*;

use crate::{character::Body, input::handle_input, movement::Speed};

#[derive(Default, NativeClass)]
#[inherit(KinematicBody)]
pub struct Player {
    schedule: Schedule,
    world: World,
}

impl Player {
    fn new(_owner: &KinematicBody) -> Self {
        Self {
            schedule: Schedule::default(),
            world: World::new(),
        }
    }
}

#[methods]
impl Player {
    #[export]
    fn _ready(&mut self, owner: &KinematicBody) {
        godot_print!("Player ready!!");

        // Spawn a player entity, with Speed and Body components
        self.world.spawn().insert(Speed::default()).insert(unsafe {
            Body {
                body: owner.assume_shared(),
            }
        });

        self.schedule.add_stage(
            "update",
            SystemStage::parallel().with_system(handle_input),
        );
    }

    #[export]
    fn _physics_process(
        &mut self,
        _owner: &KinematicBody,
        _delta: f32,
    ) {
        self.schedule.run(&mut self.world)
    }
}

// fn init_ecs() {
//     // Create a new empty World to hold our Entities and Components
//     let mut world = World::new();

//     // Spawn an entity with Position and Velocity components
//     world.spawn();
//     // .insert(Dama { x: 0.0, y: 0.0 })
//     // .insert(Velocity { x: 1.0, y: 0.0 });

//     // Create a new Schedule, which defines an execution strategy for Systems
//     let mut schedule = Schedule::default();

//     // Add a Stage to our schedule. Each Stage in a schedule runs all of its systems
//     // before moving on to the next Stage
//     schedule.add_stage("update", SystemStage::;
//     // .with_system(movement));

//     // Run the schedule once. If your app has a "loop", you would run this once per loop
//     schedule.run(&mut world);
// }
