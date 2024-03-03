use gdnative::{
    // api::{Area2D},
    // export::{
    //     hint::{EnumHint, IntHint},
    //     Export,
    // },
    prelude::*,
};
use rand::prelude::*;
use serde_json::{from_str, from_value, Map, Value};
use std::collections::HashMap;
use wasm_crossword_generator::*;

const MASTER_WORD_MAP: &str = include_str!("solutions.json");

// TODO: Think about saving and loading games?
#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Crossword {
    master_word_map: HashMap<String, Vec<Word>>,
    master_word_keys: Vec<String>,
    current_puzzle: Option<PerWordPuzzle>,
    current_key: String,
}

#[methods]
impl Crossword {
    fn new(_base: &Node2D) -> Self {
        Crossword {
            master_word_map: HashMap::new(),
            master_word_keys: Vec::new(),
            current_puzzle: None,
            current_key: "".to_string(),
        }
    }

    fn get_new_word_set(&self) -> (String, Vec<Word>) {
        let k = self
            .master_word_keys
            .choose(&mut rand::thread_rng())
            .unwrap();
        // TODO: Something more efficient than clone?
        (k.to_string(), self.master_word_map.get(k).unwrap().clone())
    }

    #[method]
    fn new_game(&mut self, #[base] base: &Node2D) {
        godot_print!("In new game");
        let (s, words) = self.get_new_word_set();
        self.current_key = s;

        let conf = SolutionConf {
            words,
            max_words: 22,
            height: 10,
            width: 10,
            initial_placement: Some(CrosswordInitialPlacement {
                min_letter_count: Some(self.current_key.len()),
                strategy: Some(CrosswordInitialPlacementStrategy::Center(
                    Direction::Horizontal,
                )),
            }),
            requirements: Some(CrosswordReqs {
                max_retries: 100,
                min_words: Some(15),
                min_letters_per_word: Some(3),
                max_empty_columns: Some(0),
                max_empty_rows: Some(0),
            }),
        };

        match PerWordPuzzle::new(conf) {
            Ok(puzzle) => {
                godot_print!("In found puzzle");
                self.current_puzzle = Some(puzzle);
                self.render_grid(base);
            }
            // NOTE: This could feasibly loop infinitely give bad luck or bad dictionary
            _ => self.new_game(base),
        };
    }

    #[method]
    fn render_grid(&mut self, #[base] base: &Node2D) {
        godot_print!("In render grid");
        if let Some(per_word_puzzle) = &self.current_puzzle {
            for (row_index, row) in per_word_puzzle.puzzle.grid.iter().enumerate() {
                for (space_index, space) in row.row.iter().enumerate() {
                    let node_path = format!("Row_{}/PuzzleTile_{}", row_index, space_index);
                    let puzzle_tile = unsafe { base.get_node_as::<ColorRect>(node_path).unwrap() };
                    let s = space.char_slot.map(|c| Some(c.to_string()));
                    unsafe {
                        puzzle_tile.call(
                            "color_and_text",
                            &[s.to_variant(), space.has_char_slot.to_variant()],
                        );
                    }
                }
            }
        }
        godot_print!("Done rendering grid");
    }

    #[method]
    fn _ready(&mut self, #[base] base: &Node2D) {
        godot_print!("Hello from Crossword!");
        let m: Map<String, Value> = from_str(MASTER_WORD_MAP).unwrap();
        let mut master_word_map = HashMap::<String, Vec<Word>>::new();
        let mut master_word_keys = Vec::<String>::new();
        for (k, v) in m {
            let words: Vec<Word> = from_value(v).unwrap();
            master_word_keys.push(k.clone());
            master_word_map.insert(k, words);
        }

        self.master_word_map = master_word_map;
        self.master_word_keys = master_word_keys;
        godot_print!("Constructed master word map");
        self.new_game(base);
    }
}
