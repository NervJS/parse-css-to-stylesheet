// Generate real default-pxtransform -> compiler -> FlatBuffer regression input.
// Usage: node scripts/generate-filter-unit-fixtures.cjs <output-directory> <pxtransform-entry>
const fs = require('node:fs')
const path = require('node:path')
const { createRequire } = require('node:module')
const assert = require('node:assert/strict')
const { parse } = require('../index.js')
const [outputDirectory, pluginEntry] = process.argv.slice(2)
assert(outputDirectory && pluginEntry, 'output directory and actual pxtransform module are required')
const pluginRequire = createRequire(path.resolve(pluginEntry))
const postcss = pluginRequire('postcss')
const pxtransform = pluginRequire(path.resolve(pluginEntry))
const rules = [
  ['blur', 'filter', 'blur(4px)'],
  ['fraction', 'filter', 'blur(.5px)'],
  ['zero', 'filter', 'blur(0px)'],
  ['large', 'filter', 'blur(12.5px)'],
  ['calc', 'filter', 'blur(calc(6px - 2px))'],
  ['combo', 'filter', 'blur(4px) invert(50%)'],
  ['backdrop', 'backdrop-filter', 'blur(4px)'],
  ['backdrop-calc', 'backdrop-filter', 'blur(calc(6px - 2px))'],
  ['none', 'filter', 'none'],
  ['backdrop-none', 'backdrop-filter', 'none'],
]
const input = rules.map(([name, property, value]) => `.${name} { ${property}: ${value}; }`).join('\n')
// Intentionally no propList, selector blacklist, or disable comment.
const processed = postcss([pxtransform({ platform: 'harmony', designWidth: 375 })])
  .process(input, { from: 'filter-units.css' }).css
const { code } = parse([processed], { platformString: 'Harmony', designWidth: 375 })
const compiled = JSON.parse(code)
assert.equal(compiled.styles.length, rules.length)
assert(compiled.styles.some(rule => rule.declarations.some(([property, value]) =>
  property === 111 && value === 'blur(8lpx)')))
for (const version of ['v1', 'v2']) {
  const { buffer } = parse([processed], {
    platformString: 'Harmony', designWidth: 375, output: { isBin: true, version },
  })
  assert(Buffer.isBuffer(buffer) && buffer.length > 0)
  fs.writeFileSync(path.join(outputDirectory, `filter-design-${version}.bin`), buffer)
}
console.log(JSON.stringify({ input, postcss: processed, compiled }, null, 2))
