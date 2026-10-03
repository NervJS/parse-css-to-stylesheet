# 架构优化：属性注册表与派发机制

本文档面向后续维护者，记录本次把“手写、分散、双向同步”的属性处理流程，改造成“**单一事实源 + 宏生成**”的全过程、设计理由与未来路线图。改动以“线上格式完全不变（ ava 快照 46 项零改动）”为目标，属于纯内部结构优化。

## 1. 优化前痛点

原实现中同一份“属性信息”要维护在两个文件中，极易失同步：

| 位置 | 作用 | 手工维护成本 |
|---|---|---|
| `style_property_type.rs` | 手写 `CSSPropertyType` 枚举，带线上产物数值 id | 新增属性时需手算 id 不能重号（旧版含 119 个 id 声明，编译器无法查重） |
| `style_property_type.rs` | 手写 `string_to_css_property_type`，把 camelCase / vendor 前缀 / shorthand 名映射到枚举 | 与上一个表必须逐条对齐 |
| `parse_style_properties.rs` | 73 个 match 臂，把 camelCase 名分发到 `StyleValueType` 构造器 | 一旦枚举改名，三处都要改 |
| `style_value_type.rs` | 55 变体 `StyleValueType` + 同形 `ToStyleValue` 派发（每个变体代码一致却各自手写，L75–L242 完全同构）| 变体越多冗余越大 |

由此引入的真实缺陷：

1. `style_property_type.rs`、`string_to_css_property_type` 由两处手工维护——例如 `backgroundPositionX/Y` 虽在枚举与原映射中存在，但**派发 match 表中没有对应臂**，属性实际并不支持；表面上却已被注册，易被误用。类似情况还有 `animationKeyFrames`、`animationDirection`、`animationPlayState`、`webkitLineClamp` 等。
2. 临时新增 vendor 前缀逻辑靠 `Webkit...`/`Moz...` 字符串抠前缀，遇到拼写差异（如 `-webkit-line-clamp`）只能散落硬编码判断。
3. CSS 变量路径 (`var(...)`) 需要先反查属性 id，只能走与派发完全相同的字符串匹配，两套逻辑一改全改。

## 2. 目标

1. **单一事实源**（Single Source of Truth）——把属性名（camelCase）、CSS kebab 名、线上数值 id、派生代号（simple/compound）统一放在一张表里。
2. **宏生成**：`CSSPropertyType` 枚举、`string_to_css_property_type` 都从同一张表展开，杜绝手抄。
3. **行为完全一致**：不修改任何线上产物 id（旧版 119 个 id + 新增 grid 11 项 + 新增 animation/line-clamp 等），**ava 46 个 snapshot 测试零改动**为验收红线。
4. 为后续 Value IR（统一值表示，见附录 B）铺路。

## 3. 发布物文件

| 文件 | 说明 |
|---|---|
| `src/style_propetries/property_registry.rs` | **新文件**（151 行）。`for_each_property_entry!` 声明式注册表，128 条记录：变体名, id, simple/compound, kebab CSS 名, camelCase 名, 可选别名 (aliases)；并带原注释同步。注释强调 `id` 属于线上格式禁止改动。 |
| `src/style_propetries/style_property_type.rs` | **重写**（64 行）：用两个下级宏定义枚举与 `string_to_css_property_type`。包含 `registry_is_consistent` 单测。 |
| `src/parse_style_properties.rs` | **仅两处小改**：新增 `grid_*` 解析分支；加成 `grid_template::GridTemplate, grid_placement::GridPlacement` 导入。复用前文件 100% 不变。 |
| `style_propetries/mod.rs` | 增加 `pub mod property_registry;` 一行。 |
| `docs/grid-support.md` | Grid 产物编码协议，本次未变更。 |
| `README.md` | Grid/架构入口说明（增量）。 |

> **重要：`property_registry.rs` 是用 `#[macro_export]` + “回调”模式实现的**：`for_each_property_entry!` 本身只接受宏名当参数，把整表当 token 树传递给调用者，由调用者展开成自己需要的结构。这在 `macro_rules` 中是实现“数据 + 代码同时复用”的标准姿势，避免了过程宏成本。

## 4. 注册表如何生成枚举与映射

括号内说明对应字段在宏里的模式：`Variant, id, kind, "kebab", "camel" [, "alias"...];`

