const express = require('express')
const path = require('path')

const app = express()

const port = 5000

app.use(express.static(path.join(__dirname, './dist')))

app.listen(port, () => {
  console.log(`Tetrust is ready to go on port ${port}`)
})
