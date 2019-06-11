use wasm_bindgen::prelude::*;
use crate::tetris_board::TetrisBoard;
use crate::utils;

extern crate web_sys;


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


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
        let block2 = block1 - 1;
        let block3 = block1 + 1;
        let block4 = block1 + 2;
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

    pub fn get_block1(&self) -> i32 {
        self.block1 as i32
    }

    pub fn get_block2(&self) -> i32 {
        self.block2 as i32
    }

    pub fn get_block3(&self) -> i32 {
        self.block3 as i32
    }

    pub fn get_block4(&self) -> i32 {
        self.block4 as i32
    }

    pub fn move_shape_down(&mut self, board: &TetrisBoard) {
        self.block1 = self.block1 + board.get_width();
        self.block2 = self.block2 + board.get_width();
        self.block3 = self.block3 + board.get_width();
        self.block4 = self.block4 + board.get_width();
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
                self.block2 = self.block1 - board.get_width();
                self.block3 = self.block1 + board.get_width();
                self.block4 = self.block1 + (2 * board.get_width());
            },
            Position::Down => {
                self.position = Position::Right;
                self.block2 = self.block1 - 1;
                self.block3 = self.block1 + 1;
                self.block4 = self.block1 + 2;
            },
            _ => ()
        }
    }

    pub fn transform_step(&mut self, board: &TetrisBoard){
        match self.position {
            Position::Right => {
                self.position = Position::Down;
                self.block2 = self.block1 - board.get_width() + 1;
                self.block3 = self.block1 + 1;
                self.block4 = self.block1 + board.get_width();
            },
            Position::Down => {
                self.position = Position::Right;
                self.block2 = self.block1 - board.get_width() - 1;
                self.block3 = self.block1 - board.get_width();
                self.block4 = self.block1 + 1;
            },
            _ => ()
        }
    }

    pub fn transform_reverse_step(&mut self, board: &TetrisBoard){
        match self.position {
            Position::Right => {
                self.position = Position::Down;
                self.block2 = self.block1 - board.get_width();
                self.block3 = self.block1 + 1;
                self.block4 = self.block1 + board.get_width() + 1;
            },
            Position::Down => {
                self.position = Position::Right;
                self.block2 = self.block1 + 1;
                self.block3 = self.block1 + board.get_width() - 1;
                self.block4 = self.block1 + board.get_width();
            },
            _ => ()
        }
    }

    pub fn transform_hat(&mut self, board: &TetrisBoard){
        match self.position {
            Position::Right => {
                self.position = Position::Down;
                self.block2 = self.block1 - board.get_width();
                self.block3 = self.block1 - 1;
                self.block4 = self.block1 + 1;
            },
            Position::Down => {
                self.position = Position::Left;
                self.block2 = self.block1 - board.get_width();
                self.block3 = self.block1 - 1;
                self.block4 = self.block1 + board.get_width();
            },
            Position::Left => {
                self.position = Position::Up;
                self.block2 = self.block1 - 1;
                self.block3 = self.block1 + 1;
                self.block4 = self.block1 + board.get_width();
            },
            Position::Up => {
                self.position = Position::Right;
                self.block2 = self.block1 - board.get_width();
                self.block3 = self.block1 + 1;
                self.block4 = self.block1 + board.get_width();
            },
        }
    }

    pub fn transform_leg(&mut self, board: &TetrisBoard){
        match self.position {
            Position::Right => {
                self.position = Position::Down;
                self.block2 = self.block1 - board.get_width() + 1;
                self.block3 = self.block1 - 1;
                self.block4 = self.block1 + 1;
            },
            Position::Down => {
                self.position = Position::Left;
                self.block2 = self.block1 - board.get_width() - 1;
                self.block3 = self.block1 - board.get_width();
                self.block4 = self.block1 + board.get_width();
            },
            Position::Left => {
                self.position = Position::Up;
                self.block2 = self.block1 - 1;
                self.block3 = self.block1 + 1;
                self.block4 = self.block1 + board.get_width() - 1;
            },
            Position::Up => {
                self.position = Position::Right;
                self.block2 = self.block1 - board.get_width();
                self.block3 = self.block1 + board.get_width();
                self.block4 = self.block1 + board.get_width() + 1;
            },
        }
    }

    pub fn transform_reverse_leg(&mut self, board: &TetrisBoard){
        match self.position {
            Position::Right => {
                self.position = Position::Down;
                self.block2 = self.block1 - board.get_width() - 1;
                self.block3 = self.block1 - 1;
                self.block4 = self.block1 + 1;
            },
            Position::Down => {
                self.position = Position::Left;
                self.block2 = self.block1 - board.get_width();
                self.block3 = self.block1 + board.get_width() - 1;
                self.block4 = self.block1 + board.get_width();
            },
            Position::Left => {
                self.position = Position::Up;
                self.block2 = self.block1 - 1;
                self.block3 = self.block1 + 1;
                self.block4 = self.block1 + board.get_width() + 1;
            },
            Position::Up => {
                self.position = Position::Right;
                self.block2 = self.block1 - board.get_width();
                self.block3 = self.block1 - board.get_width() + 1;
                self.block4 = self.block1 + board.get_width();
            },
        }
    }
}
