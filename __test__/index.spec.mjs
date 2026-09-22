import * as fs from 'fs'
import * as path from 'path'
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

import test from 'ava'
import { parse } from '../index.js'

const normal = fs.readFileSync(path.resolve(__dirname, 'fixure/normal.jsx'), 'utf8') 

test('Harmony attrbute test unit', t => {
  const { code } = parse([`
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
    height: calc((100vw - 280px) / 2);
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test flex', t => {
  const { code } = parse([`
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
  const { code } = parse([`
  .item {
    flex-grow: 1;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test flex-shrink', t => {
  const { code } = parse([`
  .item {
    flex-shrink: 1;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test flex-basis', t => {
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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

test('Harmony attrbute test margin auto', t => {
  const { code } = parse([`
  .item {
    margin-left: auto;
    margin-right: auto;
    margin-top: auto;
    margin-bottom: auto;
  }
  .item2 {
    margin: 0 auto;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test width height min-width max-width min-height max-height', t => {
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
  .item {
    background-color: #f00;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test background-size', t => {
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
  .item {
    border: 1px solid #f00;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test border-top border-bottom border-left border-right', t => {
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
  .item {
    font-size: 10px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test font-weight', t => {
  const { code } = parse([`
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
  const { code } = parse([`
  .item {
    line-height: 10px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


test('Harmony attrbute test text-align', t => {
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  const { code } = parse([`
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
  // 注意：属性名是 `-webkt-line-clamp`（缺 `i`），这是历史遗留 typo。
  // 真正的 -webkit-line-clamp 走 vendor-strip 后仍能产出 [81,...]，见下方同名
  // 但拼写正确的测试。本用例固定不识别的 vendor 属性应当产出空 declarations，
  // 防止误以为 typo 路径也走 WebkitLineClamp=81。
  const { code } = parse([`
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

test('Harmony attrbute test -webkit-line-clamp', t => {
  // 与上面 typo 测试互补：-webkit-line-clamp 应当走 vendor-strip +
  // 『lineClamp』 特殊臂 → Normal::new(WebkitLineClamp, ...)
  const { code } = parse([`
  .line1 {
    -webkit-line-clamp: 1;
  }
  .line2 {
    -webkit-line-clamp: 2;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test -webkit-appearance -moz-appearance', t => {
  // appearance 无注册表条目，应走 dispatch 未命中的兜底路径。此用例主要
  // 证明 vendor-strip 分支（src/parse_style_properties.rs:182-186）对
  // 非注册表即 vendor-only 属性也能正常走完 parse 流程。
  const { code } = parse([`
  .a {
    -webkit-appearance: none;
    -moz-appearance: none;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test animation', t => {
  const { code } = parse([`
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

test('Harmony combine test function component', t => {
  const { code } = parse([`
  .a > .b {
    height: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony combine test arrow component', t => {
  const { code } = parse([`
  .a > .b {
    height: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


test('Harmony combine test hoc', t => {
  const { code } = parse([`
  .a > .b {
    height: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})


test('Harmony combine test useHoc', t => {
  const { code } = parse([`
  .a > .b {
    height: 100px;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test grid', t => {
  const { code } = parse([`
  .grid {
    display: grid;
    grid-template-columns: 1fr 2fr;
    grid-template-rows: auto 1fr;
    gap: 10px;
    justify-items: center;
    align-items: stretch;
    justify-content: space-between;
    align-content: center;
  }
  .inline-grid {
    display: inline-grid;
  }
  .grid-repeat {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
  }
  .grid-repeat-multi {
    grid-template-columns: repeat(2, 100px 1fr);
  }
  .grid-mixed {
    grid-template-columns: 100px 50% 1fr 2fr;
  }
  .grid-none {
    grid-template-columns: none;
  }
  .grid-keywords {
    grid-template-columns: min-content max-content auto;
  }
  .grid-passthrough {
    grid-template-columns: minmax(100px, 1fr);
    grid-template-rows: repeat(auto-fill, 100px);
  }
  .item {
    grid-column: 1 / span 2;
    grid-row: 1 / 3;
    justify-self: end;
    align-self: center;
  }
  .item-start-end {
    grid-column-start: 2;
    grid-column-end: span 3;
    grid-row-start: 1;
    grid-row-end: -1;
  }
  .item-span {
    grid-column: span 2;
    grid-area: 1 / 2 / 3 / 4;
  }
  .item-named {
    grid-column: header / footer;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony attrbute test grid template shorthand', t => {
  const { code } = parse([`
  .grid-template {
    display: grid;
    grid-template: 1fr auto / 1fr 2fr;
  }
  `], {
    platformString: 'Harmony'
  })
  t.snapshot(code)
})

test('Harmony grid flatbuffer v2 round trip', t => {
  const { buffer } = parse([`
  .grid {
    display: grid;
    grid-template-columns: 1fr 2fr;
    grid-template-rows: auto 1fr;
  }
  .item {
    grid-column: 1 / span 2;
    grid-area: 1 / 2 / 3 / 4;
  }
  `], {
    platformString: 'Harmony',
    output: { isBin: true, version: 'v2' }
  })
  t.true(buffer instanceof Buffer)
  t.true(buffer.length > 0)
})
