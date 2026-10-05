import test from 'ava'
import { parse } from '../index.js'

test('Harmony filter preserves compiled design units, like layout lengths', t => {
  const { code } = parse(['.probe { width: 20px; filter: blur(4px) invert(1); backdrop-filter: blur(2.5px); }'],
    { platformString: 'Harmony' })
  t.true(code.includes('blur(4lpx)'))
  t.true(code.includes('blur(2.5lpx)'))
  t.false(code.includes('blur(4px)'))
})

for (const platformString of ['Harmony', 'ReactNative']) {
  for (const radius of [0, 0.5, 1, 4, 12.5]) {
    test(`${platformString} postcss-normalized blur(${radius * 2}px) retains lpx`, t => {
      const { code } = parse([`.probe { filter: blur(${radius * 2}px) invert(50%); backdrop-filter: blur(${radius * 2}px); }`],
        { platformString, designWidth: 375 })
      const declarations = JSON.parse(code).styles[0].declarations
      for (const [property, value] of declarations) {
        if (property === 111 || property === 117) {
          // LightningCSS serializes zero blur without an explicit argument.
          t.true(radius === 0 ? /^blur\((?:0lpx)?\)/.test(value) : value.includes(`blur(${radius * 2}lpx)`))
        }
      }
      t.is(declarations.length, 2)
    })
  }
}

test('ReactNative filter retains its existing design-unit contract', t => {
  const { code } = parse(['.probe { filter: blur(4px); }'], { platformString: 'ReactNative' })
  t.true(code.includes('blur(4lpx)'))
})

test('Harmony background position lengths use numeric design pixels', t => {
  const { code } = parse(['.probe { background-position: 12px 8px; }'], { platformString: 'Harmony' })
  const declarations = JSON.parse(code).styles[0].declarations
  t.deepEqual(declarations, [[83, 12], [84, 8]])
})
