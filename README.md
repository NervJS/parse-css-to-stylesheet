# parse-css-to-stylesheet

## 简介

解析 React 组件与对应的 CSS 文件，为每一个 React 节点计算样式最终样式，应用于 React Native 、鸿蒙等不支持 CSS 写法的场景

## 使用方式

```typescript
import { parse } from '@tarojs/parse-css-to-stylesheet'

// Harmony
const code = parse(jsxCode, [cssCode1, cssCode2, ...], {
  platformString: 'Harmony',
  isEnableNesting: true // 支持生成嵌套选择器
})
```

## 支持选择器

#### 类选择器

```css
.item {
  /* ... */
}
```

#### 后代组合器

```css
.parent .child {
  /* ... */
}
```

#### 直接子代选择器

```css
.parent > .child {
  /* ... */
}
```

#### 多类选择器

```css
.child1.child2 {
  /* ... */
}
```

#### 伪类

```css
.child1::before {
  /* ... */
}
.child1::after {
  /* ... */
}
```
