// CSS 属性枚举：由 property_registry 的属性表生成（唯一事实源）。
// ⚠️ 数值 id 是线上产物格式，新增属性只能在末尾追加，禁止改动既有值。

macro_rules! __define_css_property_type {
  ($($variant:ident, $id:expr, $kind:ident, $wrap:ident($payload:ident) $src:ident, $kebab:literal, $camel:literal $(, $alias:literal)* ;)*) => {
    #[repr(u32)]
    #[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
    pub enum CSSPropertyType {
      Invalid = 0,
      $($variant = $id,)*
      All = 99999, // used for transition-property
    }
  };
}

crate::for_each_property_entry!(__define_css_property_type);

/// 按 camelCase 属性名反查属性类型（含别名）。
/// 主 camel 名仅在 simple / compound 属性的"假 camel（仅 CSS 变量反查）"语义无歧义时生效。
macro_rules! __define_string_to_type {
  ($($variant:ident, $id:expr, $kind:ident, $wrap:ident($payload:ident) $src:ident, $kebab:literal, $camel:literal $(, $alias:literal)* ;)*) => {
    pub fn string_to_css_property_type(property: &str) -> CSSPropertyType {
      match property {
        $($camel => CSSPropertyType::$variant, $($alias => CSSPropertyType::$variant,)*)*
        _ => CSSPropertyType::Invalid,
      }
    }
  };
}

crate::for_each_property_entry!(__define_string_to_type);

#[cfg(test)]
mod tests {
  use super::*;

  /// 注册表内部一致性：id 唯一（Invalid/All 除外）、camel→variant 映射可达、
  /// enum 判别值与表内 id 一致。
  #[test]
  fn registry_is_consistent() {
    macro_rules! __collect {
      ($($variant:ident, $id:expr, $kind:ident, $wrap:ident($payload:ident) $src:ident, $kebab:literal, $camel:literal $(, $alias:literal)* ;)*) => {{
        let ids: Vec<(u32, &str)> = vec![$( ($id, stringify!($variant)) ),*];
        let enum_ids: Vec<(u32, &str)> = vec![$( (CSSPropertyType::$variant as u32, stringify!($variant)) ),*];
        (ids, enum_ids)
      }};
    }
    let (ids, enum_ids) = crate::for_each_property_entry!(__collect);
    // 1) id 唯一
    for (i, (id, name)) in ids.iter().enumerate() {
      assert!(
        !ids.iter().skip(i + 1).any(|(other, _)| other == id),
        "duplicate id {} on {}", id, name
      );
    }
    // 2) 表内 id == enum 判别值
    assert_eq!(ids, enum_ids);
    // 3) camel 名可反查（主名 + 别名）
    assert_eq!(string_to_css_property_type("width") as u32, CSSPropertyType::Width as u32);
    assert_eq!(string_to_css_property_type("webkitLineClamp") as u32, CSSPropertyType::WebkitLineClamp as u32);
    assert_eq!(string_to_css_property_type("PointerEvents") as u32, CSSPropertyType::PointerEvents as u32);
    assert_eq!(string_to_css_property_type("nope"), CSSPropertyType::Invalid);
  }
}
