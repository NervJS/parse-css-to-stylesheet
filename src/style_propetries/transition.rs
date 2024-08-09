use std::{cell::RefCell, collections::HashMap, rc::Rc};

use indexmap::Equivalent;
use lightningcss::{
  printer::PrinterOptions,
  properties::{transition, Property, PropertyId},
  traits::ToCss,
  values::{easing::EasingFunction, time},
};

use super::{
  style_property_type::CSSPropertyType,
  style_value_type::StyleValueType,
  traits::ToExpr,
  unit::{Platform, PropertyTuple},
};
use crate::{
  generate_expr_enum, generate_expr_lit_num, generate_expr_lit_str,
  style_propetries::style_property_enum, visitor::parse_style_values,
};
use swc_core::{common::DUMMY_SP, ecma::ast::*};

#[derive(Debug, Clone)]
pub enum TransitionTimingFunction {
  TransitionnCurve(style_property_enum::ArkUI_AnimationCurve),
  EasingFunction(EasingFunction),
}

#[derive(Debug, Clone)]
pub struct Transition {
  //pub transitions: Vec<SingleTransition>,
  pub transtition_properties: Vec<i32>,
  pub transition_durations: Vec<f32>,
  pub transition_delays: Vec<f32>,
  pub transition_timeing_functions: Vec<TransitionTimingFunction>,
}

pub fn convert_property_to_cssty(property_id: &PropertyId) -> CSSPropertyType {
  let mut ret_type: CSSPropertyType = CSSPropertyType::Invalid;
  ret_type = match property_id {
    PropertyId::Position => CSSPropertyType::Position,
    PropertyId::Left => CSSPropertyType::Left,
    PropertyId::Right => CSSPropertyType::Right,
    PropertyId::Top => CSSPropertyType::Top,
    PropertyId::Bottom => CSSPropertyType::Bottom,

    PropertyId::Height => CSSPropertyType::Height,
    PropertyId::Width => CSSPropertyType::Width,

    PropertyId::BorderWidth => CSSPropertyType::BorderWidth,
    PropertyId::BorderTopWidth => CSSPropertyType::BorderTopWidth,
    PropertyId::BorderLeftWidth => CSSPropertyType::BorderLeftWidth,
    PropertyId::BorderRightWidth => CSSPropertyType::BorderRightWidth,
    PropertyId::BorderBottomWidth => CSSPropertyType::BorderBottomWidth,

    PropertyId::BorderColor => CSSPropertyType::BorderColor,
    PropertyId::BorderTopColor => CSSPropertyType::BorderTopColor,
    PropertyId::BorderLeftColor => CSSPropertyType::BorderLeftColor,
    PropertyId::BorderRightColor => CSSPropertyType::BorderRightColor,
    PropertyId::BorderBottomColor => CSSPropertyType::BorderBottomColor,

    PropertyId::Margin => CSSPropertyType::Margin,
    PropertyId::MarginLeft => CSSPropertyType::MarginLeft,
    PropertyId::MarginRight => CSSPropertyType::MarginRight,
    PropertyId::MarginTop => CSSPropertyType::MarginTop,
    PropertyId::MarginBottom => CSSPropertyType::MarginBottom,

    PropertyId::Padding => CSSPropertyType::Padding,
    PropertyId::PaddingLeft => CSSPropertyType::PaddingLeft,
    PropertyId::PaddingRight => CSSPropertyType::PaddingRight,
    PropertyId::PaddingTop => CSSPropertyType::PaddingTop,
    PropertyId::PaddingBottom => CSSPropertyType::PaddingBottom,

    PropertyId::BorderRadius(_) => CSSPropertyType::BorderRadius,
    PropertyId::BorderTopLeftRadius(_) => CSSPropertyType::BorderTopLeftRadius,
    PropertyId::BorderTopRightRadius(_) => CSSPropertyType::BorderTopRightRadius,
    PropertyId::BorderBottomLeftRadius(_) => CSSPropertyType::BorderBottomLeftRadius,
    PropertyId::BorderBottomRightRadius(_) => CSSPropertyType::BorderBottomRightRadius,

    PropertyId::Color => CSSPropertyType::Color,
    PropertyId::BackgroundColor => CSSPropertyType::BackgroundColor,

    PropertyId::Opacity => CSSPropertyType::Opacity,
    PropertyId::FontSize => CSSPropertyType::FontSize,

    PropertyId::Transform(_) => CSSPropertyType::Transform,

    _ => CSSPropertyType::Invalid,
  };
  ret_type
}

