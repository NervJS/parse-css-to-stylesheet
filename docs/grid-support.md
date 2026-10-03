# CSS Grid 编译支持技术方案

## 1. 背景与目标

`parse-css-to-stylesheet` 此前仅支持 flex 布局，业务在鸿蒙端对 grid 布局（宫格、卡片墙、对齐矩阵等场景）有实际需求。本次在编译期将 **CSS Grid 跨端基础集** 解析为结构化数值产物，鸿蒙侧运行时**零字符串解析**即可消费。

支持范围（跨端基础集）：

| JWT/jtransaction CSS 属性 | 支持能力 |
| --- | --- |
| `display: grid / inline-grid / inline-flex` | 数值枚举分别为 6 / 7 / 8，和 TaroCSS 字符串解析保持一致 |
| `grid-template-columns` / `grid-template-rows` | px / vw / vh 等长度、百分比、`fr`、`min/max-content`、`auto`、`minmax()`、`fit-content()`；`repeat(n, …)` 编译期展开；`repeat(auto-fill/auto-fit, …)` 变长槽直出 |
| `grid-template` 简写（rows / columns） | 编译期展开为两行/两列模板，**不含 areas 的简写** |
| `grid-auto-rows` / `grid-auto-columns` | 与 template 同构的 track-size 列表（无 none/repeat 外层） |
| `grid-auto-flow` | row / column / dense 位组合直出整数（0/1/2/3） |
| `grid-row-start / -end`、`grid-column-start / -end` | 线号、`span n`、`auto` |
| `grid-row` / `grid-column` 简写 | start / end 两条线 |
| `grid-area` 简写 | 四条线：row-start / column-start / row-end / column-end |
| `justify-items` / `justify-self` | 复用既有 align-items / align-self 数值体系（flex-start、end、center、stretch、baseline） |
| `place-items` / `place-self` / `place-content` 简写 | 编译期展开为 align-* + justify-* 两条长属性 |
| `gap` / `justify-content` / `align-content` | 走既有实现；content 的 `stretch` 使用枚举值 4 |

**明确不支持（涉及运行时布局引擎，属后续方案）**：`grid-template-areas`、命名 grid-area、命名网格线、`subgrid`。

**lightningcss 兼容处理**：1.0.0-alpha.45 将单独的 `grid-auto-flow: dense` 留作 Unparsed；编译入口将其转换为 `row dense` 的数值 2。

## 2. 关键设计：扁平数值槽（方案 A）

早期原型把模板输出为字符串（如 `"1fr 2fr"`），运行时需要再写一遍 CSS 值解析器，属于冗余。数值槽设计的目标：**编译期把 CSS 值的所有 token 种类、数值、单位全部解析完毕**，产物是扁平 `DoubleArray`，运行时按定长 stride 消费即可。

### 2.1 grid-template-columns / rows（属性 id 118 / 119）

扁平数组，**每 3 个数描述一条轨道**：`[类型, 值, 单位]`。

**类型（type）：**

| 值 | 含义 | 槽布局 |
| --- | --- | --- |
| 0 | 百分比 | `[0, 比例, 0]`（50% → 0.5） |
| 1 | 长度 | `[1, 数值, 单位]`（单位枚举见下） |
| 2 | flex（fr） | `[2, fr 数, 0]` |
| 3 | 关键字 | `[3, 0, 关键字]`（关键字见下） |
| 4 | minmax(min, max) | **7 槽**：`[4, min(3槽), max(3槽)]`，min/max 各为 type 0-3 轨道 |
| 5 | fit-content(limit) | **4 槽**：`[5, limit(3槽)]`，limit 为 type 0/1 轨道 |
| 6 | repeat(auto-fill, …) | **变长**：`[6, 1, tracks...]`，次数槽固定 1（占位），后跟内层轨道 |
| 7 | repeat(auto-fit, …) | **变长**：`[7, 1, tracks...]`，同上 |

type 4-7 为变长结构，消费端需递归解码：遇 4 读 1+3+3 槽，遇 5 读 1+3 槽，遇 6/7 读 2 槽后递归读轨道列表直到耗尽。

**长度单位枚举（unit）**：0=px, 1=vw, 2=vh, 3=vmin, 4=vmax, 5=rem, 6=em, 7=ch, 8=ex
**关键字（unit 槽，走 ≥100 区段避免与单位碰撞）**：100=auto, 101=min-content, 102=max-content

**示例：**

