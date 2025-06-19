use std::{cell::RefCell, collections::HashMap, rc::Rc};

use lightningcss::{
  printer::PrinterOptions,
  properties::{
    animation::{self, AnimationFillMode},
    Property,
  },
  traits::ToCss,
  values::{easing::EasingFunction, time},
};

use super::{
  style_property_type::CSSPropertyType,
  traits::ToExpr,
  unit::{Platform, PropertyTuple},
};
use crate::{
  generate_expr_enum, generate_expr_lit_num, generate_expr_lit_str, style_parser::KeyFrameItem,
  style_propetries::style_property_enum, visitor::parse_style_values,
};
use swc_core::{common::DUMMY_SP, ecma::ast::*};

#[derive(Debug, Clone)]
pub enum AnimationTimingFunction {
  AnimationCurve(style_property_enum::ArkUI_AnimationCurve),
  EasingFunction(EasingFunction),
}

#[derive(Debug, Clone)]
pub enum AnimationDirection {
  Normal,
  Reverse,
  Alternate,
  AlternateReverse,
}

#[derive(Debug, Clone)]
pub enum AnimationPlayState {
  Paused,
  Running,
}

#[derive(Debug, Clone)]
pub struct AnimationMulti {
  pub animation_names: Vec<String>,
  pub animation_durations: Vec<f32>,
  pub animation_delays: Vec<f32>,
  pub animation_iterations: Vec<f32>,
  pub animation_fill_modes: Vec<AnimationFillMode>,
  pub animation_timeing_functions: Vec<AnimationTimingFunction>,
  pub animation_directions: Vec<style_property_enum::ArkUI_AnimationDirection>,
  pub animation_play_states: Vec<style_property_enum::ArkUI_AnimationPlayState>,
}