impl From<(String, &Property<'_>)> for Transition {
  fn from(value: (String, &Property<'_>)) -> Self {
    //let mut transitions: Vec<SingleTransition> = vec![];
    let mut transtition_properties: Vec<i32> = vec![];
    let mut transition_durations: Vec<f32> = vec![];
    let mut transition_delays: Vec<f32> = vec![];
    let mut transition_timeing_functions: Vec<TransitionTimingFunction> = vec![];

    match value.1 {
      Property::Transition(transtion_list, _) => {
        transtion_list.into_iter().for_each(|transition| {
          let mut transition_duration: Option<f32> = None;
          let mut transition_delay: Option<f32> = None;
          let mut transition_timeing_function: Option<TransitionTimingFunction> = None;
          let mut transtition_property: Option<i32> = None;

          transition_duration = Some(match transition.duration {
            time::Time::Seconds(s) => s * 1000.0,
            time::Time::Milliseconds(m) => m,
          });
          transition_delay = Some(match transition.delay {
            time::Time::Seconds(s) => s * 1000.0,
            time::Time::Milliseconds(m) => m,
          });
          transition_timeing_function = Some(match transition.timing_function {
            _ => TransitionTimingFunction::EasingFunction(transition.timing_function.clone()),
          });
          if transition.property == PropertyId::All {
            transtition_property = Some(-1);
          } else {
            transtition_property = Some(convert_property_to_cssty(&transition.property) as i32);
          }
          transtition_properties.push(transtition_property.unwrap());
          transition_durations.push(transition_duration.unwrap());
          transition_delays.push(transition_delay.unwrap());
          transition_timeing_functions.push(transition_timeing_function.unwrap())
        });
      }

      Property::TransitionProperty(property, _) => {
        property.into_iter().for_each(|transition_property| {
          if *transition_property == PropertyId::All {
            transtition_properties.push(-1);
            return;
          } else {
            transtition_properties.push(convert_property_to_cssty(transition_property) as i32);
          }
        });
      }

      Property::TransitionDelay(delay, _) => {
        delay.into_iter().for_each(|transition_delay| {
          transition_delays.push(match transition_delay {
            time::Time::Seconds(s) => *s * 1000.0,
            time::Time::Milliseconds(m) => *m,
          })
        });
      }
      Property::TransitionDuration(duration, _) => {
        duration.into_iter().for_each(|transition_duration| {
          transition_durations.push(match transition_duration {
            time::Time::Seconds(s) => *s * 1000.0,
            time::Time::Milliseconds(m) => *m,
          })
        });
      }
      Property::TransitionTimingFunction(timing_function, _) => timing_function
        .into_iter()
        .for_each(|transition_timing_function| {
          transition_timeing_functions.push(match transition_timing_function {
            _ => TransitionTimingFunction::EasingFunction(transition_timing_function.clone()),
          });
        }),
      _ => {}
    }

    Transition {
      transtition_properties,
      transition_durations,
      transition_delays,
      transition_timeing_functions,
    }
  }
}

impl ToExpr for Transition {
  fn to_expr(&self) -> PropertyTuple {
    let mut exprs = vec![];

    if !self.transtition_properties.is_empty() {
      let properties = &self.transtition_properties;
      let array_elements: Vec<_> = properties
        .into_iter()
        .map(|property| {
          // Assuming `generate_expr_lit_num!` generates a numeric expression
          let expr = generate_expr_lit_num!(*property as f64);
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(expr),
          })
        })
        .collect();
      exprs.push((
        CSSPropertyType::TransitionProperty,
        Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: array_elements,
        }),
      ));
    }

    if !self.transition_durations.is_empty() {
      let durations = &self.transition_durations;
      let array_elements: Vec<_> = durations
        .into_iter()
        .map(|duration| {
          // Assuming `generate_expr_lit_num!` generates a numeric expression
          let expr = generate_expr_lit_num!(*duration as f64);
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(expr),
          })
        })
        .collect();
      exprs.push((
        CSSPropertyType::TransitionDuration,
        Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: array_elements,
        }),
      ));
    }

    if !self.transition_delays.is_empty() {
      let delays = &self.transition_delays;
      let array_elements: Vec<_> = delays
        .into_iter()
        .map(|delay| {
          // Assuming `generate_expr_lit_num!` generates a numeric expression
          let expr = generate_expr_lit_num!(*delay as f64);
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(expr),
          })
        })
        .collect();
      exprs.push((
        CSSPropertyType::TransitionDelay,
        Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: array_elements,
        }),
      ));
    }

    if !self.transition_timeing_functions.is_empty() {
      let array_elements: Vec<_> = self
        .transition_timeing_functions
        .iter()
        .map(|timing_function| {
          let expr: Expr = match timing_function {
            TransitionTimingFunction::TransitionnCurve(timeing_function) => {
              generate_expr_enum!(*timeing_function)
            }
            TransitionTimingFunction::EasingFunction(easing_function) => {
              generate_expr_lit_str!(easing_function
                .to_css_string(PrinterOptions::default())
                .unwrap())
            }
          };
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(expr),
          })
        })
        .collect();
      exprs.push((
        CSSPropertyType::TransitionTimingFunction,
        Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: array_elements,
        }),
      ));
    }

    PropertyTuple::Array(exprs)
  }
}
