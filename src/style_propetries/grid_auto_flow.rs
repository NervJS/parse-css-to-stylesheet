use swc_core::{common::DUMMY_SP, ecma::ast::*};

use lightningcss::properties::Property;

use crate::generate_expr_lit_num_grid;

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

/// grid-auto-flow
///
/// lightningcss 的 GridAutoFlow 是 bitflags（Row=0b00, Column=0b01, Dense=0b10），
/// 其位组合与鸿蒙侧 runtime 的 Taro_GridAutoFlow 枚举数值一一对应：
///   0 = row（默认）, 1 = column, 2 = row dense, 3 = column dense
/// 直接输出 bits 整数值，运行时零转换消费。
#[derive(Debug, Clone)]
pub struct GridAutoFlow {
  pub id: CSSPropertyType,
  pub bits: Option<u8>,
}

impl From<(String, &Property<'_>)> for GridAutoFlow {
  fn from(prop: (String, &Property<'_>)) -> Self {
    match prop.1 {
      Property::GridAutoFlow(flow) => GridAutoFlow {
        id: CSSPropertyType::GridAutoFlow,
        bits: Some(flow.bits()),
      },
      _ => GridAutoFlow {
        id: CSSPropertyType::Invalid,
        bits: None,
      },
    }
  }
}

impl ToExpr for GridAutoFlow {
  fn to_expr(&self) -> PropertyTuple {
    match self.bits {
      Some(bits) => PropertyTuple::One(self.id, generate_expr_lit_num_grid!(bits)),
      None => PropertyTuple::One(
        CSSPropertyType::Invalid,
        Expr::Invalid(Invalid { span: DUMMY_SP }),
      ),
    }
  }
}
