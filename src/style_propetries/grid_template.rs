use swc_core::{common::DUMMY_SP, ecma::ast::*};

use lightningcss::{
  properties::grid::{
    RepeatCount, TrackBreadth, TrackListItem, TrackSize, TrackSizing,
  },
  traits::ToCss,
  values::{length::LengthValue, percentage::DimensionPercentage},
};

use crate::{generate_expr_lit_num_grid, generate_expr_lit_str};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

/// grid-template-columns / grid-template-rows
///
/// 输出扁平 DoubleArray：每 3 个槽位一条轨道 `[type, value, unit]`。
/// - type: 0=PERCENTAGE（value 为 0..1）, 1=LENGTH（unit 见 GridUnit 约定）,
///   2=FLEX（value=fr）, 3=KEYWORD（unit=UnitCode of auto/min-content/…）
/// - `repeat(n, ...)` 展开为扁平轨道；`minmax()`/`fit-content()`/`auto-fill/fit`
///   不做数值化，fallback 为字符串（不影响运行时消费其他声明）
#[derive(Debug, Clone)]
pub struct GridTemplate {
  pub id: CSSPropertyType,
  pub value: TemplateValue,
}

#[derive(Debug, Clone)]
pub enum TemplateValue {
  /// 扁平轨道数组；每 3 个槽位 = 1 条轨道
  Tracks(Vec<f64>),
  /// 降级：repeat(auto-fill/...) 等非确定性结构
  Raw(String),
}

// 轨道/线类型 tag（与鸿蒙侧 runtime 约定）
pub const GRID_TRACK_TYPE_PERCENTAGE: f64 = 0.0;
pub const GRID_TRACK_TYPE_LENGTH: f64 = 1.0;
pub const GRID_TRACK_TYPE_FLEX: f64 = 2.0;
pub const GRID_TRACK_TYPE_KEYWORD: f64 = 3.0;

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
    _ => None,
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
            // auto-fill / auto-fit：非确定性结构，降级字符串
            _ => return TemplateValue::Raw(serialize_track_sizing(sizing)),
          },
        }
      }
      TemplateValue::Tracks(slots)
    }
  }
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
