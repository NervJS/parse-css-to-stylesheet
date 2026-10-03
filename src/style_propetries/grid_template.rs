use swc_core::{common::DUMMY_SP, ecma::ast::*};

use lightningcss::{
  properties::grid::{
    RepeatCount, TrackBreadth, TrackListItem, TrackSize, TrackSizeList, TrackSizing,
  },
  traits::ToCss,
  values::{length::LengthValue, percentage::DimensionPercentage},
};

use crate::{generate_expr_lit_num_grid, generate_expr_lit_str};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

/// grid-template-columns / grid-template-rows / grid-auto-rows / grid-auto-columns
///
/// 输出扁平 DoubleArray，按 type 标签变长消费：
/// - type 0=PERCENTAGE：`[0, value(0..1), 0]`，3 槽
/// - type 1=LENGTH：`[1, value, unit]`，3 槽（unit 见 GridUnit 约定）
/// - type 2=FLEX：`[2, value(fr), 0]`，3 槽
/// - type 3=KEYWORD：`[3, 0, kw]`，3 槽（kw=100 auto / 101 min-content / 102 max-content）
/// - type 4=MINMAX：`[4, <min 3 槽>, <max 3 槽>]`，7 槽（min/max 为 0-3 型 breadth）
/// - type 5=FIT_CONTENT：`[5, <limit 3 槽>]`，4 槽（limit 仅 0/1 型 length-percentage）
/// - type 6=REPEAT_AUTO_FILL / 7=REPEAT_AUTO_FIT：`[6|7, n, <n 条轨道>]`，变长
///   （编译期无法知道容器尺寸故不展开，组内轨道按 type 步进解码；运行时按可用尺寸求次数）
/// - `repeat(n, ...)` 固定次数：编译期展开为扁平轨道
/// - calc() 等无法数值化的结构：整条声明 fallback 为字符串（不影响其他声明）
#[derive(Debug, Clone)]
pub struct GridTemplate {
  pub id: CSSPropertyType,
  pub value: TemplateValue,
}

#[derive(Debug, Clone)]
pub enum TemplateValue {
  /// 扁平轨道数组；按 type 标签变长消费
  Tracks(Vec<f64>),
  /// 降级：calc() 等编译期无法数值化的结构
  Raw(String),
}

// 轨道/线类型 tag（与鸿蒙侧 runtime 约定）
pub const GRID_TRACK_TYPE_PERCENTAGE: f64 = 0.0;
pub const GRID_TRACK_TYPE_LENGTH: f64 = 1.0;
pub const GRID_TRACK_TYPE_FLEX: f64 = 2.0;
pub const GRID_TRACK_TYPE_KEYWORD: f64 = 3.0;
pub const GRID_TRACK_TYPE_MINMAX: f64 = 4.0;
pub const GRID_TRACK_TYPE_FIT_CONTENT: f64 = 5.0;
pub const GRID_TRACK_TYPE_REPEAT_AUTO_FILL: f64 = 6.0;
pub const GRID_TRACK_TYPE_REPEAT_AUTO_FIT: f64 = 7.0;

// 长度单位 + 关键词 code（与鸿蒙侧 runtime 约定）
pub const GRID_UNIT_PX: f64 = 0.0;
pub const GRID_UNIT_VW: f64 = 1.0;
pub const GRID_UNIT_VH: f64 = 2.0;
pub const GRID_UNIT_VMIN: f64 = 3.0;
pub const GRID_UNIT_VMAX: f64 = 4.0;
pub const GRID_UNIT_REM: f64 = 5.0;
pub const GRID_UNIT_EM: f64 = 6.0;
pub const GRID_UNIT_CH: f64 = 7.0;
pub const GRID_UNIT_EX: f64 = 8.0;

pub const GRID_KW_AUTO: f64 = 100.0;
pub const GRID_KW_MIN_CONTENT: f64 = 101.0;
pub const GRID_KW_MAX_CONTENT: f64 = 102.0;

