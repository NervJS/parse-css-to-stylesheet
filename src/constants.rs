// pub const CONVERT_STYLE_PREFIX: &'static str = "_";
pub const CONVERT_STYLE_PX_FN: &'static str = "convertNumber2VP";
pub const ENV_FUN: &'static str = "__env__";

// pub const CALC_DYMAMIC_STYLE: &'static str = "calcDynamicStyle";
pub static SUPPORT_PSEUDO_KEYS: [&'static str; 6] = [":before", ":after", ":first-child", ":last-child", ":nth-child", ":empty"];


pub const RN_CONVERT_STYLE_PX_FN: &'static str = "scalePx2dp";
pub const RN_CONVERT_STYLE_VU_FN: &'static str = "scaleVu2dp";


#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum Pseudo {
  Before,
  After,
  FirstChild,
  LastChild,
  NthChild(String),
  Empty,
}

impl Pseudo {
  // 将 SelectorType 枚举值转换为 f64
  pub fn to_f64(self) -> f64 {
    (match &self {
        Pseudo::Before => 0,
        Pseudo::After => 1,
        Pseudo::FirstChild => 2,
        Pseudo::LastChild => 3,
        Pseudo::NthChild(_) => 4,
        Pseudo::Empty => 5,
    }) as f64
  }
}


#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SelectorType {
  Subject,
  Parent,
  Ancestor,
}

impl SelectorType {
  // 将 SelectorType 枚举值转换为 f64
  pub fn to_f64(self) -> f64 {
      self as u32 as f64
  }
}
