const fs = require('fs')
const path = require('path')
process.env.platform = 'arm64'
const { parse } = require('../index.js')
const css1 = fs.readFileSync(path.resolve(__dirname, 'fixure/pesudo.scss'), 'utf8')
const { code } = parse([css1], {
  platformString: 'Harmony',
  isEnableNesting: true
})

console.log(code)