fn breadth_to_slots(breadth: &TrackBreadth) -> Option<Vec<f64>> {
  match breadth {
    TrackBreadth::Length(len) => length_percentage_to_slots(len),
    TrackBreadth::Flex(fr) => Some(vec![GRID_TRACK_TYPE_FLEX, *fr as f64, 0.0]),
    TrackBreadth::Auto => Some(vec![GRID_TRACK_TYPE_KEYWORD, 0.0, GRID_KW_AUTO]),
    TrackBreadth::MinContent => Some(vec![GRID_TRACK_TYPE_KEYWORD, 0.0, GRID_KW_MIN_CONTENT]),
    TrackBreadth::MaxContent => Some(vec![GRID_TRACK_TYPE_KEYWORD, 0.0, GRID_KW_MAX_CONTENT]),
  }
}

fn length_percentage_to_slots(len: &DimensionPercentage<LengthValue>) -> Option<Vec<f64>> {
  match len {
    DimensionPercentage::Percentage(p) => Some(vec![GRID_TRACK_TYPE_PERCENTAGE, p.0 as f64, 0.0]),
    DimensionPercentage::Dimension(length_value) => match length_value {
      // Harmony：px 原生消费逻辑像素
      LengthValue::Px(v) => Some(vec![GRID_TRACK_TYPE_LENGTH, *v as f64, GRID_UNIT_PX]),
      LengthValue::Rem(v) => Some(vec![GRID_TRACK_TYPE_LENGTH, *v as f64, GRID_UNIT_REM]),
      LengthValue::Em(v) => Some(vec![GRID_TRACK_TYPE_LENGTH, *v as f64, GRID_UNIT_EM]),
      LengthValue::Vw(v) => Some(vec![GRID_TRACK_TYPE_LENGTH, *v as f64, GRID_UNIT_VW]),
      LengthValue::Vh(v) => Some(vec![GRID_TRACK_TYPE_LENGTH, *v as f64, GRID_UNIT_VH]),
      LengthValue::Vmin(v) => Some(vec![GRID_TRACK_TYPE_LENGTH, *v as f64, GRID_UNIT_VMIN]),
      LengthValue::Vmax(v) => Some(vec![GRID_TRACK_TYPE_LENGTH, *v as f64, GRID_UNIT_VMAX]),
      LengthValue::Ch(v) => Some(vec![GRID_TRACK_TYPE_LENGTH, *v as f64, GRID_UNIT_CH]),
      LengthValue::Ex(v) => Some(vec![GRID_TRACK_TYPE_LENGTH, *v as f64, GRID_UNIT_EX]),
      _ => None,
    },
    DimensionPercentage::Calc(_) => None,
  }
}

fn track_size_to_slots(size: &TrackSize) -> Option<Vec<f64>> {
  match size {
    TrackSize::TrackBreadth(b) => breadth_to_slots(b),
    // minmax(min, max) → [4, <min 3 槽>, <max 3 槽>]
    TrackSize::MinMax { min, max } => {
      let mut min_slots = breadth_to_slots(min)?;
      let mut max_slots = breadth_to_slots(max)?;
      let mut out = vec![GRID_TRACK_TYPE_MINMAX];
      out.append(&mut min_slots);
      out.append(&mut max_slots);
      Some(out)
    }
    // fit-content(limit) → [5, <limit 3 槽>]（limit 仅 length-percentage）
    TrackSize::FitContent(limit) => {
      let mut limit_slots = length_percentage_to_slots(limit)?;
      let mut out = vec![GRID_TRACK_TYPE_FIT_CONTENT];
      out.append(&mut limit_slots);
      Some(out)
    }
  }
}

