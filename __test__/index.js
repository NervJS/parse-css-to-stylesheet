const fs = require('fs')
const path = require('path')
process.env.platform = 'arm64'
const { parse } = require('../index.js')
const component = fs.readFileSync(path.resolve(__dirname, 'fixure/pesudo.jsx'), 'utf8')
const css1 = fs.readFileSync(path.resolve(__dirname, 'fixure/pesudo.scss'), 'utf8')
const code = parse(component, [css1], "Harmony")

console.log(code)