use bitflags::bitflags;

// pub const CONVERT_STYLE_PREFIX: &'static str = "_";
pub const CONVERT_STYLE_PX_FN: &'static str = "convertNumber2VP";
pub const ENV_FUN: &'static str = "__env__";

// pub const CALC_DYMAMIC_STYLE: &'static str = "calcDynamicStyle";
pub static SUPPORT_PSEUDO_KEYS: [&'static str; 6] = [
  ":before",
  ":after",
  ":first-child",
  ":last-child",
  ":nth-child",
  ":empty",
];

pub const RN_CONVERT_STYLE_PX_FN: &'static str = "scalePx2dp";
pub const RN_CONVERT_STYLE_VU_FN: &'static str = "scaleVu2dp";

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Pseudo {
  None,
  Before,
  After,
  FirstChild,
  LastChild,
  NthChild(i32, i32, bool),
  Empty,
}

impl Pseudo {
  // 将 SelectorType 枚举值转换为 f64
  pub fn to_f64(self) -> f64 {
    (match &self {
      Pseudo::None => 0,
      Pseudo::Before => 1,
      Pseudo::After => 2,
      Pseudo::FirstChild => 3,
      Pseudo::LastChild => 4,
      Pseudo::NthChild(_, _, _) => 5,
      Pseudo::Empty => 6,
    }) as f64
  }
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SelectorType {
  Subject,
  Parent,
  Ancestor,
  Multiple,
}

impl SelectorType {
  // 将 SelectorType 枚举值转换为 f64
  pub fn to_f64(self) -> f64 {
    self as u32 as f64
  }
}

bitflags! {
    #[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
    pub struct ValueFlag: u32 {
        const NONE = 0;
        const VARIABLE = 1;
        const IMPORTANT = 2;
    }
}

impl ValueFlag {
  // 将 SelectorType 枚举值转换为 f64
  pub fn to_f64(self) -> f64 {
    self.bits() as f64
  }
}