impl From<(String, &Property<'_>)> for AnimationMulti {
  fn from(value: (String, &Property<'_>)) -> Self {
    let mut animation_names: Vec<String> = vec![];
    let mut animation_durations: Vec<f32> = vec![]; // 0.0
    let mut animation_delays: Vec<f32> = vec![]; // 0.0
    let mut animation_iterations: Vec<f32> = vec![]; // 1.0
    let mut animation_fill_modes: Vec<AnimationFillMode> = vec![];
    let mut animation_timeing_functions: Vec<AnimationTimingFunction> = vec![]; // EasingFunction::Ease
    let mut animation_directions: Vec<style_property_enum::ArkUI_AnimationDirection> = vec![]; // ArkUI_AnimationDirection::ARKUI_ANIMATION_DIRECTION_NORMAL
    let mut animation_play_states: Vec<style_property_enum::ArkUI_AnimationPlayState> = vec![]; // ArkUI_AnimationPlayState::ARKUI_ANIMATION_PLAY_STATE_RUNNING

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
          let animation_name = Some(
            animation
              .name
              .to_css_string(PrinterOptions::default())
              .unwrap(),
          );
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
            _ => AnimationTimingFunction::EasingFunction(animation.timing_function.clone()),
          });
          animation_timeing_functions.push(animation_timeing_function.unwrap_or(
            AnimationTimingFunction::EasingFunction(EasingFunction::Ease),
          ));

          let animation_direction = Some(match animation.direction {
            animation::AnimationDirection::Normal => style_property_enum::ArkUI_AnimationDirection::ARKUI_ANIMATION_DIRECTION_NORMAL,
            animation::AnimationDirection::Reverse => style_property_enum::ArkUI_AnimationDirection::ARKUI_ANIMATION_DIRECTION_REVERSE,
            animation::AnimationDirection::Alternate => style_property_enum::ArkUI_AnimationDirection::ARKUI_ANIMATION_DIRECTION_ALTERNATE,
            animation::AnimationDirection::AlternateReverse => style_property_enum::ArkUI_AnimationDirection::ARKUI_ANIMATION_DIRECTION_ALTERNATE_REVERSE,
          });
          animation_directions.push(animation_direction.unwrap_or(style_property_enum::ArkUI_AnimationDirection::ARKUI_ANIMATION_DIRECTION_NORMAL));

          let animation_play_state = Some(match animation.play_state {
            animation::AnimationPlayState::Paused => style_property_enum::ArkUI_AnimationPlayState::ARKUI_ANIMATION_PLAY_STATE_PAUSED,
            animation::AnimationPlayState::Running => style_property_enum::ArkUI_AnimationPlayState::ARKUI_ANIMATION_PLAY_STATE_RUNNING,
          });
          animation_play_states.push(animation_play_state.unwrap_or(style_property_enum::ArkUI_AnimationPlayState::ARKUI_ANIMATION_PLAY_STATE_RUNNING));
        });
      }
      Property::AnimationDelay(delay, _) => {
        for delay_elem in delay {
          let animation_delay = Some(match delay_elem {
            time::Time::Seconds(s) => *s * 1000.0,
            time::Time::Milliseconds(m) => *m,
          });
          animation_delays.push(animation_delay.unwrap_or(0.0));
        }
      }
      Property::AnimationDuration(duration, _) => {
        for duration_elem in duration {
          let animation_duration = Some(match duration_elem {
            time::Time::Seconds(s) => *s * 1000.0,
            time::Time::Milliseconds(m) => *m,
          });
          animation_durations.push(animation_duration.unwrap_or(0.0));
        }
      }
      Property::AnimationIterationCount(iteration, _) => {
        for iteration_elem in iteration {
          let animation_iteration = Some(match iteration_elem {
            animation::AnimationIterationCount::Number(num) => *num,
            animation::AnimationIterationCount::Infinite => -1.0,
          });
          animation_iterations.push(animation_iteration.unwrap_or(1.0));
        }
      }
      Property::AnimationName(name, _) => {
        for name_elem in name {
          let animation_name = Some(name_elem.to_css_string(PrinterOptions::default()).unwrap());
          animation_names.push(animation_name.unwrap_or("".to_string()));
        }
      }
      Property::AnimationTimingFunction(timing_function, _) => {
        for timing_function_elem in timing_function {
          let animation_timeing_function = Some(match timing_function_elem {
            // EasingFunction::Linear => AnimationTimingFunction::AnimationCurve(style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_LINEAR),
            // EasingFunction::Ease =>AnimationTimingFunction::AnimationCurve( style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_EASE),
            // EasingFunction::EaseIn => AnimationTimingFunction::AnimationCurve( style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_EASE_IN ),
            // EasingFunction::EaseOut =>AnimationTimingFunction::AnimationCurve ( style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_EASE_OUT ),
            // EasingFunction::EaseInOut =>AnimationTimingFunction::AnimationCurve ( style_property_enum::ArkUI_AnimationCurve::ARKUI_CURVE_EASE_IN_OUT ),
            _ => AnimationTimingFunction::EasingFunction(timing_function_elem.clone()),
          });
          animation_timeing_functions.push(animation_timeing_function.unwrap_or(
            AnimationTimingFunction::EasingFunction(EasingFunction::Ease),
          ));
        }
      }
      Property::AnimationDirection(direction, _) => {
        for direction_elem in direction {
          let animation_direction = Some(match direction_elem {
            animation::AnimationDirection::Normal => style_property_enum::ArkUI_AnimationDirection::ARKUI_ANIMATION_DIRECTION_NORMAL,
            animation::AnimationDirection::Reverse => style_property_enum::ArkUI_AnimationDirection::ARKUI_ANIMATION_DIRECTION_REVERSE,
            animation::AnimationDirection::Alternate => style_property_enum::ArkUI_AnimationDirection::ARKUI_ANIMATION_DIRECTION_ALTERNATE,
            animation::AnimationDirection::AlternateReverse => style_property_enum::ArkUI_AnimationDirection::ARKUI_ANIMATION_DIRECTION_ALTERNATE_REVERSE,
          });
          animation_directions.push(animation_direction.unwrap_or(style_property_enum::ArkUI_AnimationDirection::ARKUI_ANIMATION_DIRECTION_NORMAL));
        }
      }
      Property::AnimationPlayState(play_state, _) => {
        for play_state_elem in play_state {
          let animation_play_state = Some(match play_state_elem {
            animation::AnimationPlayState::Paused => style_property_enum::ArkUI_AnimationPlayState::ARKUI_ANIMATION_PLAY_STATE_PAUSED,
            animation::AnimationPlayState::Running => style_property_enum::ArkUI_AnimationPlayState::ARKUI_ANIMATION_PLAY_STATE_RUNNING,
          });
          animation_play_states.push(animation_play_state.unwrap_or(style_property_enum::ArkUI_AnimationPlayState::ARKUI_ANIMATION_PLAY_STATE_RUNNING));
        }
      }
      Property::AnimationFillMode(fill_mode, _) => {
        for fill_mode_elem in fill_mode {
          animation_fill_modes.push(fill_mode_elem.clone());
        }
      }
      _ => {}
    }

    AnimationMulti {
      animation_names,
      animation_durations,
      animation_delays,
      animation_fill_modes,
      animation_iterations,
      animation_timeing_functions,
      animation_directions,
      animation_play_states,
    }
  }
}

