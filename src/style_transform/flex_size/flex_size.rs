use lightningcss::{
  properties::{flex::Flex, Property},
  stylesheet::PrinterOptions,
  traits::ToCss,
  values::{length::LengthPercentageOrAuto, percentage::DimensionPercentage},
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, ObjectLit, KeyValueProp, PropOrSpread, PropName, Prop, Ident};

use crate::style_transform::traits::ToExpr;

use super::{flex_basis::FlexBasis, flex_grow::FlexGrow, flex_shrink::FlexShrink};

fn parse_flex_size(flex: &Flex) -> FlexSize {
  let mut flex_size = FlexSize {
    grow: None,
    shrink: None,
    basis: None,
  };
  flex_size.grow = Some(FlexGrow(flex.grow));
  flex_size.shrink = Some(FlexShrink(flex.shrink));
  match &flex.basis {
    LengthPercentageOrAuto::Auto => {
      flex_size.basis = Some(FlexBasis::String("auto".to_string()));
    }
    LengthPercentageOrAuto::LengthPercentage(value) => match value {
      DimensionPercentage::Dimension(value) => {
        flex_size.basis = Some(FlexBasis::String(value.to_css_string(PrinterOptions::default()).unwrap()));
      }
      DimensionPercentage::Percentage(value) => {
        flex_size.basis = Some(FlexBasis::String(
          value.to_css_string(PrinterOptions::default()).unwrap(),
        ));
      }
      _ => {
        flex_size.basis = Some(FlexBasis::String("auto".to_string()));
      }
    },
  }
  flex_size
}

#[derive(Debug, Clone)]
pub struct FlexSize {
  pub grow: Option<FlexGrow>,
  pub shrink: Option<FlexShrink>,
  pub basis: Option<FlexBasis>,
}

impl FlexSize {
  pub fn new() -> Self {
    FlexSize {
      grow: None,
      shrink: None,
      basis: None,
    }
  }
}

impl ToExpr for FlexSize {
  fn to_expr(&self) -> Expr {
    let mut arr = vec![];
    
    if let Some(basis) = &self.basis {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("basis".into(), DUMMY_SP)),
        value: basis.to_expr().into(),
      }))))
    }
    if let Some(shrink) = &self.shrink {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("shrink".into(), DUMMY_SP)),
        value: shrink.to_expr().into(),
      }))))
    }
    if let Some(grow) = &self.grow {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("grow".into(), DUMMY_SP)),
        value: grow.to_expr().into(),
      }))))
    }

    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: arr.into(),
    })

   
  }
}


impl From<&Property<'_>> for FlexSize {
  fn from(value: &Property<'_>) -> Self {
    match value {
      Property::Flex(flex, _) => parse_flex_size(&flex),
      _ => FlexSize::new(),
    }
  }
}

