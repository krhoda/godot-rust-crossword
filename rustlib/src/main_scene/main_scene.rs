use gdnative::{
    // api::{Area2D},
    // export::{
    //     hint::{EnumHint, IntHint},
    //     Export,
    // },
    prelude::*,
};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct MainScene {}

#[methods]
impl MainScene {
    fn new(_base: &Node2D) -> Self {
        MainScene {}
    }

    #[method]
    fn _ready(&self, #[base] _base: &Node2D) {
        godot_print!("Hello from MainScene!")
    }
}
