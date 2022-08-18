use gdnative::prelude::*;

use crate::player::Player;

#[derive(NativeClass)]
pub struct ClientNetwork {}

impl ClientNetwork {
    fn new(_owner: &Reference) -> Self {
        ClientNetwork {}
    }

    fn send_action() {}

    fn reconcile_state() {}

    fn connect(&self) {}
}

#[methods]
impl ClientNetwork {
    #[export]
    fn handle_input(
        &mut self,
        _owner: &Reference,
        body: Ref<KinematicBody>,
        direction: Vector3,
        delta: f32,
    ) {
        // self.player.update_position(body, direction, delta);
    }
}

//* This is a workaround for the limitation on constructor params for GDNative classes */
// #[derive(NativeClass)]
// #[inherit(Reference)]
// #[no_constructor]
// pub struct ObjectFactory {}
// #[methods]
// impl ObjectFactory {
//     #[export]
//     fn client_network(
//         &self,
//         _owner: &Reference,
//     ) -> Instance<ClientNetwork, Unique> {
//         ClientNetwork {
//             player: PlayerHandler {
//                 player: Player::default(),
//             },
//         }
//         .emplace()
//     }
// }
