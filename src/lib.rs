mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use std::collections::HashMap;

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
#[derive(Debug, Clone)]
pub enum Position {
    Up,
    Right,
    Down,
    Left
}


#[wasm_bindgen]
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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


    pub fn generate_random_shape(rand: u32) -> Tetromino {
        match rand {
            0 => Tetromino::new_bar_shape(),
            1 => Tetromino::new_hat_shape(),
            2 => Tetromino::new_square_shape(),
            3 => Tetromino::new_leg_shape(),
            4 => Tetromino::new_reverse_leg_shape(),
            5 => Tetromino::new_step_shape(),
            6 => Tetromino::new_reverse_step_shape(),
            _ => Tetromino::new_bar_shape()
        }
    }

    pub fn new_reverse_step_shape() -> Tetromino {
        utils::set_panic_hook();
        let block_size = 50;
        let block1 = 5;
        let block2 = block1 + 1;
        let block3 = block1 + 11;
        let block4 = block1 + 12;
        let position = Position::Right;
        let shape_type = ShapeType::ReverseStep;
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

    pub fn new_step_shape() -> Tetromino {
        utils::set_panic_hook();
        let block_size = 50;
        let block1 = 5;
        let block2 = block1 - 13;
        let block3 = block1 - 12;
        let block4 = block1 + 1;
        let position = Position::Right;
        let shape_type = ShapeType::Step;

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

    pub fn new_leg_shape() -> Tetromino {
        utils::set_panic_hook();
        let block_size = 50;
        let block1 = 5;
        let block2 = block1 - 12;
        let block3 = block1 + 12;
        let block4 = block1 + 13;
        let position = Position::Right;
        let shape_type = ShapeType::Leg;
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

    pub fn new_reverse_leg_shape() -> Tetromino {
        utils::set_panic_hook();
        let block_size = 50;
        let block1 = 5;
        let block2 = block1 - 12;
        let block3 = block1 - 11;
        let block4 = block1 + 12;
        let position = Position::Right;
        let shape_type = ShapeType::ReverseLeg;
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

    pub fn new_square_shape() -> Tetromino {
        utils::set_panic_hook();
        let block_size = 50;
        let block1 = 5;
        let block2 = block1 + 1;
        let block3 = block1 + 12;
        let block4 = block1 + 13;
        let position = Position::Right;
        let shape_type = ShapeType::Square;

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

    pub fn new_hat_shape() -> Tetromino {
        utils::set_panic_hook();
        let block_size = 50;
        let block1 = 5;
        let block2 = block1 - 12;
        let block3 = block1 + 1;
        let block4 = block1 + 12;
        let position = Position::Right;
        let shape_type = ShapeType::Hat;

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

    pub fn transform_shape(&mut self, board: &TetrisBoard){
        match self.shape_type {
            ShapeType::Bar => self.transform_bar(board),
            ShapeType::Hat => self.transform_hat(board),
            ShapeType::Leg => self.transform_leg(board),
            ShapeType::ReverseLeg => self.transform_reverse_leg(board),
            ShapeType::Step => self.transform_step(board),
            ShapeType::ReverseStep => self.transform_reverse_step(board),
            _ => ()
        }
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

    pub fn transform_step(&mut self, board: &TetrisBoard){
        match self.position {
            Position::Right => {
                self.position = Position::Down;
                self.block2 = self.block1 - board.width + 1;
                self.block3 = self.block1 + 1;
                self.block4 = self.block1 + board.width;
            },
            Position::Down => {
                self.position = Position::Right;
                self.block2 = self.block1 - board.width - 1;
                self.block3 = self.block1 - board.width;
                self.block4 = self.block1 + 1;
            },
            _ => ()
        }
    }

    pub fn transform_reverse_step(&mut self, board: &TetrisBoard){
        match self.position {
            Position::Right => {
                self.position = Position::Down;
                self.block2 = self.block1 - board.width;
                self.block3 = self.block1 + 1;
                self.block4 = self.block1 + board.width + 1;
            },
            Position::Down => {
                self.position = Position::Right;
                self.block2 = self.block1 + 1;
                self.block3 = self.block1 + board.width - 1;
                self.block4 = self.block1 + board.width;
            },
            _ => ()
        }
    }

    pub fn transform_hat(&mut self, board: &TetrisBoard){
        match self.position {
            Position::Right => {
                self.position = Position::Down;
                self.block2 = self.block1 - board.width;
                self.block3 = self.block1 - 1;
                self.block4 = self.block1 + 1;
            },
            Position::Down => {
                self.position = Position::Left;
                self.block2 = self.block1 - board.width;
                self.block3 = self.block1 - 1;
                self.block4 = self.block1 + board.width;
            },
            Position::Left => {
                self.position = Position::Up;
                self.block2 = self.block1 - 1;
                self.block3 = self.block1 + 1;
                self.block4 = self.block1 + board.width;
            },
            Position::Up => {
                self.position = Position::Right;
                self.block2 = self.block1 - board.width;
                self.block3 = self.block1 + 1;
                self.block4 = self.block1 + board.width;
            },
            _ => ()
        }
    }

    pub fn transform_leg(&mut self, board: &TetrisBoard){
        match self.position {
            Position::Right => {
                self.position = Position::Down;
                self.block2 = self.block1 - board.width + 1;
                self.block3 = self.block1 - 1;
                self.block4 = self.block1 + 1;
            },
            Position::Down => {
                self.position = Position::Left;
                self.block2 = self.block1 - board.width - 1;
                self.block3 = self.block1 - board.width;
                self.block4 = self.block1 + board.width;
            },
            Position::Left => {
                self.position = Position::Up;
                self.block2 = self.block1 - 1;
                self.block3 = self.block1 + 1;
                self.block4 = self.block1 + board.width - 1;
            },
            Position::Up => {
                self.position = Position::Right;
                self.block2 = self.block1 - board.width;
                self.block3 = self.block1 + board.width;
                self.block4 = self.block1 + board.width + 1;
            },
            _ => ()
        }
    }

    pub fn transform_reverse_leg(&mut self, board: &TetrisBoard){
        match self.position {
            Position::Right => {
                self.position = Position::Down;
                self.block2 = self.block1 - board.width - 1;
                self.block3 = self.block1 - 1;
                self.block4 = self.block1 + 1;
            },
            Position::Down => {
                self.position = Position::Left;
                self.block2 = self.block1 - board.width;
                self.block3 = self.block1 + board.width - 1;
                self.block4 = self.block1 + board.width;
            },
            Position::Left => {
                self.position = Position::Up;
                self.block2 = self.block1 - 1;
                self.block3 = self.block1 + 1;
                self.block4 = self.block1 + board.width + 1;
            },
            Position::Up => {
                self.position = Position::Right;
                self.block2 = self.block1 - board.width;
                self.block3 = self.block1 - board.width + 1;
                self.block4 = self.block1 + board.width;
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
                    e = 4
                }
                e
            }).collect();
        log!("{:?}", space_used);
        space_used
    }

#[wasm_bindgen]
#[derive(Debug)]
pub struct TetrisBoard {    
    generate_new_shape: bool,
    space_used: Vec<i32>,
    height: u32,
    width: u32,
}

#[wasm_bindgen]
impl TetrisBoard {
    pub fn new() -> TetrisBoard {
        utils::set_panic_hook();
        let generate_new_shape = true;
        let space_used = add_walls();
        let height = 22;
        let width = 12;

        TetrisBoard {
            generate_new_shape,
            space_used,
            height,
            width,
        }
    }

    pub fn get_generate_new_shape(&self) -> bool {
        self.generate_new_shape
    }

    pub fn set_generate_new_shape(&mut self) {
        self.generate_new_shape = !self.generate_new_shape
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

    pub fn check_collision(&mut self, tetronimo_clone: &Tetromino) -> u32 {
        let board_spaces = [
            tetronimo_clone.block1, 
            tetronimo_clone.block2, 
            tetronimo_clone.block3, 
            tetronimo_clone.block4
        ];
        let mut result = 0;
        for (i, e) in self.space_used.iter().enumerate(){
            let found_space = board_spaces.iter().find(|&&x| x == i as u32);
            if *e == 3 {
                match found_space {
                    None => (),
                    Some(i) => ({ 
                        result = 3
                    }),
                }
            } else if *e == 2 || *e == 4 {
                match found_space {
                    None => (),
                    Some(i) => ({
                        result = 2
                    }),
                }
            }
        }
        result
    }

    pub fn check_for_completed_row(&mut self) {
        for mut i in 0..22 {
            let position = i * 12;
            let slice = &self.space_used[position..position+12];
            let mut map = HashMap::new();
            for num in slice.iter() {
                let count = map.entry(num).or_insert(0);
                *count += 1;
            }
            let mut indexes: Vec<usize> = (position..position+12).collect();
            let twos = map.get(&2);
            match(twos){
                Some(10) => {
                    log!("here")
                },
                None => (),
                _ => (),
            }
        }
    }


    pub fn add_shape_to_space_used(&mut self, tetronimo: &Tetromino) {
        let board_spaces = [tetronimo.block1, tetronimo.block2, tetronimo.block3, tetronimo.block4];
            for mut e in board_spaces.iter() {
                self.space_used[*e as usize] = 2;
            }
        self.set_generate_new_shape()
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

    pub fn update_shape_position(&mut self, tetronimo: &mut Tetromino) {
        let mut clone = tetronimo.clone();
        clone.transform_shape(&self);
        let can_move =self.check_collision(&clone);
        if can_move == 0 {
            tetronimo.transform_shape(&self);
            self.get_shape_position(tetronimo);
        }
    }

    pub fn move_shape_down(& mut self, tetronimo: &mut Tetromino){
        let mut clone = tetronimo.clone();
        clone.move_shape_down(&self);
        let can_move =self.check_collision(&clone);
        if can_move == 0 {
            tetronimo.move_shape_down(&self);
            self.get_shape_position(tetronimo);
        } else if can_move == 2 {
            self.add_shape_to_space_used(tetronimo);
            self.check_for_completed_row();
        }
    }

    pub fn move_shape_right(& mut self, tetronimo: &mut Tetromino){
        let mut clone = tetronimo.clone();
        clone.move_shape_right();
        let can_move =self.check_collision(&clone);
        if can_move == 0 {
            tetronimo.move_shape_right();
            self.get_shape_position(tetronimo);
        } 
    }

    pub fn move_shape_left(& mut self, tetronimo: &mut Tetromino){
        let mut clone = tetronimo.clone();
        clone.move_shape_left();
        let can_move =self.check_collision(&clone);
        if can_move == 0 {
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
            4 => symbol = "4",
            _ => (),
        }
        write!(f, "{}", symbol)?;
      }
      write!(f, "\n");
    }
    Ok(())
  }
}
