const fs = require('fs')
const path = require('path')
process.env.platform = 'arm64'
const { parse } = require('../index.js')
const component = fs.readFileSync(path.resolve(__dirname, 'fixure/mod.jsx'), 'utf8')
const css1 = fs.readFileSync(path.resolve(__dirname, 'fixure/Mod.scss'), 'utf8')
const code = parse(component, [css1], "ReactNative")

console.log(code)