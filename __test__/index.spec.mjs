import * as fs from 'fs'
import * as path from 'path'
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

import test from 'ava'

import { parse } from '../index.js'

test('test parse', (t) => {
  const jsx = fs.readFileSync(path.resolve(__dirname, 'fixure/mod.jsx'), 'utf8')
  const css1 = fs.readFileSync(path.resolve(__dirname, 'fixure/Mod.scss'), 'utf8')
  const css2 = fs.readFileSync(path.resolve(__dirname, 'fixure/test.scss'), 'utf8')
  
  const code = parse(jsx, [css1, css2])
  t.snapshot(code)
})
