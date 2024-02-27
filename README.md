# parse-css-to-stylesheet

## 简介

解析 React 组件与对应的 CSS 文件，为每一个 React 节点计算样式最终样式，应用于 React Native 、鸿蒙等不支持 CSS 写法的场景，目前仅支持类名选择器。

## 使用方式

```typescript
import { parse } from '@tarojs/parse-css-to-stylesheet'

// Harmony
const code = parse(jsxCode, [cssCode1, cssCode2, ...], "Harmony")
// ReactNative
// const code = parse(jsxCode, [cssCode1, cssCode2, ...], "ReactNative")

```
