use std::borrow::Borrow;

use lightningcss::media_query::*;
use lightningcss::media_query::{self, MediaCondition, MediaQuery};
use lightningcss::printer::PrinterOptions;
use lightningcss::traits::ToCss;
use lightningcss::values::length::{Length, LengthValue};
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::BigInt;
use swc_core::ecma::ast::*;

use crate::{
  generate_expr_by_length, generate_expr_enum, generate_expr_lit_num, generate_expr_lit_str,
};

#[derive(Debug, Clone)]
pub enum StyleMediaFeatureID {
  Invalid = 0,
  Width = 1,
  Height = 2,
  AspectRatio = 3,
  DisplayMode = 4,
  Orientation = 5,
  DeviceWidth = 6,
  DeviceHeight = 7,
  DeviceAspectRatio = 8,
  Resolution = 9,
  PrefersColorScheme = 10,
}

#[derive(Debug, Clone)]
pub enum MediaValueType {
  Length(Length),
  Float(f64),
  Number(i64),
  String(String),
}

#[derive(Debug, Clone)]
pub enum StyleMediaOpType {
  /// `=`
  Equal = 0,
  /// `>`
  GreaterThan = 1,
  /// `>=`
  GreaterThanEqual = 2,
  /// `<`
  LessThan = 3,
  /// `<=`
  LessThanEqual = 4,
}

#[derive(Debug, Clone)]
pub enum StyleMediaCondType {
  /// 'none'
  None = 0,
  /// 'not'
  NOT = 1,
  /// 'and'
  AND = 2,
  /// 'or'
  OR = 3,
}

#[derive(Debug, Clone)]
pub struct StyleMediaFeature {
  pub feature_id: StyleMediaFeatureID,
  pub op: StyleMediaOpType,
  pub value: Option<MediaValueType>,
}

#[derive(Debug, Clone)]
pub enum StyleMediaCondition {
  //not/and/or/feature
  Feature(StyleMediaFeature),
  Operation {
    operation: StyleMediaCondType,
    conditions: Vec<StyleMediaCondition>,
  },
}

#[derive(Debug, Clone)]
pub struct StyleMedia {
  pub media_id: u32,
  pub conditions: Vec<StyleMediaCondition>,
}

impl StyleMedia {
  pub fn new(media_id: u32, conditions: Vec<StyleMediaCondition>) -> Self {
    StyleMedia {
      media_id,
      conditions,
    }
  }

  pub fn parse(&mut self, medias: &Vec<MediaQuery>) {
    for media_query in medias {
      if let Some(conditon) = &media_query.condition {
        let ret_condition = self.parse_condition(conditon);
        if ret_condition.is_some() {
          self.conditions.push(ret_condition.unwrap());
        }
      }
    }
  }

