use gdnative::prelude::*;

pub mod main_scene;
use main_scene::main_scene::MainScene;

pub mod crossword;
use crossword::crossword::Crossword;

pub mod puzzle_tile;
use puzzle_tile::puzzle_tile::PuzzleTile;

// Registers all exposed classes to Godot.
fn init(handle: InitHandle) {
    handle.add_class::<Crossword>();
    handle.add_class::<MainScene>();
    handle.add_class::<PuzzleTile>();
}

// Creates entry-points of dyn lib.
godot_init!(init);
