mod utils;

use wasm_bindgen::prelude::*;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub enum Position {
    Up,
    Right,
    Down,
    Left
}

pub enum ShapeType {
    Bar,
    Square,
    Hat,
    Leg,
    ReverseLeg,
    Step,
    ReverseStep,
}

#[wasm_bindgen]
pub struct Tetromino {
    block1: u32,
    block2: u32,
    block3: u32,
    block4: u32,
    position: Position,
    shape_type: ShapeType,
}

#[wasm_bindgen]
impl Tetromino {
    pub fn new_bar_shape() -> Tetromino {
        utils::set_panic_hook();
        let block1 = 5;
        let block2 = block1 + 1;
        let block3 = block1 + 2;
        let block4 = block1 + 3;
        let position = Position::Right;
        let shape_type = ShapeType::Bar;

        Tetromino {
            block1,
            block2,
            block3,
            block4,
            position,
            shape_type,
        }
    }
}

#[wasm_bindgen]
pub struct TetrisBoard {
    space_used: Vec<u32>,
    height: u32,
    width: u32,
    current_x_positions: Vec<u32>,
    current_y_positions: Vec<u32>,
}

#[wasm_bindgen]
impl TetrisBoard {
    pub fn new() -> TetrisBoard {
        utils::set_panic_hook();
        let space_used = vec![0; 200];
        let height = 20;
        let width = 10;
        let current_x_positions = vec![];
        let current_y_positions = vec![];

        TetrisBoard {
            space_used,
            height,
            width,
            current_x_positions,
            current_y_positions,
        }
    }

    pub fn generate_start_position(&mut self, tetronimo: &Tetromino){
        let board_spaces = [tetronimo.block1, tetronimo.block2, tetronimo.block3, tetronimo.block4];
        for space in board_spaces.iter() {
            let mut counter = 0;
            for element in &mut self.space_used.iter_mut(){
                if counter == *space as usize {
                    *element = 1 as u32;
                }
                counter += 1;
            }
        }
        log!("{:?}", self.space_used)
    }
}
