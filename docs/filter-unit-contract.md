# filter / backdrop-filter 的编译单位

2026-10-04：Harmony 恢复与其他编译 CSS 长度相同的设计单位协议。

## 默认原生链路

业务 `designWidth: 375`、默认 pxtransform 配置、原生布局模式下：

| 阶段 | CSS 文件 | inline 字符串 |
| --- | --- | --- |
| 作者输入 | `filter: blur(4px)` | `filter: 'blur(4px)'` |
| 默认 pxtransform | `blur(8px)` | 不经过 PostCSS |
| 本编译器输出 | `blur(8lpx)` | 不经过本编译器 |
| TaroCSS 长度解析 | 8 LPX × designScaleRatio | 4 VP × scaleRatio |
| 相同环境的逻辑半径 | `4 × scaleRatio` | `4 × scaleRatio` |

TaroCSS 的 `designScaleRatio = scaleRatio / 2`。因此中间值 8 不是最终 8 个物理像素，也不能为了对齐而给 inline 再乘 2。

2026-10-04 用户将鸿蒙 blur 渲染口径改为逻辑 vp：端侧把解析后的逻辑长度乘设备 density，转换为 CAPI 所需的物理 px。例如 scaleRatio=1、density=3.125 时，两条链路都传 4vp，最终下发 12.5px。此前“相同数值直接当物理 px”的策略已被取代；编译器/inline 的单位一致性修复仍然保留。

## 修复与边界

- 删除 Harmony filter/backdrop-filter 保留 `px` 的特殊处理，统一使用已有 `generate_expr_lit_str!` 的 `px -> lpx` 归一化。否则 PostCSS 的 `8px` 会被字符串解析器当成 8 VP，造成 CSS 比 inline 模糊。
- 不允许用 `propList` 排除 filter/backdrop-filter 规避正常设计稿换算。
- ReactNative 已有 filter 设计单位输出不变；编译器单位修复本身不改 FlatBuffer 协议、TaroCSS/JSI/inline filter 解析器。最新鸿蒙 vp 渲染策略仅修改端侧 CAPI 消费边界，见上方 density 换算说明。
- 表格和回归针对默认 375 配置及原生模式，不代表任意自定义 designWidth/deviceRatio 或 Web 模式的数值契约。
- CSS/inline 单位一致不代表所有 ArkUI 滤镜效果与浏览器逐像素等价。

## 回归

- `__test__/filter-unit-parity.spec.mjs`：Harmony/RN 的零值、小数、整数、背景 blur 与组合输出。
- `scripts/generate-filter-unit-fixtures.cjs`：使用消费工程实际安装的 PostCSS/pxtransform，无属性排除，生成 FlatBuffer v1/v2。
- TaroCSS `filter_flatbuffer_parsing_test.cpp`：两个版本对照独立写出的 inline 输入，覆盖 10 个值 × 27 种环境 × 2 版本，并用旧 `blur(8px)` 作为负向对照。

生成 fixture 的命令（在本仓库执行；构建 Node addon 后运行）：

```sh
node scripts/generate-filter-unit-fixtures.cjs \
  /Users/zhuminghui2/workspace/taro-next/packages/TaroCSS/parser/__tests__/test_data \
  /Users/zhuminghui2/workspace/harmony-project/taro-unified-runtime-demo/node_modules/postcss-pxtransform/index.js
```