fn track_sizing_to_template_value(sizing: &TrackSizing) -> TemplateValue {
  match sizing {
    TrackSizing::None => TemplateValue::Tracks(vec![]),
    TrackSizing::TrackList(list) => {
      let mut slots: Vec<f64> = vec![];
      for item in list.items.iter() {
        match item {
          TrackListItem::TrackSize(size) => {
            if let Some(mut t) = track_size_to_slots(size) {
              slots.append(&mut t);
            } else {
              // 无法数值化：整组降级为 CSS 字符串，交给运行时自行解释或忽略
              return TemplateValue::Raw(serialize_track_sizing(sizing));
            }
          }
          TrackListItem::TrackRepeat(repeat) => match repeat.count {
            RepeatCount::Number(n) if n > 0 => {
              // 展开 repeat 轨道
              for _ in 0..n {
                for size in repeat.track_sizes.iter() {
                  if let Some(mut t) = track_size_to_slots(size) {
                    slots.append(&mut t);
                  } else {
                    return TemplateValue::Raw(serialize_track_sizing(sizing));
                  }
                }
              }
            }
            // auto-fill / auto-fit：编译期无法展开次数，但结构可数值化——
            // 输出 [6|7, 组内轨道数, <组内轨道槽...>]，运行时按可用尺寸求次数
            RepeatCount::AutoFill | RepeatCount::AutoFit => {
              let tag = if matches!(repeat.count, RepeatCount::AutoFill) {
                GRID_TRACK_TYPE_REPEAT_AUTO_FILL
              } else {
                GRID_TRACK_TYPE_REPEAT_AUTO_FIT
              };
              let mut inner: Vec<f64> = vec![];
              for size in repeat.track_sizes.iter() {
                if let Some(mut t) = track_size_to_slots(size) {
                  inner.append(&mut t);
                } else {
                  return TemplateValue::Raw(serialize_track_sizing(sizing));
                }
              }
              slots.push(tag);
              slots.push(repeat.track_sizes.len() as f64);
              slots.append(&mut inner);
            }
            _ => return TemplateValue::Raw(serialize_track_sizing(sizing)),
          },
        }
      }
      TemplateValue::Tracks(slots)
    }
  }
}

/// grid-auto-rows / grid-auto-columns 的 TrackSizeList → 数值槽（与 template 轨道同构）
fn track_size_list_to_template_value(sizes: &TrackSizeList) -> TemplateValue {
  let mut slots: Vec<f64> = vec![];
  for size in sizes.0.iter() {
    if let Some(mut t) = track_size_to_slots(size) {
      slots.append(&mut t);
    } else {
      return TemplateValue::Raw(
        sizes
          .to_css_string(lightningcss::stylesheet::PrinterOptions::default())
          .unwrap_or_default()
          .replace("px", "lpx"),
      );
    }
  }
  TemplateValue::Tracks(slots)
}

fn serialize_track_sizing(sizing: &TrackSizing) -> String {
  sizing
    .to_css_string(lightningcss::stylesheet::PrinterOptions::default())
    .unwrap_or_default()
    .replace("px", "lpx")
}

impl From<(String, &lightningcss::properties::Property<'_>)> for GridTemplate {
  fn from(prop: (String, &lightningcss::properties::Property<'_>)) -> Self {
    let (id, value) = match prop.1 {
      lightningcss::properties::Property::GridTemplateColumns(sizing) => (
        CSSPropertyType::GridTemplateColumns,
        track_sizing_to_template_value(sizing),
      ),
      lightningcss::properties::Property::GridTemplateRows(sizing) => (
        CSSPropertyType::GridTemplateRows,
        track_sizing_to_template_value(sizing),
      ),
      // grid-auto-rows / grid-auto-columns：<track-size> 列表（无 none/repeat 外层结构）
      lightningcss::properties::Property::GridAutoRows(sizes) => (
        CSSPropertyType::GridAutoRows,
        track_size_list_to_template_value(sizes),
      ),
      lightningcss::properties::Property::GridAutoColumns(sizes) => (
        CSSPropertyType::GridAutoColumns,
        track_size_list_to_template_value(sizes),
      ),
      // grid-template 简写：仅在无 areas 时把 rows/columns 合并产出
      // 复合轨道字符串走 Raw 降级（方案 A 未拆分出 rows/columns 两组槽位）
      lightningcss::properties::Property::GridTemplate(styles) => (
        CSSPropertyType::GridTemplateColumns,
        TemplateValue::Raw(format!(
          "{} / {}",
          serialize_track_sizing(&styles.rows),
          serialize_track_sizing(&styles.columns)
        )),
      ),
      _ => (CSSPropertyType::Invalid, TemplateValue::Tracks(vec![])),
    };
    GridTemplate { id, value }
  }
}

impl ToExpr for GridTemplate {
  fn to_expr(&self) -> PropertyTuple {
    match &self.value {
      TemplateValue::Tracks(slots) => {
        let elems: Vec<Option<ExprOrSpread>> = slots
          .iter()
          .map(|v| {
            Some(ExprOrSpread {
              spread: None,
              expr: Box::new(generate_expr_lit_num_grid!(*v)),
            })
          })
          .collect();
        PropertyTuple::One(
          self.id,
          Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems,
          }),
        )
      }
      TemplateValue::Raw(raw) => PropertyTuple::One(self.id, generate_expr_lit_str!(raw.clone())),
    }
  }
}
