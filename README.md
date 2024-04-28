# parse-css-to-stylesheet

## 简介

解析 React 组件与对应的 CSS 文件，为每一个 React 节点计算样式最终样式，应用于 React Native 、鸿蒙等不支持 CSS 写法的场景

## 使用方式

```typescript
import { parse } from '@tarojs/parse-css-to-stylesheet'

// Harmony
const { code } = parse(jsxCode, [cssCode1, cssCode2, ...], {
  platformString: 'Harmony',
  isEnableNesting: true // 支持解析嵌套选择器，默认关闭
})
// code: jsx代码 string
```

## 参数说明

```typescript
export interface ParseOptions {
  platformString: string; // 平台：'Harmony'
  isEnableNesting?: boolean; // 是否支持嵌套解析
}
export interface ParseResult {
  code: string; // 输出的jsxcode
}

// 样式解析
export function parse(
  component: string,
  styles: Array<string>,
  options: ParseOptions
): ParseResult;
```

#### ParseOptions

| 配置参数        | 类型    | 可选值                   | 说明             |
| --------------- | ------- | ------------------------ | ---------------- |
| platformString  | String  | 'Harmony'、'ReactNative' | 平台             |
| isEnableNesting | Boolean |                          | 样式嵌套解析开关 |

#### ParseResult

| 配置参数 | 类型   | 说明                      |
| -------- | ------ | ------------------------- |
| code     | String | 经过样式解析后的 JSX 代码 |

在 Harmony 中，编译结果会依赖`@tarojs/plugin-platform-harmony-ets`中提供的几个包方法：

1. `convertNumber2VP` 用于运行时进行单位转换
2. `calcStaticStyle` 用于合成类，匹配类名
3. `__combine_nesting_style__` 嵌套样式的合成

