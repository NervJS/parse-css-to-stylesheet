use crate::generate_transform_item;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{ArrayLit, Expr};

use self::{matrix::Matrix, rotate::Rotate, scale::Scale, translate::Translate};

use super::traits::ToExpr;

pub mod matrix;
pub mod rotate;
pub mod scale;
pub mod transform;
pub mod translate;

generate_transform_item!(Translates, Translate);
generate_transform_item!(Rotates, Rotate);
generate_transform_item!(Scales, Scale);
generate_transform_item!(Matrices, Matrix);
