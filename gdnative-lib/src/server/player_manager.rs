use gdnative::prelude::*;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::broadcast;

use crate::player::{OutputState, Player};

#[derive(Clone, Debug)]
pub enum PlayerUpdateCommand {
    Create { id: i64 },
    Destroy { id: i64 },
    Update { id: i64, direction: Vector3 },
}

#[derive(Clone, Debug)] // We don't actually need a multi-producer channel that requires Clone.
pub enum PlayerUpdateNotification {
    Create { id: i64 },
    Destroy { id: i64 },
    Update { id: i64, output_state: OutputState },
}

// Attach our player state with it's physics body
struct ServerPlayer {
    player: Player,
    body_idx: i64,
}

// We can't close over `self` in _physics_process when calling player state update functions,
// so we abstract the functions out to this class.
// See: https://stackoverflow.com/questions/64921625/closure-requires-unique-access-to-self-but-it-is-already-borrowed#comment114785595_64921799
pub struct PlayerManager {
    body_idx: i64,
    players: HashMap<i64, ServerPlayer>,
    pub rx_command: Option<broadcast::Receiver<PlayerUpdateCommand>>,
    pub tx_notification:
        Option<Arc<broadcast::Sender<PlayerUpdateNotification>>>,
}

impl Default for PlayerManager {
    fn default() -> Self {
        Self {
            body_idx: 1,
            players: HashMap::new(),
            rx_command: None,
            tx_notification: None,
        }
    }
}

impl PlayerManager {
    pub fn spawn_player(&mut self, owner: &Node, id: i64) {
        // Add body to scene tree, and player to PlayerManager
        owner.add_child(KinematicBody::new(), false);
        self.players.insert(
            id,
            ServerPlayer {
                player: Player::default(),
                body_idx: self.body_idx,
            },
        );
        self.body_idx += 1;

        for child in &owner.get_children() {
            godot_print!("child: {}", child);
        }

        self.tx_notification
            .as_ref()
            .unwrap()
            .send(PlayerUpdateNotification::Create { id: id })
            .unwrap();

        godot_print!("[PlayerManager::spawn_player] Player spawned!!");
    }

    pub fn update_player(&self, owner: &Node, id: i64, direction: Vector3) {
        let player = self.players.get(&id).unwrap();

        let next_pos;
        unsafe {
            let body = owner
                .get_child(player.body_idx)
                .unwrap()
                .assume_unique()
                .try_cast::<KinematicBody>()
                .unwrap();

            next_pos = player.player.update_position(body, direction);
        }

        self.tx_notification
            .as_ref()
            .unwrap()
            .send(PlayerUpdateNotification::Update {
                id: id,
                output_state: OutputState { next_pos: next_pos },
            })
            .unwrap();

        godot_print!(
            "[PlayerManager::update_player] Player updated!! direction: {:?}",
            direction
        );
    }

    pub fn disconnect_player(&mut self, id: i64) {
        // Free physics body memory, and remove player entry
        // let body = &self.players.get(&id).unwrap().body.free; // TODO: Does engine free for us?
        self.players.remove(&id);

        self.tx_notification
            .as_ref()
            .unwrap()
            .send(PlayerUpdateNotification::Destroy { id: id })
            .unwrap();

        godot_print!(
            "[PlayerManager::disconnect_player] Player disconnected!!"
        );
    }
}
