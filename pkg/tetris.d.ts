/* tslint:disable */
/**
*/
export enum Position {
  Up,
  Right,
  Down,
  Left,
}
/**
*/
export enum ShapeType {
  Bar,
  Square,
  Hat,
  Leg,
  ReverseLeg,
  Step,
  ReverseStep,
}
/**
*/
export class TetrisBoard {
  free(): void;
/**
* @returns {TetrisBoard} 
*/
  static new(): TetrisBoard;
/**
* @returns {number} 
*/
  get_width(): number;
/**
* @returns {number} 
*/
  get_score(): number;
/**
* @returns {boolean} 
*/
  get_game_over(): boolean;
/**
* @returns {void} 
*/
  set_game_over(): void;
/**
* @returns {boolean} 
*/
  get_generate_new_shape(): boolean;
/**
* @returns {void} 
*/
  set_generate_new_shape(): void;
/**
* @returns {number} 
*/
  get_height(): number;
/**
* @returns {number} 
*/
  get_space_used(): number;
/**
* @returns {number} 
*/
  get_space_length(): number;
/**
* @param {Tetromino} tetronimo_clone 
* @returns {void} 
*/
  check_shape_is_out_of_bounds(tetronimo_clone: Tetromino): void;
/**
* @param {Tetromino} tetronimo_clone 
* @returns {number} 
*/
  check_collision(tetronimo_clone: Tetromino): number;
/**
* @returns {void} 
*/
  check_for_completed_row(): void;
/**
* @param {Tetromino} tetronimo 
* @returns {void} 
*/
  add_shape_to_space_used(tetronimo: Tetromino): void;
/**
* @param {Tetromino} tetronimo 
* @returns {void} 
*/
  get_shape_position(tetronimo: Tetromino): void;
/**
* @param {Tetromino} tetronimo 
* @returns {void} 
*/
  update_shape_position(tetronimo: Tetromino): void;
/**
* @param {Tetromino} tetronimo 
* @returns {void} 
*/
  move_shape_down(tetronimo: Tetromino): void;
/**
* @param {Tetromino} tetronimo 
* @returns {void} 
*/
  move_shape_right(tetronimo: Tetromino): void;
/**
* @param {Tetromino} tetronimo 
* @returns {void} 
*/
  move_shape_left(tetronimo: Tetromino): void;
/**
* @returns {string} 
*/
  render(): string;
}
/**
*/
export class Tetromino {
  free(): void;
/**
* @param {number} rand 
* @returns {Tetromino} 
*/
  static generate_random_shape(rand: number): Tetromino;
/**
* @returns {Tetromino} 
*/
  static new_reverse_step_shape(): Tetromino;
/**
* @returns {Tetromino} 
*/
  static new_step_shape(): Tetromino;
/**
* @returns {Tetromino} 
*/
  static new_leg_shape(): Tetromino;
/**
* @returns {Tetromino} 
*/
  static new_reverse_leg_shape(): Tetromino;
/**
* @returns {Tetromino} 
*/
  static new_square_shape(): Tetromino;
/**
* @returns {Tetromino} 
*/
  static new_bar_shape(): Tetromino;
/**
* @returns {Tetromino} 
*/
  static new_hat_shape(): Tetromino;
/**
* @returns {number} 
*/
  get_block_size(): number;
/**
* @returns {number} 
*/
  get_block1(): number;
/**
* @returns {number} 
*/
  get_block2(): number;
/**
* @returns {number} 
*/
  get_block3(): number;
/**
* @returns {number} 
*/
  get_block4(): number;
/**
* @param {TetrisBoard} board 
* @returns {void} 
*/
  move_shape_down(board: TetrisBoard): void;
/**
* @returns {void} 
*/
  move_shape_right(): void;
/**
* @returns {void} 
*/
  move_shape_left(): void;
/**
* @param {TetrisBoard} board 
* @returns {void} 
*/
  transform_shape(board: TetrisBoard): void;
/**
* @param {TetrisBoard} board 
* @returns {void} 
*/
  transform_bar(board: TetrisBoard): void;
/**
* @param {TetrisBoard} board 
* @returns {void} 
*/
  transform_step(board: TetrisBoard): void;
/**
* @param {TetrisBoard} board 
* @returns {void} 
*/
  transform_reverse_step(board: TetrisBoard): void;
/**
* @param {TetrisBoard} board 
* @returns {void} 
*/
  transform_hat(board: TetrisBoard): void;
/**
* @param {TetrisBoard} board 
* @returns {void} 
*/
  transform_leg(board: TetrisBoard): void;
/**
* @param {TetrisBoard} board 
* @returns {void} 
*/
  transform_reverse_leg(board: TetrisBoard): void;
}