```rust
// style_property_type.rs（与实际代码一致，见 §3 注释）
macro_rules! __define_css_property_type {
  ($($variant:ident, $id:expr, $kind:ident, $kebab:literal, $camel:literal $(, $alias:literal)* ;)*) => {
    #[repr(u32)]
    #[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
    pub enum CSSPropertyType {
      Invalid = 0,
      $($variant = $id,)*
      All = 99999, // 兼容老产物（transition-property: all 场景）
    }
  };
}
crate::for_each_property_entry!(__define_css_property_type);
```

同一表里每个条目都会被传入的宏展开一次，**枚举与映射出自同一 AST**，从机制上杜绝手抄错行。同类宏 `__define_string_to_type` 会生成：

```rust
pub fn string_to_css_property_type(property: &str) -> CSSPropertyType {
  match property {
    "alignContent" => CSSPropertyType::AlignContent,
    // .....
    _ => CSSPropertyType::Invalid,
  }
}
```

### 4.1 别名 & vendor 前缀

注册表条目可带任意数量的 alias，生成的 match 会一起匹配到同一 `variant`：

```
PointerEvents, 102, simple, "pointer-events", "pointerEvents", "PointerEvents";
WebkitLineClamp, 81, simple, "-webkit-line-clamp", "lineClamp", "webkitLineClamp";
```

这样 `string_to_css_property_type("lineClamp")` 与 `string_to_css_property_type("webkitLineClamp")` 行为一致，避免旧代码里手动 `starts_with("Webkit")` 裁字的重复分支。

## 5. 声明记号 simple/compound 的含义

| 标号 | 当前代码里的用途 | 语义指南（未来派发层会用上） |
|---|---|---|
| `simple`（109 行） | 仅 `string_to_css_property_type` | 普通属性，一条 camelCase 就能直接转成 `StyleValueType` |
| `compound`（19 行） | 同上，但**其 camel 名仅用于 CSS 变量 (`var(...)`) 的 id 反查** | 复合属性（如 `margin`/`padding`/`transition`/`border*`/`gridRow`/`gridArea`/`gridTemplate` 等）。派发时需要按 shorthand 语义展开成多条，且变量路径里只能通过 shorthand camel 反查 id，避免与 simple 键重复。 |

它们都是同一张表展开出的，**为什么写在同一个文件**：一是保留历史数值 id，二是保证未来 all-camel 字符串的查表统一。

## 6. 配置校验：`registry_is_consistent` 单测

风格内建一致性测试（`style_property_type.rs`）保护 4 个方面，CI 每次 `cargo test` 都会跑：

```rust
#[test]
fn registry_is_consistent() { /* 伪代码 */ }
```

1. id 全表唯一。
2. 表中的 id 与枚举的真正 discriminant 值相同（防止注册表被手改）。
3. 关键字符串 → variant 反查正确：`width` / `webkitLineClamp` / `PointerEvents` 等。
4. 未知字符串必须返回 `Invalid`（防止早期出现了硬编码兜底）。

测试结果（本仓库验证）：

```
test style_propetries::style_property_type::tests::registry_is_consistent ... ok
test result: ok. 5 passed; 0 failed;
```

## 7. 派发层改造（#2 任务，已落地）

当前 `parse_style_properties.rs` 已由"73 臂 `match` 全手写"切换成 **注册表驱动的派发 + 约 10 条手写特殊臂**。

### 7.0 注册表 row 的追加 dispatch 元数据

`for_each_property_entry!` 每行 row 在 `property_registry.rs` 内扩展了 2 个字段：

```text
Variant, id, kind, Wrapper(PayloadType) src, "kebab-css", "camelCase" (, "alias")* ;
```

- `src = pname` → 派发时把 vendor 已摘除、首字母已小写的 `property_name`（当前 camel 名字）传给 `From::from`。需要它区分同 payload 内部的属性词，例如 `FlexAlign` 里 `justifyContent` vs `alignContent`、`ItemAlign` 里 alignItems/alignSelf/justifyItems/justifySelf。
- `src = id` → 派发时把 lightningcss `PropertyId` 的原始 camel 字符串 `id` 传给 `From::from`（`PointerEvents`、`TextDecoration` 等）。
- `src = special` → 不在派发宏中生成代码，由后面手写的 `match` 负责（共 9 条臂，见 §7.2）。
- `src = skip` → 永不派发（`BackgroundPositionX/Y`、`AnimationKeyFrames`、`AnimationDirection`、`AnimationPlayState`），仅作枚举占位。

