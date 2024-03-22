import * as fs from 'fs'
import * as path from 'path'
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

import test from 'ava'
import { parse } from '../index.js'

const normal = fs.readFileSync(path.resolve(__dirname, 'fixure/normal.jsx'), 'utf8') 

test('Harmony variables', t => {
  const { code, cssVariables } = parse(normal, [`
  :root {
    --height: 100px;
    --color: #f00;
    --width: var(--height);
    --margin-top: var(--height, 300px);
  }
  .item {
    height: var(--height)
  }
  .item2 {
    height: var(--height, 200px)
  }
  .item3 {
    height: var(--height, var(--width))
  }
  .item4 {
    height: var(--height, var(--width))
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(cssVariables + code)
})
