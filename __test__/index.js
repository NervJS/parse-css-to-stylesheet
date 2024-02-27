const fs = require('fs')
const path = require('path')
process.env.platform = 'arm64'
const { parse } = require('../index.js')
const component = fs.readFileSync(path.resolve(__dirname, 'fixure/rn/index.jsx'), 'utf8')
const css1 = fs.readFileSync(path.resolve(__dirname, 'fixure/rn/index.scss'), 'utf8')
const code = parse(component, [css1])

console.log(code)