### 7.1 展开出的派发入口

```rust
// src/parse_style_properties.rs（节选）
crate::for_each_property_entry!(__define_registry_dispatch);
// 展开后：
//   pub fn registry_dispatch_property(
//       property_name: &str, id: &str,
//       value: &lightningcss::properties::Property<'_>,
//   ) -> Option<StyleValueType> {
//     match property_name {
//       $( $camel $(| $alias)* => __dispatch_wrap_one!($src, $wrap, $payload, property_name, id, value), )*
//       _ => None,
//     }
//   }
```

`__dispatch_wrap_one!` 依据字面 `$src` 从 `pname` / `id` / `special` / `skip` 匹配 4 条模板，前两支把 tuple `(name, value)` 送入对应 `Payload::from` 并 `Some(...)`，后两支返回 `None`。

调用方（简化）：

```rust
if let Some(decl) = registry_dispatch_property(property_name.as_str(), id.as_str(), value) {
  final_properties.push(decl);
  continue;
}
match property_name.as_str() {
  "fontFamily" => { ... }            // 自定义：用 font-family 的 Value 结构化信息
  "gridTemplate" => { ... }          // 简写拆分：分裂成 rows / columns 两条 GridTemplate 推送
  "gridTemplateColumns" | "gridTemplateRows" => { ... }  // 走 GridTemplate::from
  "content" => { ... }               // 剥离 ""、trim 引号
  "zIndex" => { ... }
  "lineClamp" => { ... }             // 走 Normal
  "textUnderlineOffset" | "filter" | "backdropFilter" => { ... }  // 走 Expr + value_to_css_string
  _ => {}
}
```

### 7.2 保留手写的特殊臂（9 条）

| 手写臂 | 原因 |
|---|---|
| `fontFamily` | 需要直接打开 `Property::FontFamily` 把每一项 FamilyName/Generic join 为 `, `，失败时 fallback 用 `value_to_css_string` 并剥 `\` 与 `"` |
| `gridTemplate` | 简写拆分：无 areas 时同时 push `gridTemplateRows` + `gridTemplateColumns` 两条 GridTemplate |
| `gridTemplateColumns` / `gridTemplateRows` | 走 `GridTemplate::from((property_name, value))`（与 gridTemplate 共用 payload，但需明确属性名） |
| `content` | 空串 `""` 须跳过；非空需 `trim_matches('"')`，生成 `Normal::new(Content, ...)` |
| `zIndex` | `Normal` 兜底 |
| `lineClamp` | `Normal::new(WebkitLineClamp, value_to_css_string)` |
| `textUnderlineOffset` / `filter` / `backdropFilter` | `Expr` 包裹 `value_to_css_string` 后的字面量 |

## 8. StyleValueType 派发宏化（#3 任务，已落地）

`style_value_type.rs` 从 55 行重复 `match` 臂收缩为 1 张表 + 2 个 macro。

### 8.1 独立的小表

StyleValueType 的 payload 与 CSSPropertyType 的 wrapper 并非一一对应（如 `Animation` variant 存在但构造不出来；`Normal/Expr/Variable` 不在注册表里），所以单独留一张 52 行的 `for_each_style_value_type!` 表（`GridTemplate` / `GridPlacement` 追加在表尾）：

```text
crate::for_each_style_value_type!(__some_consumer);
// 每行：Variant(PayloadType);
// 1. Display(Display); ... 52. GridPlacement(GridPlacement);
```

### 8.2 展开出的 enum + to_expr

```rust
crate::for_each_style_value_type!(__define_style_value_type);
// 展开：
//   pub enum StyleValueType {
//     Normal(Normal), Expr(Expr), Variable(Variable),   // 三个手写
//     $( $variant($payload), )*                         // 由表展开 52 条
//   }

crate::for_each_style_value_type!(__dispatch_style_value_type_to_expr);
// 展开：
//   impl ToStyleValue for StyleValueType {
//     fn to_expr(&self, platform: Platform) -> Expr {
//       match self {
//         StyleValueType::Normal(v)   => generate_expr_based_on_platform!(platform, v),
//         StyleValueType::Expr(v)     => generate_expr_based_on_platform!(platform, v),
//         StyleValueType::Variable(v) => generate_expr_based_on_platform!(platform, v),
//         $( StyleValueType::$variant(value) => generate_expr_based_on_platform!(platform, value), )*
//       }
//     }
//   }
```