```css
.a { grid-template-columns: 1fr 100px auto 2fr; }
.b { grid-template-rows: 50% min-content 1fr; }
.c { grid-template-columns: repeat(3, 1fr); }
```

```jsonc
// .a → [118, [2,1,0, 1,100,0, 3,0,100, 2,2,0]]
//      (1fr)      (100px)      (auto)      (2fr)
// .b → [119, [0,0.5,0, 3,0,101, 2,1,0]]
// .c → [118, [2,1,0, 2,1,0, 2,1,0]]   // repeat(3, 1fr) 编译期展开
// none → [118, []]                      // 空数组表示显式 none
// minmax(100px, 1fr)        → [4, 1,100,0, 2,1,0]
// fit-content(200px)        → [5, 1,200,0]
// repeat(auto-fill, 100px)  → [6, 1, 1,100,0]
// repeat(auto-fit, minmax(50px, 1fr)) → [7, 1, 4,1,50,0,2,1,0]
```

### 2.1.1 grid-auto-rows / grid-auto-columns（id 129 / 130）

与 template 同构的 track-size 列表，复用同一套数值槽编码（无 `none` / 外层 `repeat` 结构，但内层可含 minmax/fit-content）：

```css
.a { grid-auto-rows: 100px; grid-auto-columns: minmax(50px, auto); }
```

```jsonc
// → [129, [1,100,0]], [130, [4, 1,50,0, 3,0,100]]
```

### 2.1.2 grid-auto-flow（id 131）

lightningcss 的 `GridAutoFlow` 是 bitflags（Row=0b00, Column=0b01, Dense=0b10），位组合与鸿蒙侧 runtime 的 grid-auto-flow 枚举数值一一对应，直接输出 bits 整数：

| 写法 | 值 |
| --- | --- |
| `row`（默认） | 0 |
| `column` | 1 |
| `row dense` / `dense row` | 2 |
| `column dense` / `dense column` | 3 |

### 2.2 grid 定位属性（id 120–126）

每条线 2 个槽：`[类型, 值]`。

| 属性 | id | 槽数 | 布局 |
| --- | --- | --- | --- |
| `grid-row-start` / `grid-column-start` | 120 / 122 | 2 | `[type, index]` |
| `grid-row-end` / `grid-column-end` | 121 / 123 | 2 | `[type, index]` |
| `grid-row` / `grid-column` | 124 / 125 | 4 | `[start..., end...]` |
| `grid-area` | 126 | 8 | `[row-start, col-start, row-end, col-end]` |

**线类型（type）：** 0=auto, 1=线号（index 可为负，如 `grid-column-end: -1` → `[1, -1]`）, 2=span

**示例：**

```css
.item1 { grid-column: 2 / span 2; grid-row: 1 / 3; }
.item2 { grid-column-start: span 2; }
.item3 { grid-area: 1 / 1 / span 2 / span 3; }
```

```jsonc
// .item1 → [125, [1,2, 2,2]],         [124, [1,1, 1,3]]
// .item2 → [122, [2,2]]               (span 2)
// .item3 → [126, [1,1, 1,1, 2,2, 2,3]]
```

### 2.3 display 与对齐

- `display: grid` → 6（`Display::Grid`），`display: inline-grid` → 7（`Display::InlineGrid`），追加在既有 0–5 之后，老产物枚举值不受影响。
- `display: inline-flex` → 8；`justify-content: stretch` / `align-content: stretch` → 4，与字符串解析和运行时枚举一致。
- `justify-items` / `justify-self` 走 `ItemAlign` 数值体系，与 `align-items` / `align-self` 同枚举，运行时一套解析代码同时服务 flex 和 grid。
- `place-items` / `place-self` / `place-content` 简写在编译期展开为两条长属性（`place-items: center stretch` → `align-items: center` + `justify-items: stretch`），运行时无需感知简写，产物中不出现 place-* 条目。
- `place-*` 含 `var()` / `env()` 时保留原声明，以属性 ID 132 / 133 / 134 交给运行时变量及字符串解析路径。

## 3. 回退与降级策略

> 原则：**能数值化就数值化；不能数值化的尽量兜底为字符串；语义无映射才丢弃。** 产物永远合法，绝不中断编译。

