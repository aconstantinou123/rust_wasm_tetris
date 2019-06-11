use wasm_bindgen::prelude::*;
use std::fmt;
use std::collections::HashMap;
use crate::tetronimo::Tetromino;
use crate::utils;

extern crate web_sys;


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
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
        space_used
    }

#[wasm_bindgen]
#[derive(Debug)]
pub struct TetrisBoard {    
    generate_new_shape: bool,
    space_used: Vec<i32>,
    height: u32,
    width: u32,
    game_over: bool,
    score: u32,
}

#[wasm_bindgen]
impl TetrisBoard {
    pub fn new() -> TetrisBoard {
        utils::set_panic_hook();
        let generate_new_shape = true;
        let space_used = add_walls();
        let height = 22;
        let width = 12;
        let game_over = false;
        let score = 0;

        TetrisBoard {
            generate_new_shape,
            space_used,
            height,
            width,
            game_over,
            score,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_game_over(&self) -> bool {
        self.game_over
    }

    pub fn set_game_over(&mut self) {
        self.game_over = !self.get_game_over()
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

    pub fn get_space_used(&self) -> *const i32 {
        self.space_used.as_ptr()
    }

    pub fn get_space_length(&self) -> usize {
        self.space_used.len()
    }

    pub fn check_shape_is_out_of_bounds(&mut self, tetronimo_clone: &Tetromino) {
        let board_spaces = [
            tetronimo_clone.get_block1() as i32, 
            tetronimo_clone.get_block2() as i32, 
            tetronimo_clone.get_block3() as i32, 
            tetronimo_clone.get_block4() as i32
        ];
        let out_of_bounds = board_spaces.iter().find(|&&x| x < 0);
        match out_of_bounds {
            Some(i) => self.game_over = true,
            None => self.add_shape_to_space_used(&tetronimo_clone),
        }
    }

    pub fn check_collision(&mut self, tetronimo_clone: &Tetromino) -> u32 {
        let board_spaces = [
            tetronimo_clone.get_block1(), 
            tetronimo_clone.get_block2(), 
            tetronimo_clone.get_block3(), 
            tetronimo_clone.get_block4()
        ];
        let mut result = 0;
        for (i, e) in self.space_used.iter().enumerate(){
            let found_space = board_spaces.iter().find(|&&x| x == i as i32);
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
        let mut number_of_lines = 0;
        for i in 0..self.height as usize {
            let position = i * self.width as usize;
            let slice = &self.space_used[position..position + self.width as usize];
            let mut map = HashMap::new();
            for num in slice.iter() {
                let count = map.entry(num).or_insert(0);
                *count += 1;
            }
            let indexes: Vec<usize> = (position..position + self.width as usize).collect();
            let twos = map.get(&2);
            match twos {
                Some(10) => {
                    number_of_lines += 1;
                    self.space_used.drain(indexes[0]..indexes[indexes.len() - 1] + 1);
                    let mut vec :Vec<i32> = vec![0; self.width as usize];
                    vec[0] = 3;
                    vec[self.width as usize - 1] = 3;
                    self.space_used.splice(0..0, vec);
                },
                None => (),
                _ => (),
            }
        }
        self.score += number_of_lines * number_of_lines * 10;
    }


    pub fn add_shape_to_space_used(&mut self, tetronimo: &Tetromino) {
        let board_spaces = [tetronimo.get_block1(), tetronimo.get_block2(), tetronimo.get_block3(), tetronimo.get_block4()];
            for e in board_spaces.iter() {
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
        let board_spaces = [tetronimo.get_block1() as i32, tetronimo.get_block2() as i32, tetronimo.get_block3() as i32, tetronimo.get_block4() as i32];
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
            self.check_shape_is_out_of_bounds(&tetronimo);
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