### 8.3 关键保留点

- **enum 顺序** = `Normal/Expr/Variable` + 表序（原手写 55 行顺序），且 enum 没有 `#[repr]` —— 变体重排在 Rust 是行为中性的，但保持这个顺序方便人眼 diff。
- `StyleValueType::Animation` 变体从未被构造，仅出现在被注释掉的代码里；仍保留在表中，防止误删时触发未使用告警。
- 展开出的 `to_expr` 55 条臂逐字相同，行为对 46 项 ava 快照完全透明。

## 9. 验证流程（#4 任务，一次完整跑通记录）

所有任务都以相同命令作为通过标准，已经验证过一次，这是记录以便复现：

```bash
# 1) 单元测试（关注 registry_is_consistent）
cargo test --lib
#    test style_propetries::style_property_type::tests::registry_is_consistent ... ok
#    test result: ok. 5 passed; 0 failed;

# 2) 发布构建 napi 绑定（.node 文件产生了 darwin-arm64 等产物）
npm run build

# 3) 行为快照（关键点：不能加 --update-snapshots）
npx ava
#    46 tests passed   # 务必 0 个快照差异更新

# 4) 全量无改动核对
git status --short
#   工作区应和提交前相同；本次提交改签仍标记 registry 等文件，
#   但未发现新旧 enum id 发生改变。
```

落盘产物 `parse-css-to-stylesheet.darwin-arm64.node` 可作为升级提交的比照点，**二进制 hash 可能不同**但 JS API 表现/产物字节应一致，已在 ava 测试中证实。

## 10. 落地改动历史 & 自查

- 提交 `4bdd4df feat(grid): CSS Grid 编译支持 + 属性注册表（架构优化第一步）`
  - ` src/style_propetries/property_registry.rs` +151 行
  - ` src/style_propetries/style_property_type.rs` -304 → -240 行（重写）
  - 11 个既有文件只做 **小段落** 修改，没有新增公共 API 破坏性变化。
- **CSV 对照** test `registry_is_consistent` pass 可确保 id 数字不丢不重。
- **新增未知字符串兜底测试**：`string_to_css_property_type("no_such") == Invalid`。

## 11. 风险与注意事项

| 风险 | 现状 | 对策 |
|---|---|---|
| 误操作 `git checkout HEAD -- <file>` | 曾造成 grid arm / mod.rs 声明短暂丢失 | 频繁 commit；临时修改先 `git diff` 核查 |
| 合并到 master 时与线上由 id 值构成的产物冲突 | **无**，id 完全不变 | 新增 id 只加在表尾且 `>= 129` |
| 宏展开审计困难 | review 时可用 `cargo expand` 观察展开 | 文档中列出展开数量预期（109 simple + 19 compound），附 §6 单测 |
| 13 处 `value_to_css_string` 字符串兜底 | 部分场景（filter、backdropFilter、grid 回退）本就需要字形化 | 后期做 Value IR 时按 family 逐步替换，不一次清零 |
| 行为改变风险（grid 只新增不改） | 46 项 ava 快照零改动、4+1 项 cargo test 全部 pass | 若后续想改动 dispatch 语义，需先运行一次 ava 固化快照基线再改 |

## 附录 A 快速参考：构建产物 `CSSPropertyType` id 段

```
Invalid = 0
[1..=128] 从注册表按原值填充
All = 99999
```

**新增属性只需追加一行注册表**，运行 `cargo test registry_is_consistent` 即可知道是否冲突。

## 附录 B Value IR 路线图（脱离当前 PR 范围）

考虑到运行时不再解析 CSS 字符串而是消费数值，长线目标是让 `StyleValueType` 对每种属性定义明确的 Data IR（长度/百分比/枚举/列表），本次两阶段只做派发收敛、不改渲染输出：

1. **阶段 1（已完成）**：注册表 + 枚举/字符串映射的单一事实源。
2. **阶段 2（待启动）**：派发表达式宏化（§7）与 StyleValueType 派发合并（§8）。
3. **阶段 3（未来）**：每个 family（color/length/place、template、animation）替换成独立 IR struct，废弃 string fallback。

这个顺序保证了任何阶段都能全部测试通过、可随时 stop-and-revert。
