const fs = require('fs')
const path = require('path')
process.env.platform = 'arm64'
const { parseStyle } = require('../index.js')
const css1 = fs.readFileSync(path.resolve(__dirname, 'fixure/Mod.scss'), 'utf8')
const code = parseStyle([css1])

console.log(code)