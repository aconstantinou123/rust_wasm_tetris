import { TetrisBoard, Tetromino } from "tetris";
import { memory } from 'tetris/tetris_bg'

let tetronimo
const blockSize = 50
const tetrisBoard = TetrisBoard.new()
const spaceLength = tetrisBoard.get_space_length()
const height = tetrisBoard.get_height() * 50
const width = tetrisBoard.get_width() * 50
// tetrisBoard.get_shape_position(tetronimo)
tetrisBoard.render()

const canvas = document.getElementById("tetris-canvas")
canvas.height = height
canvas.width = width
const ctx = canvas.getContext('2d')

const getRandomInt = (max) => Math.floor(Math.random() * Math.floor(max))


const clearRect = () => {
  ctx.clearRect(0, 0, canvas.width, canvas.height)
}

document.addEventListener('keydown', function(event) {
  const key = event.key
  if(!tetrisBoard.get_generate_new_shape()){
    if(key === 'd'){
      tetrisBoard.move_shape_right(tetronimo)
      clearRect()
      drawShapes()
    } 
    else if(key === 'a'){
      tetrisBoard.move_shape_left(tetronimo)
      clearRect()
      drawShapes()
    } 
    else if(key == 's'){
      tetrisBoard.move_shape_down(tetronimo)
      clearRect()
      drawShapes()
    }
  }
})

const drawShapes = () => {
  const spacePtr = tetrisBoard.get_space_used()
  const spaceUsed = new Uint32Array(memory.buffer, spacePtr, spaceLength);
  // console.log(spaceUsed)
  spaceUsed.forEach((space, index) => {
    if (space == 3 || index >= 253 || index <= 12){
    const y =  Math.floor(index / tetrisBoard.get_width()) *  blockSize
    const x = index % tetrisBoard.get_width() * blockSize 
    ctx.fillRect(x, y, blockSize, blockSize)
    } else if(space === 1 || space === 2){
      const y =  Math.floor(index / tetrisBoard.get_width()) *  blockSize
      const x = index % tetrisBoard.get_width() * blockSize 
      ctx.fillRect(x, y, blockSize - 2, blockSize - 2)
    }
  })
}

document.addEventListener('keydown', (event) => {
  const key = event.key
  if(!tetrisBoard.get_generate_new_shape()){
    if(key === 'Enter'){
      tetrisBoard.update_shape_position(tetronimo)
      clearRect()
      drawShapes()
    }
  }
})

setInterval(() => {
  if(!tetrisBoard.get_generate_new_shape()){
    tetrisBoard.move_shape_down(tetronimo)
    clearRect()
    drawShapes()
  } else {
    const rand = getRandomInt(7)
    tetronimo = Tetromino.generate_random_shape(6)
    tetrisBoard.get_shape_position(tetronimo)
    tetrisBoard.set_generate_new_shape()
  }
}, 500)

// ctx.fillRect(100, 400, blockSize, blockSize)

drawShapes()