  pub fn to_expr(self) -> Vec<PropOrSpread> {
    return vec![
      PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("id".into(), DUMMY_SP)),
        value: Box::new(generate_expr_lit_num!(self.media_id as f64)),
      }))),
      PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("conditions".into(), DUMMY_SP)),
        value: Box::new(Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: self
            .conditions
            .iter()
            .map(|condition| self.condtition_to_expr(condition))
            .collect(),
        })),
      }))),
    ];
  }

  fn value_to_expr(&self, value: Option<MediaValueType>) -> Option<ExprOrSpread> {
    if let Some(tmp_val) = value {
      match tmp_val {
        MediaValueType::Length(val) => {
          return Some(ExprOrSpread {
            spread: None,
            expr: Box::new(generate_expr_by_length!(val, Platform::Harmony)),
          });
        }
        MediaValueType::Float(val) => {
          return Some(ExprOrSpread {
            spread: None,
            expr: Box::new(generate_expr_lit_num!(val)),
          });
        }
        MediaValueType::Number(val) => {
          return Some(ExprOrSpread {
            spread: None,
            expr: Box::new(generate_expr_lit_num!(val as f64)),
          });
        }
        MediaValueType::String(val) => {
          return Some(ExprOrSpread {
            spread: None,
            expr: Box::new(generate_expr_lit_str!(val)),
          });
        }
      }
    }
    return None;
  }

  fn condtition_to_expr(&self, condtion: &StyleMediaCondition) -> Option<ExprOrSpread> {
    match condtion {
      StyleMediaCondition::Feature(feature) => {
        return Some(ExprOrSpread {
          spread: None,
          expr: Box::new(Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: vec![
              Some(ExprOrSpread {
                spread: None,
                expr: Box::new(generate_expr_enum!(StyleMediaCondType::None)),
              }),
              Some(ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Array(ArrayLit {
                  span: DUMMY_SP,
                  elems: vec![
                    Some(ExprOrSpread {
                      spread: None,
                      expr: Box::new(generate_expr_enum!(feature.feature_id.clone())),
                    }),
                    Some(ExprOrSpread {
                      spread: None,
                      expr: Box::new(generate_expr_enum!(feature.op.clone())),
                    }),
                    self.value_to_expr(feature.value.clone()),
                  ],
                })),
              }),
            ],
          })),
        });
      }
      StyleMediaCondition::Operation {
        operation,
        conditions,
      } => {
        let condition_arr: Vec<Option<ExprOrSpread>> = conditions
          .iter()
          .map(|elem| self.condtition_to_expr(elem))
          .collect();
        return Some(ExprOrSpread {
          spread: None,
          expr: Box::new(Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: vec![
              Some(ExprOrSpread {
                spread: None,
                expr: Box::new(generate_expr_enum!(operation.clone())),
              }),
              Some(ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Array(ArrayLit {
                  span: DUMMY_SP,
                  elems: condition_arr,
                })),
              }),
            ],
          })),
        });
      }
    }
  }

  fn parse_condition(&mut self, condition: &MediaCondition) -> Option<StyleMediaCondition> {
    match condition {
      MediaCondition::Feature(feature) => match feature {
        QueryFeature::Range {
          name,
          operator,
          value,
        } => {
          let op = self.parse_compare(operator);
          let tmp_feature = self.parse_feature(name, op, Some(value));
          return match tmp_feature {
            Some(ret_feature) => Some(StyleMediaCondition::Feature(ret_feature)),
            None => None,
          };
        }
        QueryFeature::Plain { name, value } => {
          return match self.parse_feature(name, StyleMediaOpType::Equal, Some(value)) {
            Some(ret_feature) => Some(StyleMediaCondition::Feature(ret_feature)),
            None => None,
          }
        }
        QueryFeature::Boolean { name } => {
          return match self.parse_feature(name, StyleMediaOpType::Equal, None) {
            Some(ret_feature) => Some(StyleMediaCondition::Feature(ret_feature)),
            None => None,
          }
        }
        QueryFeature::Interval {
          name,
          start,
          start_operator,
          end,
          end_operator,
        } => {
          let start_op = self.parse_compare(start_operator);
          let end_op = self.parse_compare(end_operator);
          return Some(StyleMediaCondition::Operation {
            operation: StyleMediaCondType::AND,
            conditions: vec![
              self.parse_feature(name, start_op, Some(start)),
              self.parse_feature(name, end_op, Some(end)),
            ]
            .into_iter()
            .filter_map(|x| x.map(|y| StyleMediaCondition::Feature(y)))
            .collect(),
          });
        }
      },
      MediaCondition::Not(not) => {
        let ret_condition = self.parse_condition(not);
        match ret_condition {
          Some(ret_some) => {
            return Some(StyleMediaCondition::Operation {
              operation: StyleMediaCondType::NOT,
              conditions: vec![ret_some],
            });
          }
          None => {
            return None;
          }
        }
      }
      MediaCondition::Operation {
        operator,
        conditions,
      } => {
        let op = match operator {
          Operator::And => StyleMediaCondType::AND,
          Operator::Or => StyleMediaCondType::OR,
        };

        let mut ret_conditions = vec![];
        conditions.iter().for_each(|condition| {
          let ret_condition = self.parse_condition(condition);
          if ret_condition.is_some() {
            ret_conditions.push(ret_condition.unwrap());
          }
        });
        return Some(StyleMediaCondition::Operation {
          operation: op,
          conditions: ret_conditions,
        });
      }
    }
  }

  fn parse_feature(
    &mut self,
    feature_id: &MediaFeatureName<MediaFeatureId>,
    feature_op: StyleMediaOpType,
    feature_val: Option<&MediaFeatureValue>,
  ) -> Option<StyleMediaFeature> {
    let id = self.convert_featureid(feature_id);
    return match feature_val {
      Some(value) => Some(StyleMediaFeature {
        feature_id: id,
        op: feature_op,
        value: self.parse_value(value),
      }),
      None => Some(StyleMediaFeature {
        feature_id: id,
        op: feature_op,
        value: None,
      }),
    };
  }

  fn parse_compare(&mut self, operator: &MediaFeatureComparison) -> StyleMediaOpType {
    match operator {
      MediaFeatureComparison::Equal => StyleMediaOpType::Equal,
      MediaFeatureComparison::GreaterThan => StyleMediaOpType::GreaterThan,
      MediaFeatureComparison::GreaterThanEqual => StyleMediaOpType::GreaterThanEqual,
      MediaFeatureComparison::LessThan => StyleMediaOpType::LessThan,
      MediaFeatureComparison::LessThanEqual => StyleMediaOpType::LessThanEqual,
    }
  }

  fn parse_value(&mut self, feature_val: &MediaFeatureValue) -> Option<MediaValueType> {
    match feature_val {
      MediaFeatureValue::Length(length_value) => {
        return Some(MediaValueType::Length(length_value.clone()));
      }
      MediaFeatureValue::Number(value) => {
        return Some(MediaValueType::Float(*value as f64));
      }
      MediaFeatureValue::Integer(value) => {
        return Some(MediaValueType::Number(*value as i64));
      }
      MediaFeatureValue::Boolean(value) => {
        return Some(MediaValueType::Number(*value as i64));
      }
      MediaFeatureValue::Resolution(value) => {
        let num = match value {
          lightningcss::values::resolution::Resolution::Dpi(val) => *val,
          lightningcss::values::resolution::Resolution::Dpcm(val) => *val * 96.0 / 37.7953,
          lightningcss::values::resolution::Resolution::Dppx(val) => *val * 96.0,
        };
        return Some(MediaValueType::Float(num as f64));
      }
      MediaFeatureValue::Ratio(value) => {
        return Some(MediaValueType::Float((value.0 / value.1) as f64));
      }
      MediaFeatureValue::Ident(value) => {
        return Some(MediaValueType::String(value.to_ascii_lowercase()));
      }
      MediaFeatureValue::Env(_) => return None,
    }
  }

  fn convert_featureid(
    &mut self,
    feature_id: &MediaFeatureName<MediaFeatureId>,
  ) -> StyleMediaFeatureID {
    match feature_id {
      MediaFeatureName::Standard(stand_id) => match stand_id {
        MediaFeatureId::Width => return StyleMediaFeatureID::Width,
        MediaFeatureId::Height => return StyleMediaFeatureID::Height,
        MediaFeatureId::AspectRatio => return StyleMediaFeatureID::AspectRatio,
        MediaFeatureId::DisplayMode => return StyleMediaFeatureID::DisplayMode,
        MediaFeatureId::Orientation => return StyleMediaFeatureID::Orientation,
        MediaFeatureId::DeviceWidth => return StyleMediaFeatureID::DeviceWidth,
        MediaFeatureId::DeviceHeight => return StyleMediaFeatureID::DeviceHeight,
        MediaFeatureId::DeviceAspectRatio => return StyleMediaFeatureID::DeviceAspectRatio,
        MediaFeatureId::Resolution => return StyleMediaFeatureID::Resolution,
        MediaFeatureId::PrefersColorScheme => return StyleMediaFeatureID::PrefersColorScheme,
        _ => return StyleMediaFeatureID::Invalid,
      },
      _ => return StyleMediaFeatureID::Invalid,
    }
  }
}
