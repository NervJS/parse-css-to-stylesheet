# parse-css-to-stylesheet

## 简介

解析 React 组件与对应的 CSS 文件，为每一个 React 节点计算样式最终样式，应用于 React Native 、鸿蒙等不支持 CSS 写法的场景

## 使用方式

```typescript
import { parse } from '@tarojs/parse-css-to-stylesheet'

// Harmony
const code = parse(jsxCode, [cssCode1, cssCode2, ...], {
  platformString: 'Harmony',
  isEnableNesting: true // 支持解析嵌套选择器，默认关闭
})
```

## 样式支持情况

| 类型   | 举例合法值                                       |        备注         |
| ------ | ------------------------------------------------ | :-----------------: |
| Length | 10px、10vw、10vh、100%、10rem、calc(100% - 20px) |     1rem = 16px     |
| Color  | #f00、rgb(0,0,0)、rgba(0,0,0,0.2)、green         | 暂不支持 hsl 等方法 |
| Border | '1px solid #f00'                                 |    符合 w3c 规范    |

### 通用属性

所有元素都支持的样式：

| 属性                       | 可选值 / 单位                                                                                                                               | ✔️  |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- | :-: |
| flex                       | `flexGrow  flexShrink flexBasis`                                                                                                            | ✔️  |
| flexGrow                   | Number                                                                                                                                      | ✔️  |
| flexShrink                 | Number                                                                                                                                      | ✔️  |
| flexBasis                  | Length                                                                                                                                      | ✔️  |
| flexDirection              | 'row','row-reverse','column','column-reverse'                                                                                               | ✔️  |
| justifyContent             | 'flex-start', 'flex-end', 'center', 'space-between', 'space-around', 'space-evenly'                                                         | ✔️  |
| alignContent               | 'flex-start', 'flex-end', 'center', 'space-between', 'space-around', 'space-evenly'                                                         | ✔️  |
| alignItems                 | 'flex-start', 'flex-end', 'center', 'baseline', 'stretch' , 'auto'                                                                          | ✔️  |
| alignSelf                  | 'flex-start', 'flex-end', 'center', 'baseline', 'stretch' , 'auto'                                                                          | ✔️  |
| flexWrap                   | 'nowrap', 'wrap', 'wrap-reverse'                                                                                                            | ❌  |
| position                   | 'relative', 'absolute'                                                                                                                      | ✔️  |
| left                       | Length                                                                                                                                      | ✔️  |
| top                        | Length                                                                                                                                      | ❌  |
| right                      | Length                                                                                                                                      | ❌  |
| zIndex                     | Number                                                                                                                                      | ✔️  |
| bottom                     | Length                                                                                                                                      | ✔️  |
| margin                     | Length \ Length Length \ Length Length Length \ Length Length Length Length                                                                 | ✔️  |
| marginTop                  | Length                                                                                                                                      | ✔️  |
| marginRight                | Length                                                                                                                                      | ✔️  |
| marginBottom               | Length                                                                                                                                      | ✔️  |
| marginLeft                 | Length                                                                                                                                      | ✔️  |
| padding                    | Length \ Length Length \ Length Length Length \ Length Length Length Length                                                                 | ✔️  |
| paddingTop                 | Length                                                                                                                                      | ✔️  |
| paddingRight               | Length                                                                                                                                      | ✔️  |
| paddingBottom              | Length                                                                                                                                      | ✔️  |
| paddingLeft                | Length                                                                                                                                      | ✔️  |
| width                      | Length                                                                                                                                      | ✔️  |
| height                     | Length                                                                                                                                      | ✔️  |
| minHeight                  | Length                                                                                                                                      | ✔️  |
| minWidth                   | Length                                                                                                                                      | ✔️  |
| maxHeight                  | Length                                                                                                                                      | ✔️  |
| maxWidth                   | Length                                                                                                                                      | ✔️  |
| background                 |                                                                                                                                             | ✔️  |
| background-color           | Color                                                                                                                                       | ✔️  |
| background-image           | "src('xxx')"                                                                                                                                | ✔️  |
| background-size            | 'cover', 'contain', Length(x y), Length(x) Length(y)                                                                                        | ✔️  |
| background-position        | center', 'top', 'bottom', 'left', 'right', , Length(x y), Length(x) Length(y)                                                               | ✔️  |
| background-repeat          | 'repeat', 'no-repeat', 'repeat-x', 'repeat-y'                                                                                               | ✔️  |
| border                     | Border（可设置 4 个值，控制 4 个方向）                                                                                                      | ✔️  |
| border-top                 | Border                                                                                                                                      | ✔️  |
| border-left                | Border                                                                                                                                      | ✔️  |
| border-right               | Border                                                                                                                                      | ✔️  |
| border-bottom              | Border                                                                                                                                      | ✔️  |
| border-color               | Color（可设置 4 个值，控制 4 个方向）                                                                                                       | ✔️  |
| border-top-color           | Color                                                                                                                                       | ✔️  |
| border-right-color         | Color                                                                                                                                       | ✔️  |
| border-bottom-color        | Color                                                                                                                                       | ✔️  |
| border-left-color          | Color                                                                                                                                       | ✔️  |
| border-radius              | Length（可设置 4 个值，控制 4 个方向）                                                                                                      | ✔️  |
| border-top-left-radius     | Length                                                                                                                                      | ✔️  |
| border-top-right-radius    | Length                                                                                                                                      | ✔️  |
| border-bottom-left-radius  | Length                                                                                                                                      | ✔️  |
| border-bottom-right-radius | Length                                                                                                                                      | ✔️  |
| border-style               | 'dotted', 'dashed', 'solid' （4 个值，控制 4 个方向）                                                                                       | ✔️  |
| border-top-style           | 'dotted', 'dashed', 'solid'                                                                                                                 | ✔️  |
| border-right-style         | 'dotted', 'dashed', 'solid'                                                                                                                 | ✔️  |
| border-bottom-style        | 'dotted', 'dashed', 'solid'                                                                                                                 | ✔️  |
| border-left-style          | 'dotted', 'dashed', 'solid'                                                                                                                 | ✔️  |
| opacity                    | Number                                                                                                                                      | ✔️  |
| display                    | 'flex', 'none', 'block'                                                                                                                     | ✔️  |
| display                    | 'inline-block', 'inline-flex', 'inline'                                                                                                     | ❌  |
| overflow                   | 'hidden', 'visible'                                                                                                                         | ✔️  |
| transform                  | translate、translateX、translateY、translateZ、translate2d、translate3d、scale、scaleX、scaleY、scale3d、rotate、rotateX、rotateY、rotate3d | ✔️  |
| transform-origin           | Length Length                                                                                                                               | ✔️  |
| content                    |                                                                                                                                             | ✔️  |

