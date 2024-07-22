

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use lightningcss::{printer::PrinterOptions, properties::{animation::{self, AnimationFillMode}, Property}, traits::ToCss, values::{easing::{self, EasingFunction},  time}};

use crate::{generate_expr_lit_num, generate_expr_enum, generate_expr_lit_str, style_parser::KeyFrameItem, style_propetries::style_property_enum, visitor::parse_style_values};
use swc_core::{common::DUMMY_SP, ecma::ast::*};
use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::{Platform, PropertyTuple}};


#[derive(Debug, Clone)]
pub enum AnimationTimingFunction {
  AnimationCurve(style_property_enum::ArkUI_AnimationCurve),
  EasingFunction(EasingFunction),
}
#[derive(Debug, Clone)]
pub struct AnimationMulti {
  pub animation_names: Vec<String>,
  pub animation_durations: Vec<f32>,
  pub animation_delays: Vec<f32>,
  pub animation_iterations: Vec<f32>,
  pub animation_fill_modes: Vec<AnimationFillMode>,
  pub animation_timeing_functions: Vec<AnimationTimingFunction>,
  pub keyframes: Option<Rc<RefCell<HashMap<String, Vec<KeyFrameItem>>>>>,
}

impl From<(String, &Property<'_>, Option<Rc<RefCell<HashMap<String, Vec<KeyFrameItem>>>>>)> for AnimationMulti {
  fn from(value: (String, &Property<'_>, Option<Rc<RefCell<HashMap<String, Vec<KeyFrameItem>>>>>)) -> Self {

    let mut animation_names: Vec<String> = vec![];
    let mut animation_durations: Vec<f32> =  vec![]; // 0.0
    let mut animation_delays: Vec<f32> =  vec![]; // 0.0
    let mut animation_iterations: Vec<f32> =  vec![]; // 1.0
    let mut animation_fill_modes: Vec<AnimationFillMode> = vec![];
    let mut animation_timeing_functions: Vec<AnimationTimingFunction> = vec![]; // EasingFunction::Ease
    
    match value.1 {
      // Property::AnimationName(_, _) => todo!(),
      // Property::AnimationDuration(_, _) => todo!(),
      // Property::AnimationTimingFunction(_, _) => todo!(),
      // Property::AnimationIterationCount(_, _) => todo!(),
      // Property::AnimationDirection(_, _) => todo!(),
      // Property::AnimationPlayState(_, _) => todo!(),
      // Property::AnimationDelay(_, _) => todo!(),
      // Property::AnimationFillMode(_, _) => todo!(),
      Property::Animation(animation_list, _) => {
        animation_list.into_iter().for_each(|animation| {
          let animation_name = Some(animation.name.to_css_string(PrinterOptions::default()).unwrap());
          if animation_name.is_none() {
            return;
          }
          animation_names.push(animation_name.unwrap());

          let animation_duration = Some(match animation.duration {
            time::Time::Seconds(s) => s * 1000.0,
            time::Time::Milliseconds(m) => m,
          });
          animation_durations.push(animation_duration.unwrap_or(0.0));

          let animation_delay = Some(match animation.delay {
            time::Time::Seconds(s) => s * 1000.0,
            time::Time::Milliseconds(m) => m,
          });
          animation_delays.push(animation_delay.unwrap_or(0.0));

          let animation_iteration = Some(match animation.iteration_count {
            animation::AnimationIterationCount::Number(num) => num,
            animation::AnimationIterationCount::Infinite => -1.0,
          });
          animation_iterations.push(animation_iteration.unwrap_or(1.0));

          // animation_fill_mode = Some(match animation.fill_mode {
          //   animation::AnimationFillMode::Forwards => style_property_enum::ArkUI_AnimationFillMode::ARKUI_ANIMATION_FILL_MODE_FORWARDS,
          //   animation::AnimationFillMode::Backwards => style_property_enum::ArkUI_AnimationFillMode::ARKUI_ANIMATION_FILL_MODE_BACKWARDS,
          //   animation::AnimationFillMode::Both => style_property_enum::ArkUI_AnimationFillMode::ARKUI_ANIMATION_FILL_MODE_BOTH,
          //   animation::AnimationFillMode::None => style_property_enum::ArkUI_AnimationFillMode::ARKUI_ANIMATION_FILL_MODE_NONE,
          // });
          let animation_fill_mode = Some(animation.fill_mode.clone());
          animation_fill_modes.push(animation_fill_mode.unwrap_or(AnimationFillMode::None));

          let animation_timeing_function = Some(match animation.timing_function {
            // EasingFunction::Linear => AnimationTimingFunction::AnimationCurve(style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_LINEAR),
            // EasingFunction::Ease =>AnimationTimingFunction::AnimationCurve( style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_EASE),
            // EasingFunction::EaseIn => AnimationTimingFunction::AnimationCurve( style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_EASE_IN ),
            // EasingFunction::EaseOut =>AnimationTimingFunction::AnimationCurve ( style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_EASE_OUT ),
            // EasingFunction::EaseInOut =>AnimationTimingFunction::AnimationCurve ( style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_EASE_IN_OUT ),

            _ => AnimationTimingFunction::EasingFunction(animation.timing_function.clone())
          });
          animation_timeing_functions.push(animation_timeing_function.unwrap_or(AnimationTimingFunction::EasingFunction(EasingFunction::Ease)))
        });
      },
      Property::AnimationDelay(delay, _) => {
        for delay_elem in delay {
            let animation_delay = Some(match delay_elem {
                time::Time::Seconds(s) => *s * 1000.0,
                time::Time::Milliseconds(m) => *m,
            });
            animation_delays.push(animation_delay.unwrap_or(0.0));
        }
      },
      Property::AnimationDuration(duration, _) => {
        for duration_elem in duration {
            let animation_duration = Some(match duration_elem {
                time::Time::Seconds(s) => *s * 1000.0,
                time::Time::Milliseconds(m) => *m,
            });
            animation_durations.push(animation_duration.unwrap_or(0.0));
        }
      },
      Property::AnimationIterationCount(iteration, _) => {
        for iteration_elem in iteration {
            let animation_iteration = Some(match iteration_elem {
                animation::AnimationIterationCount::Number(num) => *num,
                animation::AnimationIterationCount::Infinite => -1.0,
            });
            animation_iterations.push(animation_iteration.unwrap_or(1.0));
        }
      },
      Property::AnimationName(name, _) => {
        for name_elem in name {
            let animation_name = Some(name_elem.to_css_string(PrinterOptions::default()).unwrap());
            animation_names.push(animation_name.unwrap_or("".to_string()));
        }
      },
      Property::AnimationTimingFunction(timing_function, _) => {
        for timing_function_elem in timing_function {
            let animation_timeing_function = Some(match timing_function_elem {
                // EasingFunction::Linear => AnimationTimingFunction::AnimationCurve(style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_LINEAR),
                // EasingFunction::Ease =>AnimationTimingFunction::AnimationCurve( style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_EASE),
                // EasingFunction::EaseIn => AnimationTimingFunction::AnimationCurve( style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_EASE_IN ),
                // EasingFunction::EaseOut =>AnimationTimingFunction::AnimationCurve ( style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_EASE_OUT ),
                // EasingFunction::EaseInOut =>AnimationTimingFunction::AnimationCurve ( style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_EASE_IN_OUT ),
                _ => AnimationTimingFunction::EasingFunction(timing_function_elem.clone())
            });
            animation_timeing_functions.push(animation_timeing_function.unwrap_or(AnimationTimingFunction::EasingFunction(EasingFunction::Ease)));
        }
      },
      _ => {}
    }
    
    AnimationMulti {
      keyframes: value.2.clone(),
      animation_names,
      animation_durations,
      animation_delays,
      animation_fill_modes,
      animation_iterations,
      animation_timeing_functions
    }

  }
}


impl ToExpr for AnimationMulti {
  fn to_expr(&self) -> PropertyTuple {
    let mut exprs: Vec<(CSSPropertyType, Expr)> = vec![];
    if !self.animation_names.is_empty() && !self.keyframes.is_none() {
        let mut arr_names: Vec<Option<ExprOrSpread>> = vec![];
        let mut arr_keyframes: Vec<Option<ExprOrSpread>> = vec![];
        let keyframe_map = self.keyframes.as_ref().unwrap().borrow();
        for name in &self.animation_names {
            arr_names.push(
                Some(ExprOrSpread{
                    spread: None,
                    expr: Box::new(generate_expr_lit_str!(name.to_string()))
            }));

            let mut arr_keyframe_items:Vec<Option<ExprOrSpread>> = vec![];
            if let Some(keyframe_items) = keyframe_map.get(name) {
                for keyframe_item in keyframe_items {
                    arr_keyframe_items.push(Some(ExprOrSpread{
                        spread: None,
                        expr: Box::new(Expr::Object(ObjectLit{
                            span: DUMMY_SP,
                            props: vec![
                                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp{
                                    key: PropName::Str("percentage".into()),
                                    value: Box::new(generate_expr_lit_num!(keyframe_item.percentage as f64))
                                }))),
                                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp{
                                    key: PropName::Str("event".into()),
                                    value: Box::new(Expr::Array(ArrayLit{
                                        span: DUMMY_SP,
                                        elems: parse_style_values(keyframe_item.declarations.clone(), Platform::Harmony)
                                    }))
                                })))
                            ]
                        }))
                    }));
                }
            }
            arr_keyframes.push(Some(ExprOrSpread{
                spread: None,
                expr: Box::new(Expr::Array(ArrayLit{
                    span: DUMMY_SP,
                    elems: arr_keyframe_items
                }))
            }));
        }
        exprs.push((
            CSSPropertyType::AnimationName,
            Expr::Array(ArrayLit { span: DUMMY_SP, elems: arr_names.clone() })
        ));
        exprs.push((
            CSSPropertyType::AnimationKeyFrames,
            Expr::Array(ArrayLit { span: DUMMY_SP, elems: arr_keyframes.clone() })
        ));
    }

    if !self.animation_durations.is_empty() {
        let durations = &self.animation_durations;
        let array_elements: Vec<_> = durations.into_iter().map(|duration| {
          // Assuming `generate_expr_lit_num!` generates a numeric expression
          let expr = generate_expr_lit_num!(*duration as f64);
          Some(ExprOrSpread { spread: None, expr: Box::new(expr) })
        }).collect();
        exprs.push((
          CSSPropertyType::AnimationDuration,
          Expr::Array(ArrayLit { span: DUMMY_SP, elems: array_elements })
      ));
    }

    if !self.animation_delays.is_empty() {
        let mut arr_delays: Vec<Option<ExprOrSpread>> = vec![];
        for delay in &self.animation_delays {
            arr_delays.push(
                Some(ExprOrSpread{
                    spread: None,
                    expr: Box::new(generate_expr_lit_num!(*delay as f64))
                }));
        }
        exprs.push((
            CSSPropertyType::AnimationDelay,
            Expr::Array(ArrayLit { span: DUMMY_SP, elems: arr_delays })
        ));
    }

    if !self.animation_iterations.is_empty() {
        let iterations = &self.animation_iterations;
        let array_elements: Vec<_> = iterations.into_iter().map(|iteration| {
          // Assuming `generate_expr_lit_num!` generates a numeric expression
          let expr = generate_expr_lit_num!(*iteration as f64);
          Some(ExprOrSpread { spread: None, expr: Box::new(expr) })
        }).collect();
        exprs.push((
          CSSPropertyType::AnimationIterationCount,
          Expr::Array(ArrayLit { span: DUMMY_SP, elems: array_elements })
      ));
    }

    if !self.animation_fill_modes.is_empty() {
        let fill_modes = &self.animation_fill_modes;
        let array_elements: Vec<_> = fill_modes.into_iter().map(|fill_mode| {
          // Assuming `generate_expr_lit_num!` generates a numeric expression
          let expr = generate_expr_lit_str!(fill_mode.to_css_string(PrinterOptions::default()).unwrap());
          Some(ExprOrSpread { spread: None, expr: Box::new(expr) })
        }).collect();
        exprs.push((
          CSSPropertyType::AnimationFillMode,
          Expr::Array(ArrayLit { span: DUMMY_SP, elems: array_elements })
      ));
    }

    if !self.animation_timeing_functions.is_empty() {
        let array_elements: Vec<_> = self.animation_timeing_functions.iter().map(|timing_function| {
            let expr: Expr = match timing_function {
                AnimationTimingFunction::AnimationCurve(timeing_function) => {
                    generate_expr_enum!(*timeing_function)
                }
                AnimationTimingFunction::EasingFunction(easing_function) => {
                    generate_expr_lit_str!(easing_function.to_css_string(PrinterOptions::default()).unwrap())
                }
            };
            Some(ExprOrSpread { spread: None, expr: Box::new(expr) })
        }).collect();
        exprs.push((
            CSSPropertyType::AnimationTimingFunction,
            Expr::Array(ArrayLit { span: DUMMY_SP, elems: array_elements,
            })
        ));
    }

    PropertyTuple::Array(exprs)
  }

}


