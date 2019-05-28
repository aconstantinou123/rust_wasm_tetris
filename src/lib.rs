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

    pub fn move_shape_down(&mut self, board: &TetrisBoard) {
        self.block1 = self.block1 + board.width;
        self.block2 = self.block2 + board.width;
        self.block3 = self.block3 + board.width;
        self.block4 = self.block4 + board.width;
    }


    pub fn move_shape_right(&mut self) {
        self.block1 = self.block1 + 1;
        self.block2 = self.block2 + 1;
        self.block3 = self.block3 + 1;
        self.block4 = self.block4 + 1;
    }

    pub fn move_shape_left(&mut self) {
        self.block1 = self.block1 - 1;
        self.block2 = self.block2 - 1;
        self.block3 = self.block3 - 1;
        self.block4 = self.block4 - 1;
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

pub fn check_number(num: &u32) -> Vec<u32> {
    num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect()
}

pub fn add_walls() -> Vec<i32> {
        let space_used: Vec<i32> = vec![0; 264]
            .into_iter()
            .enumerate()
            .map(|(i, mut e)| {
                if i % 12 == 0 || i % 12 == 11 {
                    e = 3
                }
                else if i >= 252 {
                    e = 2
                }
                e
            }).collect();
        log!("{:?}", space_used);
        space_used
    }

#[wasm_bindgen]
#[derive(Debug)]
pub struct TetrisBoard {
    space_used: Vec<i32>,
    height: u32,
    width: u32,
}

#[wasm_bindgen]
impl TetrisBoard {
    pub fn new() -> TetrisBoard {
        utils::set_panic_hook();
        let space_used = add_walls();
        let height = 22;
        let width = 12;

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

    pub fn get_space_used(&self) -> *const i32 {
        self.space_used.as_ptr()
    }

    pub fn get_space_length(&self) -> usize {
        self.space_used.len()
    }

    pub fn update_shape_position(&mut self, tetronimo: &mut Tetromino) {
        tetronimo.transform_bar(&self);
        self.get_shape_position(tetronimo);
    }

    pub fn check_wall_collision(&mut self, tetronimo: &Tetromino, direction: i32) -> bool {
        let board_spaces: Vec <i32> = [tetronimo.block1 as i32, tetronimo.block2 as i32, tetronimo.block3 as i32, tetronimo.block4 as i32]
            .into_iter()
            .enumerate()
            .map(|(i, e)| e + direction as i32)
            .collect();
        let mut result = true;
        for space in board_spaces.iter() {
            let mut counter = 0;
            for element in self.space_used.iter_mut(){
                if counter == *space as u32 && *element == 3 {
                    result = false;
                    break;
                }
                counter += 1;
            }
        }
        result
    }

    pub fn get_shape_position(&mut self, tetronimo: &Tetromino){
        for element in self.space_used.iter_mut() {
            if *element == 1 {
                *element = 0
            }
        }
        let board_spaces = [tetronimo.block1, tetronimo.block2, tetronimo.block3, tetronimo.block4];
        for space in board_spaces.iter() {
            let mut counter = 0;
            for element in self.space_used.iter_mut(){
                if counter == *space as u32 {
                    *element = 1 as i32;
                }
                counter += 1;
            }
        }
    }

    pub fn move_shape_down(& mut self, tetronimo: &mut Tetromino){
        tetronimo.move_shape_down(&self);
        self.get_shape_position(tetronimo);
    }

    pub fn move_shape_right(& mut self, tetronimo: &mut Tetromino){
        let can_move = self.check_wall_collision(tetronimo, 1);
        if can_move == true {
            tetronimo.move_shape_right();
            self.get_shape_position(tetronimo);
        }
    }

    pub fn move_shape_left(& mut self, tetronimo: &mut Tetromino){
        let can_move = self.check_wall_collision(tetronimo, -1);
        if can_move == true {
            tetronimo.move_shape_left();
            self.get_shape_position(tetronimo);
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
        let mut symbol = "";
        println!("cell {}", cell);
        match cell {
            0 => symbol = "0",
            1 => symbol = "1",
            2 => symbol = "2",
            3 => symbol = "3",
            _ => (),
        }
        write!(f, "{}", symbol)?;
      }
      write!(f, "\n");
    }
    Ok(())
  }
}