具体位于 [Taro 主仓](https://github.com/NervJS/taro) 路径：_/taro/packages/taro-platform-harmony/src/runtime-ets_ 中

## 样式支持情况

| 类型   | 举例合法值                                       |        备注         |
| ------ | ------------------------------------------------ | :-----------------: |
| Length | 10px、10vw、10vh、100%、10rem、calc(100% - 20px) |     1rem = 16px     |
| Color  | #f00、rgb(0,0,0)、rgba(0,0,0,0.2)、green         | 暂不支持 hsl 等方法 |
| Border | '1px solid #f00'                                 |    符合 w3c 规范    |

### 通用属性

所有元素都支持的样式：

| 属性                       | 可选值 / 单位                                                                                                                               | 支持情况 |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- | :------: |
| flex                       | 'flexGrow', 'flexShrink', 'flexBasis'                                                                                                       |    ✔️    |
| flex-grow                  | Number                                                                                                                                      |    ✔️    |
| flex-shrink                | Number                                                                                                                                      |    ✔️    |
| flex-basis                 | Length                                                                                                                                      |    ✔️    |
| flex-direction             | 'row','row-reverse','column','column-reverse'                                                                                               |    ✔️    |
| justify-content            | 'flex-start', 'flex-end', 'center', 'space-between', 'space-around', 'space-evenly'                                                         |    ✔️    |
| align-content              | 'flex-start', 'flex-end', 'center', 'space-between', 'space-around', 'space-evenly'                                                         |    ✔️    |
| align-items                | 'flex-start', 'flex-end', 'center', 'baseline', 'stretch'                                                                                   |    ✔️    |
| align-self                 | 'flex-start', 'flex-end', 'center', 'baseline', 'stretch' , 'auto'                                                                          |    ✔️    |
| flex-wrap                  | 'nowrap', 'wrap', 'wrap-reverse'                                                                                                            |    ✔️    |
| position                   | 'relative', 'absolute', 'fixed'                                                                                                             |    ✔️    |
| left                       | Length                                                                                                                                      |    ✔️    |
| top                        | Length                                                                                                                                      |    ✔️    |
| bottom                     | Length                                                                                                                                      |    ❌    |
| right                      | Length                                                                                                                                      |    ❌    |
| z-index                    | Number                                                                                                                                      |    ✔️    |
| bottom                     | Length                                                                                                                                      |    ✔️    |
| margin                     | Length \ Length Length \ Length Length Length \ Length Length Length Length                                                                 |    ✔️    |
| margin-top                 | Length                                                                                                                                      |    ✔️    |
| margin-right               | Length                                                                                                                                      |    ✔️    |
| margin-bottom              | Length                                                                                                                                      |    ✔️    |
| margin-left                | Length                                                                                                                                      |    ✔️    |
| padding                    | Length \ Length Length \ Length Length Length \ Length Length Length Length                                                                 |    ✔️    |
| padding-top                | Length                                                                                                                                      |    ✔️    |
| padding-right              | Length                                                                                                                                      |    ✔️    |
| padding-bottom             | Length                                                                                                                                      |    ✔️    |
| padding-left               | Length                                                                                                                                      |    ✔️    |
| width                      | Length                                                                                                                                      |    ✔️    |
| height                     | Length                                                                                                                                      |    ✔️    |
| min-height                 | Length                                                                                                                                      |    ✔️    |
| min-width                  | Length                                                                                                                                      |    ✔️    |
| max-height                 | Length                                                                                                                                      |    ✔️    |
| max-width                  | Length                                                                                                                                      |    ✔️    |
| background                 |                                                                                                                                             |    ✔️    |
| background-color           | Color                                                                                                                                       |    ✔️    |
| background-image           | "src('xxx')", "linear-gradient(xxx)", "radial-gradient(xxx)" 支持图片资源和性渐变                                                           |    ✔️    |
| background-size            | 'cover', 'contain', Length(x y), Length(x) Length(y)                                                                                        |    ✔️    |
| background-position        | center', 'top', 'bottom', 'left', 'right', , Length(x y), Length(x) Length(y)                                                               |    ✔️    |
| background-repeat          | 'repeat', 'no-repeat', 'repeat-x', 'repeat-y'                                                                                               |    ✔️    |
| border                     | Border（可设置 4 个值，控制 4 个方向）                                                                                                      |    ✔️    |
| border-top                 | Border                                                                                                                                      |    ✔️    |
| border-left                | Border                                                                                                                                      |    ✔️    |
| border-right               | Border                                                                                                                                      |    ✔️    |
| border-bottom              | Border                                                                                                                                      |    ✔️    |
| border-color               | Color（可设置 4 个值，控制 4 个方向）                                                                                                       |    ✔️    |
| border-top-color           | Color                                                                                                                                       |    ✔️    |
| border-right-color         | Color                                                                                                                                       |    ✔️    |
| border-bottom-color        | Color                                                                                                                                       |    ✔️    |
| border-left-color          | Color                                                                                                                                       |    ✔️    |
| border-radius              | Length（可设置 4 个值，控制 4 个方向）, 不支持百分比                                                                                        |    ✔️    |
| border-top-left-radius     | Length, 不支持百分比                                                                                                                        |    ✔️    |
| border-top-right-radius    | Length, 不支持百分比                                                                                                                        |    ✔️    |
| border-bottom-left-radius  | Length, 不支持百分比                                                                                                                        |    ✔️    |
| border-bottom-right-radius | Length, 不支持百分比                                                                                                                        |    ✔️    |
| border-style               | 'dotted', 'dashed', 'solid' （4 个值，控制 4 个方向）                                                                                       |    ✔️    |
| border-top-style           | 'dotted', 'dashed', 'solid'                                                                                                                 |    ✔️    |
| border-right-style         | 'dotted', 'dashed', 'solid'                                                                                                                 |    ✔️    |
| border-bottom-style        | 'dotted', 'dashed', 'solid'                                                                                                                 |    ✔️    |
| border-left-style          | 'dotted', 'dashed', 'solid'                                                                                                                 |    ✔️    |
| opacity                    | Number                                                                                                                                      |    ✔️    |
| display                    | 'flex', 'none', 'block'                                                                                                                     |    ✔️    |
| display                    | 'inline-block', 'inline-flex', 'inline'                                                                                                     |    ❌    |
| overflow                   | 'hidden', 'visible', 'scroll', 'auto'                                                                                                       |    ✔️    |
| transform                  | translate、translateX、translateY、translateZ、translate2d、translate3d、scale、scaleX、scaleY、scale3d、rotate、rotateX、rotateY、rotate3d |    ✔️    |
| transform-origin           | Length(top/center/bottom) Length(left/center/right)                                                                                         |    ✔️    |
| animation                  | 仅支持 animation-name, animation-duration , animation-timing-function, animation-delay, animation-iteration-count， 暂不支持 style 设置     |    ✔️    |
| box-shadow                 |                                                                                                                                             |    ✔️    |
| content                    |                                                                                                                                             |    ✔️    |

⚠️ 注意：

- `transform` 不允许连续出现 2 个同类型如：transform: translate(20px 20px) translate3d(10px, 30px, 30px)
- `radial-linear`
  - \<radial-extent>不支持, 如（closest-side、closest-corner、farthest-side、farthest-corner）
- `display` 不支持 **行内**
- 定位不支持 **bottom** 和 **right**

### 文本样式

| 属性               | 可选值 / 单位                                           | 支持情况 |
| ------------------ | ------------------------------------------------------- | :------: |
| font-size          | Length                                                  |    ✔️    |
| font-family        |                                                         |    ✔️    |
| font-style         | 'normal', 'italic'                                      |    ✔️    |
| font-weight        | 100~900, 'bold','bolder','lighter','normal'             |    ✔️    |
| line-height        | 'XXpx' (需要指定具体指，不支持 Number)                  |    ✔️    |
| text-align         | 'center', 'left', 'right'                               |    ✔️    |
| text-decoration    | ('none', 'underline', 'line-through', 'overline') Color |    ✔️    |
| text-overflow      | 'ellipsis', 'clip'                                      |    ✔️    |
| vertical-align     | 'middle', 'top', 'bottom'                               |    ✔️    |
| color              | Color                                                   |    ✔️    |
| -webkit-line-clamp | Number                                                  |    ✔️    |

⚠️ 注意：

- 文本样式 **仅对`<Text></Text>`节点生效**
- 文本样式 **不支持继承**
- `line-height`**不支持数值**

**以下两种情况是正确的对文本进行样式添加的案例：**

1.直接将样式添加在`<Text/>`上

```jsx
// ✅ 允许
<Text className="txt">hello</Text>
```

2.样式添加到`<View/>`下是一个文本内容

```jsx
// ✅ 允许
<View className="txt">hello</View>
```

**错误案例：**

```jsx
// ❌ hello 父级没有添加文本样式，txt的文本属性无法继承下去
<View className="txt">
  <Text>hello</Text>
</View>
```

## CSS 选择器

### 通用选择器

注意点：

- 支持**类选择器**，
- 不支持**ID 选择器、标签选择器、属性选择器**

| 选择器             | 示例                | 示例说明                                                     | Harmony |   备注   |
| ------------------ | ------------------- | ------------------------------------------------------------ | :-----: | :------: |
| .class             | .intro              | 选择所有 class="intro" 的元素                                |   ✔️    |          |
| .class.class       | .red.big            | 选择所有 class="red big" 元素                                |   ✔️    |          |
| .class, .class     | .item, .text        | 选择所有 class="item" 元素和 class="text" 元素               |   ✔️    |          |
| .class .class      | .grandfather .child | 选择所有 class="grandfather" 内所有的 class="child" 的元素   |   ✔️    |          |
| .class > .class    | .parent > .child    | 选择所有 父级是 class="parent"的 class="child" 元素          |   ✔️    |          |
| .class+.class      | .red+.big           | 选择所有紧跟在 class="red" 元素之后的第一个 class="big" 元素 |   ❌    | 后续支持 |
| .class~.class      | .red~.big           | 选择所有紧跟在 class="red" 之后的每一个 class="big" 元素     |   ❌    | 后续支持 |
| #id                | #firstname          | 选择所有 id="firstname"的元素                                |   ❌    |          |
| \*                 | \*                  | 选择所有元素                                                 |   ❌    |  不支持  |
| element            | p                   | 选择所有\<p>元素                                             |   ❌    |          |
| \[attribute]       | \[target]           | 选择所有带有 target 属性元素                                 |   ❌    |  不支持  |
| \[attribute=value] | \[target=blank]     | 选择所有使用 target="blank"的元素                            |   ❌    |  不支持  |
| ...                |                     | 其他                                                         |   ❌    |          |

### 伪元素 / 伪类

- 支持**before、after**，

| 选择器       | 示例                | 示例说明                                      | 支持情况 | 备注 |
| ------------ | ------------------- | --------------------------------------------- | :------: | :--: |
| :before      | .intro:before       | 在每个 class="intro" 元素之前插入内容         |    ✔️    |      |
| :after       | .intro:after        | 在每个 class="intro" 元素之后插入内容         |    ✔️    |      |
| :nth-child() | .intro:nth-child(2) | 选择 class="intro" 元素是其父级的第二个子元素 |    ✔️    |      |
| :first-child | .intro:first-child  | 选择 class="intro" 元素是其父级的第一个子级   |    ✔️    |      |
| :last-child  | .intro:last-child   | 选择 class="intro" 元素是其父级的最后一个子级 |    ✔️    |      |
| :empty       | .intro:empty        | 选择 class="intro" 元素并且其没有子级         |    ✔️    |      |
| :checked     | input:checked       | 选择每个选中的输入元素                        |    ❌    |      |
| ...          |                     | 其他                                          |    ❌    |      |

## CSS 变量

⚠️：暂不支持 **动态修改 Css 变量的值**

```css
:root {
  --color: #403635;
  --angle: 30deg;
  --var: var(--color, 30px);
}

.hello {
  height: 30px;
  color: var(--color);
}
```

## 常见问题

### 1. 跨组件传递 className、style

#### ❌ 错误做法

比如业务上针对`@tarojs/components`的组件进行重导出，如引入了`Image`，对其进行了**二次封装**，然后通过一个入口统一导出如：

```js
// ./components.js
import { View, Text } from "@tarojs/components";

// 这里的Image实际上是对TaroImage的二次封装，一样的暴露出style和classname使用
export { default as Image } from "./xxxx";
```

- 在 Taro 编译的视角来看`<Image/>`已经是一个**自定义组件**，并且它接收了`className`，也就说明了它的类名其实是往下传递了，我们会在运行时进行**样式合成**
- `<View/>`和`<Text/>`其实是原封不动的直接导出的，本质上它并不是一个自定义组件，所以 Taro 在编译时，会在**编译阶段将样式赋予上去**

```js
// 注意：这里的组件从统一入口进行导入
import { View, Image } from "./components";

function Index() {
  return (
    <View className="xxx">
      <Image className="xxxxxx" />
    </View>
  );
}
```

但是问题来了，这里在实际使用时，`<View/>`和`<Image/>`都是通过`'./components'`导入，编译阶段无法知道他们是**Taro 组件**还是**自定义组件**，顾在实际运行时，都会视为**自定义组件**对待

因为**自定义组件**是在**运行时动态合成样式**，顾性能远不及**Taro 组件**

#### ✅ 正确做法

如果 Taro 组件没有二次封装，我们建议从`@tarojs/components`导入，提供编译的优化效果

```js
// 自定义组件引入
import { Image } from "./components";
// Taro组件引入
import { View } from "@tarojs/components";

function Index() {
  return (
    <View className="xxx">
      <Image className="xxxxxx" />
    </View>
  );
}
```
