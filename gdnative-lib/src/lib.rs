mod server;
mod player;
mod client_network;

use client_network::{ClientNetwork, ObjectFactory};
use gdnative::prelude::*;
use server::Server;
// use player::PlayerAPI;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {    
    handle.add_class::<Server>();
    handle.add_class::<ClientNetwork>();
    // handle.add_class::<ObjectFactory>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);