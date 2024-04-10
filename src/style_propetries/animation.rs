

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use lightningcss::{printer::PrinterOptions, properties::{animation, Property}, traits::ToCss, values::time};

use crate::{generate_expr_lit_num, generate_invalid_expr, style_parser::KeyFrameItem, visitor::parse_style_values};
use swc_core::{common::DUMMY_SP, ecma::ast::*};
use super::{traits::ToExpr, unit::{Platform, PropertyTuple}};

#[derive(Debug, Clone)]
pub struct Animation {
  pub id: String,
  pub keyframs: Rc<RefCell<HashMap<String, Vec<KeyFrameItem>>>>,
  pub animation_name: Option<String>,
  pub animation_duration: f32,
  pub animation_delay: f32,
  pub animation_iteration: f32
  // pub value: Option<Vec<KeyFrameItem>>
}

impl From<(String, &Property<'_>, Rc<RefCell<HashMap<String, Vec<KeyFrameItem>>>>)> for Animation {
  fn from(value: (String, &Property<'_>, Rc<RefCell<HashMap<String, Vec<KeyFrameItem>>>>)) -> Self {


    let mut animation_name = None;
    let mut animation_duration: f32 = 0.0;
    let mut animation_delay: f32 = 0.0;
    let mut animation_iteration: f32 = 1.0;
    
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
          animation_name = Some(animation.name.to_css_string(PrinterOptions::default()).unwrap());
          animation_duration = match animation.duration {
            time::Time::Seconds(s) => s,
            time::Time::Milliseconds(m) => m * 60.0,
          };
          animation_delay = match animation.delay {
            time::Time::Seconds(s) => s,
            time::Time::Milliseconds(m) => m * 60.0,
          };
          animation_iteration = match animation.iteration_count {
            animation::AnimationIterationCount::Number(num) => num,
            animation::AnimationIterationCount::Infinite => -1.0,
          }
        });
      },
      _ => {}
    }
    
    Animation {
      id: value.0,
      keyframs: value.2.clone(),
      animation_name,
      animation_duration,
      animation_delay,
      animation_iteration
    }

  }
}


impl ToExpr for Animation {
  fn to_expr(&self) -> PropertyTuple {
    if let Some(name) = &self.animation_name {
      let keyframe_map = self.keyframs.borrow();
      if let Some(keyframe_items) = keyframe_map.get(name) {

        

        return PropertyTuple::One(
          "animation".to_string(),
          Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![
              PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Str("params".into()),
                value: Box::new(Expr::Object(ObjectLit {
                  span: DUMMY_SP,
                  props: vec![
                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                      key: PropName::Str("delay".into()),
                      value: Box::new(generate_expr_lit_num!((self.animation_delay * 1000.0) as f64))
                    }))),
                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                      key: PropName::Str("iterations".into()),
                      value: Box::new(generate_expr_lit_num!(self.animation_iteration as f64))
                    }))),
                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                      key: PropName::Str("duration".into()),
                      value: Box::new(generate_expr_lit_num!((self.animation_duration * 1000.0) as f64))
                    }))),
                  ]
                }))
              }))),
              PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Str("keyframes".into()),
                value: Box::new(Expr::Array(ArrayLit {
                  span: DUMMY_SP,
                  elems: keyframe_items.into_iter().map(|item| {
                    Some(ExprOrSpread {
                      spread: None,
                      expr: Box::new(Expr::Object(ObjectLit {
                        span: DUMMY_SP,
                        props: vec![
                          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                            key: PropName::Str("percentage".into()),
                            value: Box::new(generate_expr_lit_num!(item.percentage as f64))
                          }))),
                          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                            key: PropName::Str("event".into()),
                            value: Box::new(Expr::Object(ObjectLit {
                              span: DUMMY_SP,
                              props: parse_style_values(item.declarations.clone(), Platform::Harmony)
                            }))
                          })))
                        ]
                      }))
                    })
                  }).collect::<Vec<Option<ExprOrSpread>>>()
                }))
              })))
            ]
          })
        )
      }
    }
    PropertyTuple::One(
      self.id.to_string(),
      generate_invalid_expr!()
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.id.to_string(),
      generate_invalid_expr!()
    )
  }
}

