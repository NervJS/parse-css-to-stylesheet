import * as path from 'path'
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

import test from 'ava'
import { parse } from '../index.js'


test('Harmony attrbute test unit', t => {
  const { code } = parse([`
  .px {
    top: 100px;
    left: 100ch;
    right: 100ch;
    bottom: 100ch;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})