⚠️ 注意：

- `transform` 不允许连续出现 2 个同类型如：transform: translate(20px 20px) translate3d(10px, 30px, 30px)
- `display` 不支持**行内**
- 定位不支持 **bottom** 和 **right**

### 文本样式

| 属性               | 可选值 / 单位                                           | ✔️  |
| ------------------ | ------------------------------------------------------- | :-: |
| font-size          | Length                                                  | ✔️  |
| font-family        |                                                         | ✔️  |
| font-style         | 'normal', 'italic'                                      | ✔️  |
| font-weight        | 100~900, 'bold','bolder','light','lighter','normal'     | ✔️  |
| line-height        | 'XXpx' (需要指定具体指，不支持 Number)                  | ✔️  |
| text-align         | 'center', 'left', 'right'                               | ✔️  |
| text-decoration    | ('none', 'underline', 'line-through', 'overline') Color | ✔️  |
| text-overflow      | 'ellipsis', 'clip'                                      | ✔️  |
| color              | Color                                                   | ✔️  |
| -webkit-line-clamp | Number                                                  | ✔️  |

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

| 选择器             | 示例                | 示例说明                                                     | Harmony |
| ------------------ | ------------------- | ------------------------------------------------------------ | :-----: |
| .class             | .intro              | 选择所有 class="intro" 的元素                                |   ✔️    |
| .class.class       | .red.big            | 选择所有 class="red big" 元素                                |   ✔️    |
| .class, .class     | .item, .text        | 选择所有 class="item" 元素和 class="text" 元素               |   ✔️    |
| .class .class      | .grandfather .child | 选择所有 class="grandfather" 内所有的 class="child" 的元素   |   ✔️    |
| .class > .class    | .parent > .child    | 选择所有 父级是 class="parent"的 class="child" 元素          |   ✔️    |
| .class+.class      | .red+.big           | 选择所有紧跟在 class="red" 元素之后的第一个 class="big" 元素 |   ❌    |
| .class~.class      | .red~.big           | 选择所有紧跟在 class="red" 之后的每一个 class="big" 元素     |   ❌    |
| #id                | #firstname          | 选择所有 id="firstname"的元素                                |   ❌    |
| \*                 | \*                  | 选择所有元素                                                 |   ❌    |
| element            | p                   | 选择所有\<p>元素                                             |   ❌    |
| \[attribute]       | \[target]           | 选择所有带有 target 属性元素                                 |   ❌    |
| \[attribute=value] | \[target=blank]     | 选择所有使用 target="blank"的元素                            |   ❌    |
| ...                |                     | 其他                                                         |   ❌    |

### 伪类

- 支持**before、after**，

| 选择器            | 示例                     | 示例说明                                                          | Harmony |
| ----------------- | ------------------------ | ----------------------------------------------------------------- | :-----: |
| :before           | .intro:before            | 在每个 class="intro" 元素之前插入内容                             |   ✔️    |
| :after            | .intro:after             | 在每个 class="intro" 元素之后插入内容                             |   ✔️    |
| :nth-child()      | .intro:nth-child(2)      | 选择 class="intro" 元素是其父级的第二个子元素                     |   ❌    |
| :nth-last-child() | .intro:nth-last-child(2) | 选择 class="intro" 元素是其父级的第二个子元素, 从最后一个子项计数 |   ❌    |
| :first-child      | .intro:first-child       | 选择 class="intro" 元素是其父级的第一个子级                       |   ❌    |
| :last-child       | .intro:last-child        | 选择 class="intro" 元素是其父级的最后一个子级                     |   ❌    |
| :root             | :root                    | 选择文档的根元素                                                  |   ❌    |
| :checked          | input:checked            | 选择每个选中的输入元素                                            |   ❌    |
| ...               |                          | 其他                                                              |   ❌    |