| 情况 | 产物行为 | 示例 |
| --- | --- | --- |
| `minmax()` / `fit-content()` / `repeat(auto-fill/auto-fit, …)` | **已数值化**（type 4-7 变长槽，见 2.1） | `[118, [4, 1,100,0, 2,1,0]]` |
| `calc()` 等无法静态求值的结构 | 整条声明回退为 CSS 字符串（px → lpx 照常转换），运行时收到 `string` 而非数组 | `[118, "calc(100lpx - 10lpx) 1fr"]` |
| 命名网格线（`grid-column: sidebar-start / content-end`） | **整条声明被丢弃**（鸿蒙侧无线名概念，静默忽略比错误输出安全） | 无 `[12x, …]` 条目 |
| `grid-template` 简写含 `areas`（行内 `"A B"` 字符串） | **整个声明被跳过**（areas 属后续方案，避免输出残缺布局误导运行时） | 无 `[118]/[119]` 条目 |
| `subgrid` | lightningcss alpha.45 无此 AST 变体，声明按 Unparsed 丢弃 | 无条目 |
| `grid-auto-flow: dense`（单独写） | 兼容 lightningcss 的 Unparsed 分支，等同 `row dense` | `[131, 2]` |

## 4. 实现要点

- **AST 层**：基于 lightningcss `Property::GridTemplateColumns` / `GridTemplate` / `GridArea` / `GridAutoRows` / `GridAutoFlow` 等结构化 AST，解析出 `TrackSizing` / `GridLine` 枚举后做数值槽映射；`TrackBreadth`（含 `MinMax` / `FitContent`）、`GridLine::Line/Span/Auto`、`repeat(auto-fill/auto-fit)` 均可完全映射，`calc()` / 命名线 / `subgrid` 走回退或丢弃分支。
- **不写快照数字的 `generate_expr_lit_num_grid!` 宏**：既有数值宏默认保留两位小数，索引、类型码、单位码必须原值输出，新增独立宏保证槽位精确。
- **简写展开在 `parse_style_properties.rs` 完成**：`grid-template: rows / columns` 的 `areas: None` 分支展开为 `GridTemplateRows` + `GridTemplateColumns` 两条声明，让下游 `GridTemplate` 类型只需处理长属性 + 回退字符串。
- **flatbuffer v2 无 schema 变更**：数值槽落进既有 `DoubleArray`，字符串回退落进既有 `string`，v1/v2 双产物天然兼容。

## 5. 验证

`__test__/index.spec.mjs` 的 Grid 相关用例覆盖以下路径（当前全套 52 个测试通过）：

- `Harmony attrbute test grid`：cover fr / px / em / min-content / max-content / 百分比 / none / repeat 展开 / minmax 与 auto-fill 数值槽 / 全部定位属性（线号、负数、span、area）/ 命名线丢弃
- `Harmony attrbute test grid template shorthand`：`grid-template: 1fr auto / 1fr 2fr` → `[119, [2,1,0, 3,0,100]]` + `[118, [2,1,0, 2,2,0]]`（简写展开为数值槽，而非回退字符串）
- `Harmony attrbute test grid full features`：minmax（含百分比/内容关键字边界）、fit-content（px/%）、repeat(auto-fill/auto-fit)（含嵌套 minmax）、grid-auto-rows/columns、grid-auto-flow 全位组合、place-items/self/content 展开（含单值 baseline）
- `Harmony grid flatbuffer v2 round trip`：含 grid 属性的样式表编译为 v2 二进制后能被 `flatbuffers` 官方 JS 包完整读回（选择器、display、模板数组、跨属性同 id 去重全链路验证）

## 6. 运行时消费伪码（鸿蒙侧）

```cpp
// grid-template-columns → vector<Track>s
for (i = 0; i + 2 < arr.size(); i += 3) {
  switch ((int)arr[i]) {
    case 0: track.type = PERCENTAGE; track.pct  = arr[i+1]; break;
    case 1: track.type = LENGTH;     track.len  = arr[i+1];
            track.unit = Unit(arr[i+2]); break;
    case 2: track.type = FLEX;       track.fr   = arr[i+1]; break;
    case 3: track.type = KEYWORD;    track.kw   = Kw(arr[i+2]); break;
  }
}
// grid-column → {start, end}
auto decode_line = [&](int off) -> Line {
  switch ((int)arr[off]) {
    case 0: return Line::Auto();
    case 1: return Line::Line((int)arr[off+1]);
    case 2: return Line::Span((int)arr[off+1]);
  }
};
```

字符串回退场景（`minmax` / `auto-fill` / `fit-content`）运行时拿到的是 `string`，可选择忽略或用既有 CSS 值解析能力兜底；数值槽场景已经不需要任何字符串处理。
