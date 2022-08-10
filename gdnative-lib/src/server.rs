use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Server;
impl Server {
    fn new(_owner: &Node) -> Self {
        Server
    }
}

#[methods]
impl Server {
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("SERVER!!")
    }


}
