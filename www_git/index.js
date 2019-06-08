import { TetrisBoard, Tetromino } from "tetris";
import { memory } from 'tetris/tetris_bg'

let rand
let tetronimo
let previewTetronimo
let interval
let tetrisBoard = TetrisBoard.new()
const blockSize = 37
const spaceLength = tetrisBoard.get_space_length()
const height = tetrisBoard.get_height() * 37
const width = tetrisBoard.get_width() * 37
const startButton = document.getElementById("start")
let startPressed = false
let paused = false
tetrisBoard.render()

const canvas = document.getElementById("tetris-canvas")
canvas.height = height
canvas.width = width
const ctx = canvas.getContext('2d')

const previewCanvas = document.getElementById("tetris-preview-canvas")
previewCanvas.height = 5 * 37
previewCanvas.width = 7 * 37
const previewCtx = previewCanvas.getContext('2d')

const getRandomInt = (max) => Math.floor(Math.random() * Math.floor(max))

const scoreHeader = document.getElementById("score")
scoreHeader.innerText = 'Score: 0'


const config = {
attributes: true,
childList: true,
}

const increaseSpeed = () => {
  if(tetrisBoard.get_score() >= 1050){
    return 150
  }
  if(tetrisBoard.get_score() >= 900){
    return 200
  }
  if(tetrisBoard.get_score() >= 750){
    return 250
  }
  if(tetrisBoard.get_score() >= 600){
    return 300
  }
  if(tetrisBoard.get_score() >= 450){
    return 350
  }
  if(tetrisBoard.get_score() >= 300){
    return 400
  }
  if(tetrisBoard.get_score() >= 150){
    return 450
  }
  return 500
}

let oldScore = 'Score: 0'
const observer = new MutationObserver((mutationsList)=>{
  for(let mutation of mutationsList) {
    if(oldScore !== mutation.target.innerText){
      clearInterval(interval)
      startInterval(increaseSpeed())
    }
    oldScore = mutation.target.innerText
  }
})

observer.observe(scoreHeader, config)

const renderScore = () => {
  const scoreHeader = document.getElementById("score")
  const score = tetrisBoard.get_score()
  scoreHeader.innerText = `Score: ${score}`
}

const clearRect = () => {
  ctx.clearRect(0, 0, canvas.width, canvas.height)
}

const clearPreviewRect = () => {
  previewCtx.clearRect(0, 0, canvas.width, canvas.height)
}

const update = () => {
  clearRect()
  drawShapes()
  renderScore()
}

document.addEventListener('keydown', function(event) {
  const key = event.key
  if(!tetrisBoard.get_generate_new_shape() && !tetrisBoard.get_game_over()){
    if(key === 'd'){
      tetrisBoard.move_shape_right(tetronimo)
      update()
    } 
    else if(key === 'a'){
      tetrisBoard.move_shape_left(tetronimo)
      update()
    } 
    else if(key == 's'){
      tetrisBoard.move_shape_down(tetronimo)
      update()
    }
  }
})

const drawShapes = () => {
  const spacePtr = tetrisBoard.get_space_used()
  const spaceUsed = new Uint32Array(memory.buffer, spacePtr, spaceLength);
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
  if(!tetrisBoard.get_generate_new_shape() && !tetrisBoard.get_game_over()){
    if(key === 'w'){
      tetrisBoard.update_shape_position(tetronimo)
      update()
    }
  }
})

const drawPreview = (previewTetronimo) => {
  clearPreviewRect()
  const positionArray = [
    previewTetronimo.get_block1() + 22,
    previewTetronimo.get_block2() + 22,
    previewTetronimo.get_block3() + 22,
    previewTetronimo.get_block4() + 22,
  ]
  positionArray.forEach((index) => {
    const y =  Math.floor(index / tetrisBoard.get_width()) *  blockSize
    const x = index % tetrisBoard.get_width() * blockSize 
    previewCtx.fillRect(x, y, blockSize - 2, blockSize - 2)
  })
}

const startInterval = (speed) => {
  interval = setInterval(() => {
    if(!tetrisBoard.get_generate_new_shape() && !tetrisBoard.get_game_over()){
      tetrisBoard.move_shape_down(tetronimo)
      update()
    } else if(!tetrisBoard.get_game_over()) {
      if(rand === undefined){
        rand = getRandomInt(7)
      }
      tetronimo = Tetromino.generate_random_shape(rand)
      tetrisBoard.get_shape_position(tetronimo)
      tetrisBoard.set_generate_new_shape()
      rand = getRandomInt(7)
      previewTetronimo = Tetromino.generate_random_shape(rand)
      drawPreview(previewTetronimo)
    } else if(tetrisBoard.get_game_over()) {
      const scoreHeader = document.getElementById("score")
      const score = tetrisBoard.get_score()
      scoreHeader.innerText = `Game over. Final Score: ${score}`
      clearInterval(interval)
    }
  }, speed);
}

startButton.addEventListener('click', () => {
  if(!startPressed){
    startPressed = true
    startInterval(increaseSpeed())
  } else if(tetrisBoard.get_game_over()) {
    tetrisBoard = TetrisBoard.new()
    tetrisBoard.get_game_over()
    startPressed = true
    startInterval(increaseSpeed())
  }
})

drawShapes()