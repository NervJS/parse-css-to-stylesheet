import * as fs from 'fs'
import * as path from 'path'
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

import test from 'ava'
import { parse } from '../index.js'

const normal = fs.readFileSync(path.resolve(__dirname, 'fixure/normal.jsx'), 'utf8') 

test('Harmony attrbute test unit', t => {
  const { code } = parse(normal, [`
  .px {
    top: 100px;
    left: 100ch;
    right: 100ch;
    bottom: 100ch;
  }
  .rem {
    width: 100rem;
  }
  .vh {
    height: 100vh;
  }
  .vw {
    width: 100vw;
  }
  .percent {
    width: 100%;
  }
  .decimal {
    width: 0.5px;
  }
  .decimal2 {
    width: .5px;
  }
  .calc {
    width: calc((100% - 280px) / 2);
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test flex', t => {
  const { code } = parse(normal, [`
  .flex1 {
    flex: 1;
  }
  .flex2 {
    flex: 1 0 auto;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test flex-grow', t => {
  const { code } = parse(normal, [`
  .item {
    flex-grow: 1;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test flex-shrink', t => {
  const { code } = parse(normal, [`
  .item {
    flex-shrink: 1;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test flex-basis', t => {
  const { code } = parse(normal, [`
  .item {
    flex-basis: 10rem;
  }
  .item2 {
    flex-basis: 3px;
  }
  .item3 {
    flex-basis: 50%;
  }
  .item4 {
    flex-basis: auto;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test flex-direction', t => {
  const { code } = parse(normal, [`
  .row {
    flex-direction: row
  }
  .row-reverse {
    flex-direction: row-reverse
  }
  .column {
    flex-direction: column
  }
  .column-reverse {
    flex-direction: column-reverse
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test justify-content', t => {
  const { code } = parse(normal, [`
  .flex-start {
    justify-content: flex-start
  }
  .flex-end {
    justify-content: flex-end
  }
  .center {
    justify-content: center
  }
  .space-between {
    justify-content: space-between
  }
  .space-around {
    justify-content: space-around
  }
  .space-evenly {
    justify-content: space-evenly
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test align-content', t => {
  const { code } = parse(normal, [`
  .flex-start {
    align-content: flex-start
  }
  .flex-end {
    align-content: flex-end
  }
  .center {
    align-content: center
  }
  .space-between {
    align-content: space-between
  }
  .space-around {
    align-content: space-around
  }
  .space-evenly {
    align-content: space-evenly
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})



test('Harmony attrbute test align-items', t => {
  const { code } = parse(normal, [`
  .flex-start {
    align-items: flex-start
  }
  .flex-end {
    align-items: flex-end
  }
  .center {
    align-items: center
  }
  .baseline {
    align-items: baseline
  }
  .stretch {
    align-items: stretch
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


test('Harmony attrbute test align-self', t => {
  const { code } = parse(normal, [`
  .flex-start {
    align-self: flex-start
  }
  .flex-end {
    align-self: flex-end
  }
  .center {
    align-self: center
  }
  .baseline {
    align-self: baseline
  }
  .stretch {
    align-self: stretch
  }
  .auto {
    align-self: auto
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


test('Harmony attrbute test flex-wrap', t => {
  const { code } = parse(normal, [`
  .nowrap {
    flex-wrap: nowrap
  }
  .wrap{
    flex-wrap: wrap
  }
  .wrap-reverse {
    flex-wrap: wrap-reverse
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


test('Harmony attrbute test position', t => {
  const { code } = parse(normal, [`
  .relative {
    position: relative
  }
  .absolute {
    position: absolute
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


test('Harmony attrbute test left top right bottom margin-left margin-right margin-bottom margin-top padding-bottom padding-left padding-right padding-top', t => {
  const { code } = parse(normal, [`
  .item {
    left: 10px;
    top: 10px;
    right: 10px;
    bottom: 10px;
    margin-top: 10px;
    margin-right: 10px;
    margin-bottom: 10px;
    margin-left: 10px;
    padding-top: 10px;
    padding-right: 10px;
    padding-bottom: 10px;
    padding-left: 10px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test width height min-width max-width min-height max-height', t => {
  const { code } = parse(normal, [`
  .item {
    width: 10px;
    height: 10px;
    min-width: 10px;
    max-width: 10px;
    min-height: 10px;
    max-height: 10px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


test('Harmony attrbute test background', t => {
  const { code } = parse(normal, [`
  .item {
    background: #f00;
  }
  .item2 {
    background: #f00 url('https://www.baidu.com');
  }
  .item2 {
    background: url('https://www.baidu.com');
  }
  .item3 {
    background: url('https://www.baidu.com') no-repeat;
  }
  .item4 {
    background: url('https://www.baidu.com') no-repeat center;
  }
  .item5 {
    background: url('https://www.baidu.com') no-repeat top right;
  }
  .item6 {
    background: url('https://www.baidu.com') no-repeat center center / 100px 200px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test background-image', t => {
  const { code } = parse(normal, [`
  .url {
    background-image: url('https://www.baidu.com');
  }
  .linear-gradient {
    background-image: linear-gradient(to right, #f00, #00f);
  }
  .radial-gradient {
    background-image: radial-gradient(30px at center, #fff, #000);
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test background-color', t => {
  const { code } = parse(normal, [`
  .item {
    background-color: #f00;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test background-size', t => {
  const { code } = parse(normal, [`
  .length {
    background-size: 100px;
  }
  .length_x_length_y {
    background-size: 100px 200px;
  }
  .contain {
    background-size: contain;
  }
  .cover {
    background-size: cover;
  }
  .auto {
    background-size: auto;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test background-repeat', t => {
  const { code } = parse(normal, [`
  .repeat {
    background-repeat: repeat;
  }
  .repeat-x {
    background-repeat: repeat-x;
  }
  .repeat-y {
    background-repeat: repeat-y;
  }
  .no-repeat {
    background-repeat: no-repeat;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test background-position', t => {
  const { code } = parse(normal, [`
  .center {
    background-position: center;
  }
  .top {
    background-position: top;
  }
  .bottom {
    background-position: bottom;
  }
  .left {
    background-position: left;
  }
  .right {
    background-position: right;
  }
  .length {
    background-position: 100px;
  }
  .length_x_length_y {
    background-position: 100px 200px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test border', t => {
  const { code } = parse(normal, [`
  .item {
    border: 1px solid #f00;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test border-top border-bottom border-left border-right', t => {
  const { code } = parse(normal, [`
  .item {
    border-top: 1px solid #f00;
  }
  .item2 {
    border-bottom: 1px solid #f00;
  }
  .item3 {
    border-left: 1px solid #f00;
  }
  .item4 {
    border-right: 1px solid #f00;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test border-top-width border-bottom-width border-left-width border-right-width', t => {
  const { code } = parse(normal, [`
  .item {
    border-top-width: 1px;
  }
  .item2 {
    border-bottom-width: 1px;
  }
  .item3 {
    border-left-width: 1px;
  }
  .item4 {
    border-right-width: 1px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test border-top-color border-bottom-color border-left-color border-right-color', t => {
  const { code } = parse(normal, [`
  .item {
    border-top-color: #f00;
  }
  .item2 {
    border-bottom-color: #f00;
  }
  .item3 {
    border-left-color: #f00;
  }
  .item4 {
    border-right-color: #f00;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test border-top-style border-bottom-style border-left-style border-right-style', t => {
  const { code } = parse(normal, [`
  .item {
    border-top-style: solid;
  }
  .item2 {
    border-bottom-style: dashed;
  }
  .item3 {
    border-left-style: dotted;
  }
  .item4 {
    border-right-style: solid;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test border-radius', t => {
  const { code } = parse(normal, [`
  .item {
    border-radius: 10px;
  }
  .item2 {
    border-radius: 10px 20px;
  }
  .item3 {
    border-radius: 10px 20px 30px;
  }
  .item4 {
    border-radius: 10px 20px 30px 40px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


test('Harmony attrbute test border-top-left-radius border-top-right-radius border-bottom-left-radius border-bottom-right-radius', t => {
  const { code } = parse(normal, [`
  .item {
    border-top-left-radius: 10px;
  }
  .item2 {
    border-top-right-radius: 10px;
  }
  .item3 {
    border-bottom-left-radius: 10px;
  }
  .item4 {
    border-bottom-right-radius: 10px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test transform', t => {
  const { code } = parse(normal, [`
  .item {
    transform: scale(1);
  }
  .item2 {
    transform: scale(1, 2);
  }
  .item3 {
    transform: rotate(45deg);
  }
  .item4 {
    transform: translate(10px, 20px);
  }
  .item5 {
    transform: scale(1) translate(10px, 20px) rotateX(45deg);
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test transform-origin', t => {
  const { code } = parse(normal, [`
  .item {
    transform-origin: 10px 20px;
  }
  .item2 {
    transform-origin: left bottom;
  }
  .item3 {
    transform-origin: center;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test font-size', t => {
  const { code } = parse(normal, [`
  .item {
    font-size: 10px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test font-weight', t => {
  const { code } = parse(normal, [`
  .bold {
    font-weight: bold;
  }
  .border {
    font-weight: bolder;
  }
  .lighter {
    font-weight: lighter;
  }
  .normal {
    font-weight: normal;
  }
  .number {
    font-weight: 100;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


test('Harmony attrbute test line-height', t => {
  const { code } = parse(normal, [`
  .item {
    line-height: 10px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


test('Harmony attrbute test text-align', t => {
  const { code } = parse(normal, [`
  .center {
    text-align: center;
  }
  .left {
    text-align: left;
  }
  .right {
    text-align: right;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test text-decoration', t => {
  const { code } = parse(normal, [`
  .none {
    text-decoration: none;
  }
  .underline {
    text-decoration: underline;
  }
  .overline {
    text-decoration: overline;
  }
  .line-through {
    text-decoration: line-through;
  }
  .color {
    text-decoration: underline #f00;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test vertical-align', t => {
  const { code } = parse(normal, [`
  .middle {
    vertical-align: middle
  }
  .top {
    vertical-align: top
  }
  .bottom {
    vertical-align: bottom
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test color', t => {
  const { code } = parse(normal, [`
  .hex {
    color: #f00;
  }
  .rgb {
    color: rgb(0, 10, 20);
  }
  .rgba {
    color: rgba(0, 10, 20, 0.5);
  }
  .orange {
    color: orange;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test -webkt-line-clamp', t => {
  const { code } = parse(normal, [`
  .line1 {
    -webkt-line-clamp: 1;
  }
  .line2 {
    -webkt-line-clamp: 2;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test animation', t => {
  const { code } = parse(normal, [`
  .anim {
    animation: move 2s infinite;
  }
  
  @keyframes move {
    0% {
      transform: translateX(0);
    }
    50% {
      transform: translateX(650px);
    }
    100% {
      transform: translateX(0);
    }
  }
  
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})