impl ToExpr for AnimationMulti {
  fn to_expr(&self) -> PropertyTuple {
    let mut exprs: Vec<(CSSPropertyType, Expr)> = vec![];
    if !self.animation_names.is_empty() {
      let mut arr_names: Vec<Option<ExprOrSpread>> = vec![];
      for name in &self.animation_names {
        arr_names.push(Some(ExprOrSpread {
          spread: None,
          expr: Box::new(generate_expr_lit_str!(name.to_string())),
        }));
      }
      exprs.push((
        CSSPropertyType::AnimationName,
        Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: arr_names.clone(),
        }),
      ));
    }

    if !self.animation_durations.is_empty() {
      let durations = &self.animation_durations;
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
        CSSPropertyType::AnimationDuration,
        Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: array_elements,
        }),
      ));
    }

    if !self.animation_delays.is_empty() {
      let mut arr_delays: Vec<Option<ExprOrSpread>> = vec![];
      for delay in &self.animation_delays {
        arr_delays.push(Some(ExprOrSpread {
          spread: None,
          expr: Box::new(generate_expr_lit_num!(*delay as f64)),
        }));
      }
      exprs.push((
        CSSPropertyType::AnimationDelay,
        Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: arr_delays,
        }),
      ));
    }

    if !self.animation_iterations.is_empty() {
      let iterations = &self.animation_iterations;
      let array_elements: Vec<_> = iterations
        .into_iter()
        .map(|iteration| {
          // Assuming `generate_expr_lit_num!` generates a numeric expression
          let expr = generate_expr_lit_num!(*iteration as f64);
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(expr),
          })
        })
        .collect();
      exprs.push((
        CSSPropertyType::AnimationIterationCount,
        Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: array_elements,
        }),
      ));
    }

    if !self.animation_fill_modes.is_empty() {
      let fill_modes = &self.animation_fill_modes;
      let array_elements: Vec<_> = fill_modes
        .into_iter()
        .map(|fill_mode| {
          let enum_value = match fill_mode {
            AnimationFillMode::None => style_property_enum::ArkUI_AnimationFillMode::ARKUI_ANIMATION_FILL_MODE_NONE,
            AnimationFillMode::Forwards => style_property_enum::ArkUI_AnimationFillMode::ARKUI_ANIMATION_FILL_MODE_FORWARDS,
            AnimationFillMode::Backwards => style_property_enum::ArkUI_AnimationFillMode::ARKUI_ANIMATION_FILL_MODE_BACKWARDS,
            AnimationFillMode::Both => style_property_enum::ArkUI_AnimationFillMode::ARKUI_ANIMATION_FILL_MODE_BOTH,
          };
          let expr = generate_expr_enum!(enum_value);
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(expr),
          })
        })
        .collect();
      exprs.push((
        CSSPropertyType::AnimationFillMode,
        Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: array_elements,
        }),
      ));
    }

    if !self.animation_timeing_functions.is_empty() {
      let array_elements: Vec<_> = self
        .animation_timeing_functions
        .iter()
        .map(|timing_function| {
          let expr: Expr = match timing_function {
            AnimationTimingFunction::AnimationCurve(timeing_function) => {
              generate_expr_enum!(*timeing_function)
            }
            AnimationTimingFunction::EasingFunction(easing_function) => {
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
        CSSPropertyType::AnimationTimingFunction,
        Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: array_elements,
        }),
      ));
    }

    if !self.animation_directions.is_empty() {
      let directions = &self.animation_directions;
      let array_elements: Vec<_> = directions
        .into_iter()
        .map(|direction| {
          let expr = generate_expr_enum!(*direction);
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(expr),
          })
        })
        .collect();
      exprs.push((
        CSSPropertyType::AnimationDirection,
        Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: array_elements,
        }),
      ));
    }

    if !self.animation_play_states.is_empty() {
      let play_states = &self.animation_play_states;
      let array_elements: Vec<_> = play_states
        .into_iter()
        .map(|play_state| {
          let expr = generate_expr_enum!(*play_state);
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(expr),
          })
        })
        .collect();
      exprs.push((
        CSSPropertyType::AnimationPlayState,
        Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: array_elements,
        }),
      ));
    }

    PropertyTuple::Array(exprs)
  }
}
