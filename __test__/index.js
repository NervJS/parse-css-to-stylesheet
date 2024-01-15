const fs = require('fs')
const path = require('path')
process.env.platform = 'arm64'
const { parse, parseStyle } = require('../index.js')

const jsx = fs.readFileSync(path.resolve(__dirname, 'fixure/rn/index.jsx'), 'utf8')
const css1 = fs.readFileSync(path.resolve(__dirname, 'fixure/rn/index.scss'), 'utf8')

const code = parseStyle([css1])

console.log(code)