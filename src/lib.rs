mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

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
#[derive(Debug)]
pub enum Position {
    Up,
    Right,
    Down,
    Left
}


#[wasm_bindgen]
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Tetromino {
    block_size: u32,
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
        let block_size = 50;
        let block1 = 5;
        let block2 = block1 + 1;
        let block3 = block1 + 2;
        let block4 = block1 + 3;
        let position = Position::Right;
        let shape_type = ShapeType::Bar;

        Tetromino {
            block_size,
            block1,
            block2,
            block3,
            block4,
            position,
            shape_type,
        }
    }

    pub fn get_block_size(&self) -> u32 {
        self.block_size
    }

    pub fn transform_bar(&mut self, board: &TetrisBoard){
        match self.position {
            Position::Right => {
                self.position = Position::Down;
                self.block2 = self.block1 + board.width;
                self.block3 = self.block1 + (2 * board.width);
                self.block4 = self.block1 + (3 * board.width);
            },
            Position::Down => {
                self.position = Position::Right;
                self.block2 = self.block1 + 1;
                self.block3 = self.block1 + 2;
                self.block4 = self.block1 + 3;
            },
            _ => ()
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct TetrisBoard {
    space_used: Vec<u32>,
    height: u32,
    width: u32,
}

#[wasm_bindgen]
impl TetrisBoard {
    pub fn new() -> TetrisBoard {
        utils::set_panic_hook();
        let space_used = vec![0; 200];
        let height = 20;
        let width = 10;

        TetrisBoard {
            space_used,
            height,
            width,
        }
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_space_used(&self) -> *const u32 {
        self.space_used.as_ptr()
    }

    pub fn get_space_length(&self) -> usize {
        self.space_used.len()
    }

    pub fn update_shape_position(&mut self, tetronimo: &mut Tetromino) {
        tetronimo.transform_bar(&self);
        self.generate_start_position(tetronimo);
    }

    pub fn generate_start_position(&mut self, tetronimo: &Tetromino){
        for element in self.space_used.iter_mut() {
            if *element == 1 {
                *element = 0
            }
        }
        let board_spaces = [tetronimo.block1, tetronimo.block2, tetronimo.block3, tetronimo.block4];
        log!("{:?}", board_spaces);
        for space in board_spaces.iter() {
            let mut counter = 0;
            for element in &mut self.space_used.iter_mut(){
                if counter == *space as usize {
                    *element = 1 as u32;
                }
                counter += 1;
            }
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for TetrisBoard {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for line in self.space_used.as_slice().chunks(self.width as usize) {
      for &cell in line {
        write!(f, "{}", cell)?;
      }
      write!(f, "\n");
    }
    Ok(())
  }
}
