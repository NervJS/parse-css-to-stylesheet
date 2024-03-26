import * as fs from 'fs'
import * as path from 'path'
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

import test from 'ava'
import { parse } from '../index.js'

const normal = fs.readFileSync(path.resolve(__dirname, 'fixure/normal.jsx'), 'utf8') 

test('Harmony compile mode', t => {
  const { code } = parse(`
  import { View } from '@tarojs/components';
  const Index = () => {
    return (
      <View className='item' compileMode />
    )
  };
  export default Index;
  `, [`
  .item {
    height: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony compile mode2', t => {
  const { code } = parse(`
  import { View } from '@tarojs/components';
  const Index = () => {
    return (
      <View className='item' compileMode />
    )
  };
  export default Index;
  `, [`
  .item {
    display: flex
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony compile mode3', t => {
  const { code } = parse(`
  import { View } from '@tarojs/components';
  const Index = () => {
    return (
      <View className='item' compileMode />
    )
  };
  export default Index;
  `, [`
  .item {
    display: flex;
    flex-direction: row;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony compile mode4', t => {
  const { code } = parse(`
  import { View } from '@tarojs/components';
  const Index = () => {
    return (
      <View className='item' compileMode />
    )
  };
  export default Index;
  `, [`
  .item {
    display: flex;
    flex-direction: column;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


test('Harmony compile mode5', t => {
  const { code } = parse(`
  import { View } from '@tarojs/components';
  const Index = () => {
    return (
      <View className='item' compileMode style={{ flexDirection: 'row' }} />
    )
  };
  export default Index;
  `, [`
  .item {
    display: flex;
    flex-direction: column;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})
