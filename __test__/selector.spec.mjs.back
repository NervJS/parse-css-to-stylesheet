import * as fs from 'fs'
import * as path from 'path'
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

import test from 'ava'
import { parse } from '../index.js'

const normal = fs.readFileSync(path.resolve(__dirname, 'fixure/normal.jsx'), 'utf8') 

test('Harmony selector .item', t => {
  const { code } = parse(normal, [`
  .item {
    height: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony selector .item>.child', t => {
  const { code } = parse(normal, [`
  .item {
    height: 100px;
  }
  .item > .child {
    width: 100px;
  }
  .item > .child > child2 {
    width: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony selector .item .child', t => {
  const { code } = parse(normal, [`
  .item {
    height: 100px;
  }
  .item .child {
    width: 100px;
  }
  .item .child .child2 {
    width: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony selector .item .child > .grandchild', t => {
  const { code } = parse(normal, [`
  .item {
    height: 100px;
  }
  .item .child >.grandchild {
    width: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony selector .item, .another', t => {
  const { code } = parse(normal, [`
  .item, .another {
    height: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony selector static classname', t => {
  let jsx = `
  import { View } from '@tarojs/components';
  const Index = () => {
    return <View className='index' />
  };
  export default Index;
  `
  const { code } = parse(normal, [`
  .index {
    height: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony selector dynamic classname', t => {
  let jsx = `
  import { View } from '@tarojs/components';
  const Index = () => {
    return <View className={'index'} />
  };
  export default Index;
  `
  const { code } = parse(normal, [`
  .index {
    height: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony selector static style', t => {
  let jsx = `
  import { View } from '@tarojs/components';
  const Index = () => {
    return <View style={{ height: '100px' }} />
  };
  export default Index;
  `
  const { code } = parse(jsx, [`
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony selector dynamic style', t => {
  let jsx = `
  import { View } from '@tarojs/components';
  const Index = () => {
    return <View style={{ height: Math.random() > 0 ? '100px' : '50px' }} />
  };
  export default Index;
  `
  const { code } = parse(jsx, [`
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony selector mix static & dynamic style', t => {
  let jsx = `
  import { View } from '@tarojs/components';
  const Index = () => {
    return <View style={{ height: Math.random() > 0 ? '100px' : '50px', width: '100px' }} />
  };
  export default Index;
  `
  const { code } = parse(jsx, [`
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})



test('Harmony selector combine .item{} .item{}', t => {
  let jsx = `
  import { View } from '@tarojs/components';
  const Index = () => {
    return <View className='item' />
  };
  export default Index;
  `
  const { code } = parse(jsx, [`
  .item {
    height: 300px;
  }
  .item {
    height: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony selector combine .item{} .item{} important!', t => {
  let jsx = `
  import { View } from '@tarojs/components';
  const Index = () => {
    return <View className='item' />
  };
  export default Index;
  `
  const { code } = parse(jsx, [`
  .item {
    height: 300px !important;
  }
  .item {
    height: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony selector .item:before .item:after', t => {
  const { code } = parse(normal, [`
  .item {
    height: 100px;
  }
  .item::before {
    height: 100px;
    width: 30px;
  }
  .item::after {
    height: 100px;
    width: 30px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


test('Harmony selector .item:first-child .item:last-child .item.nth-child(2n-1)', t => {
  const { code } = parse(normal, [`
  .item {
    height: 100px;
  }
  .item:first-child {
    height: 100px;
    width: 30px;
  }
  .item:last-child {
    height: 100px;
    width: 30px;
  }
  .item:nth-child(2n-1) {
    height: 100px;
    width: 30px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony selector style', t => {
  const { code } = parse(`
  import { View, Text } from '@tarojs/components'
  import './pesudo.scss'
  export default function Pesudo() {
    return <>
      <View style={{width: '100px'}}></View>
      <View style={{width: \`${'100px'}\`}}></View>
      <View style={{width: \`${'100px'}\`, height: '100px'}}></View>
      <View className='a'></View>
      <View className={'a' + 1}></View>
      <View style={{width: '100px'}} className='a'></View>
      <View style={{width: \`${'100px'}\`}} className='a'></View>
      <View style={{width: \`${'100px'}\`, height: '100px'}} className='a'></View>
    </>
  }
  `, [``], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


