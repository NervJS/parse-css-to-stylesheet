pub mod translate;
pub mod rotate;
pub mod scale;
pub mod skew;
pub mod matrix;

#[repr(u32)] 
#[derive(Debug, PartialEq, Eq)]
pub enum ETransformType {
    Matrix = 0,
    Translate = 1,
    Scale = 2,
    Rotate = 3,
    Skew = 4,
}