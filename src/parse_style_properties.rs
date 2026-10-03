use lightningcss::{
  properties::{custom::TokenOrValue, Property},
  stylesheet::PrinterOptions, traits::ToCss, vendor_prefix::VendorPrefix,
};

use crate::{
  generate_expr_lit_str,
  style_propetries::{
    animation_multi::AnimationMulti, aspect_ratio::AspectRatio, background::Background, background_image::BackgroundImage, background_position::BackgroundPosition, background_repeat::BackgroundRepeat, background_size::BackgroundSize, border::Border, border_color::BorderColor, border_radius::BorderRadius, border_style::BorderStyle, border_width::BorderWidth, box_orient::BoxOrient, box_shadow::BoxShadow, color::ColorProperty, display::Display, expr::Expr, flex::Flex, flex_align::FlexAlign, flex_basis::FlexBasis, flex_direction::FlexDirection, flex_wrap::FlexWrap, font_size::FontSize, font_style::FontStyle, font_weight::FontWeight, gap::Gap, grid_auto_flow::GridAutoFlow, grid_placement::GridPlacement, grid_template::GridTemplate, item_align::ItemAlign, length_value::LengthValueProperty, letter_spacing::LetterSpacing, line_height::LineHeight, marin_padding::MarginPadding, max_size::MaxSizeProperty, normal::Normal, number::NumberProperty, opacity::Opacity, overflow::Overflow, pointer_events::PointerEvents, position::Position, size::SizeProperty, style_property_type::{string_to_css_property_type, CSSPropertyType}, style_value_type::{CssVariable, StyleValueType}, text_align::TextAlign, text_decoration::TextDecoration, text_overflow::TextOverflow, text_shadow::TextShadow, text_transform::TextTransform, transform::Transform, transform_origin::TransformOrigin, transition::Transition, variable::Variable, vertical_align::VerticalAlign, visibility::Visibility, white_space::WhiteSpace, word_break::WordBreak

  },
  utils::lowercase_first,
};

#[derive(Debug, Clone)]
pub struct DeclsAndVars {
  pub decls: Vec<StyleValueType>,
  pub vars: Vec<CssVariable>,
  pub has_env: bool
}

// ---------------------------------------------------------------------------
// 注册表派发：把 `match property_name { ... }` 的同构臂（约 60 条）从
// `property_registry.rs` 的表自动展开。
//
// 表内 `src` 字段：
//   - `pname` — 把当前 camelCase 属性名（vendor 前缀摘除 + 首字母小写）传给 `From::from`
//     例如 `FlexAlign::from(("justifyContent", value))` 中 `"justifyContent"` 决定枚举 id。
//   - `id`    — 把底层 lightningcss `PropertyId` 的原始 camelCase 字符串传给 `From::from`
//     例如 `Gap::from(("columnGap", value))`。
//   - `special` / `skip` — 在此生成 `None`，由下面的手写 match 继续处理。
//
// 返回 `Some(StyleValueType)` 表示匹配并成功构造，`None` 则 fallthrough。
// 语义与原始 dispatch 完全一致：行为差异仅在源码层面（去重），JS 产物字节级不变。
// ---------------------------------------------------------------------------

/// 单个属性的 wrap 派发器：`StyleValueType::$wrapper($payload::from((.., value)))`。
/// `$src` ∈ {pname, id} 决定第一个 tuple 元素用的是 property name 还是原始 id。
macro_rules! __dispatch_wrap_one {
  (pname, $wrapper:ident, $payload:ident, $pname:expr, $id:expr, $value:expr) => {
    Some(StyleValueType::$wrapper($payload::from(($pname.to_string(), $value))))
  };
  (id, $wrapper:ident, $payload:ident, $pname:expr, $id:expr, $value:expr) => {
    Some(StyleValueType::$wrapper($payload::from(($id.to_string(), $value))))
  };
  // special / skip：不在通用派发中处理
  (special, $wrapper:ident, $payload:ident, $pname:expr, $id:expr, $value:expr) => { None };
  (skip, $wrapper:ident, $payload:ident, $pname:expr, $id:expr, $value:expr) => { None };
}

macro_rules! __define_registry_dispatch {
  ($($variant:ident, $id_num:expr, $kind:ident, $wrap:ident($payload:ident) $src:ident, $kebab:literal, $camel:literal $(, $alias:literal)* ;)*) => {
    /// 注册表驱动的属性派发。匹配 camelCase 属性名（含别名）→ 构造 StyleValueType。
    ///
    /// - `property_name`：已 vendor 前缀摘除 + 首字母小写的 camelCase 名
    /// - `id`：原始 lightningcss `PropertyId` 的 camelCase 字符串（`FlexAlign` 等结构需用它选 id）
    /// - `value`：lightningcss `Property`
    ///
    /// 返回 `None` 时由调用点继续走手写的特殊臂（fontFamily / content / zIndex / …）。
    pub fn registry_dispatch_property(
      property_name: &str,
      id: &str,
      value: &lightningcss::properties::Property<'_>,
    ) -> Option<StyleValueType> {
      match property_name {
        $(
          $camel $(| $alias)* => __dispatch_wrap_one!($src, $wrap, $payload, property_name, id, value),
        )*
        _ => None,
      }
    }
  };
}

crate::for_each_property_entry!(__define_registry_dispatch);

pub fn parse_style_properties(properties: &Vec<(String, Property)>) -> DeclsAndVars {
  let mut final_properties = vec![];
  let mut variable_properties = vec![];
  let mut has_env = false;
  for (id, value)  in properties.iter() {
    let mut is_var: bool = false;
    let mut is_env: bool = false;
    match value {
      Property::Unparsed(unparsed) => {
        // 检查是否包含 var() 函数
        is_var = unparsed.value.0.iter().any(|token| {
          match token {
            TokenOrValue::Function(f) => {
              // 检查函数内的变量
              f.arguments.0.iter().any(|arg| matches!(arg, TokenOrValue::Var(_)))
            },
            TokenOrValue::Var(_) => true,
            _ => false
          }
        });

        // 分析属性值中的所有 token
        for token in unparsed.value.0.iter() {
          match token {
            TokenOrValue::Env(_) => is_env = true,
            _ => {}
          }
        }

        // // 处理包含变量的情况
        if is_var {
          final_properties.push(
            StyleValueType::Variable(
              Variable::new(
                string_to_css_property_type(id),
                generate_expr_lit_str!(
                      value.value_to_css_string(PrinterOptions::default()).unwrap()
                    )
              )
            )
          );
          continue;
        }

        // 处理环境变量
        if is_env {
          if let Ok(env_value) = value.value_to_css_string(PrinterOptions::default()) {
            has_env = true;
            final_properties.push(
              StyleValueType::Expr(
                Expr::new(
                  string_to_css_property_type(id),
                  generate_expr_lit_str!(env_value)
                )
              )
            );
          }
          continue;
        }
      },
      Property::Custom(custom) => {
        let id_ = custom.name.to_css_string(Default::default()).unwrap();
        // css 变量
        if id_.starts_with("--") {

          let re = regex::Regex::new(r#"\b(\d+(?:px|vw|vh))\b"#).unwrap();
          let var_str = value.value_to_css_string(PrinterOptions::default()).unwrap().to_string();
          let result = re.replace_all(var_str.as_str(), |caps: &regex::Captures| {
            let value = &caps[1];
            let unit = &value[value.len() - 2..];
            let parsed_value: i32 = value[..value.len() - 2].parse().unwrap();
            if unit == "px" {
              format!("{}lpx", parsed_value)
            } else {
              format!("{}{}", parsed_value, unit)
            }
          });
          variable_properties.push(
            CssVariable {
              id: id_,
              value: result.to_string(),
            }
          );
        }
      }
      _ => {}
    };
    if is_env || is_var {
      continue;
    }


    let mut property_name = id.as_str();

    // 移除部分厂商前缀: Webkit, Moz, 并且把首字母小写
    if property_name.starts_with("Webkit") {
      property_name = &property_name[6..];
    } else if property_name.starts_with("Moz") {
      property_name = &property_name[3..];
    }

    // 将property_name首字母小写
    let mut property_name = property_name.to_string();
    lowercase_first(&mut property_name);

    // lightningcss 1.0.0-alpha.45 将合法的单独 dense 留作 Unparsed，
    // 但 CSS string parser 会把它当作 row dense（位值 2）。
    if property_name == "gridAutoFlow" && matches!(value, Property::Unparsed(_)) {
      if value.value_to_css_string(PrinterOptions::default())
        .is_ok_and(|css| css.trim().eq_ignore_ascii_case("dense")) {
        final_properties.push(StyleValueType::GridAutoFlow(GridAutoFlow {
          id: CSSPropertyType::GridAutoFlow,
          bits: Some(2),
        }));
        continue;
      }
    }

    // -------------------------------------------------------------------
    // 第一步：注册表同构派发（约 60 条 wrap 臂），匹配即 push
    // -------------------------------------------------------------------
    if let Some(decl) = registry_dispatch_property(property_name.as_str(), id.as_str(), value) {
      final_properties.push(decl);
      continue;
    }

    // -------------------------------------------------------------------
    // 第二步：特殊臂（每条都是真实业务逻辑，保留手写）
    //   8 条：fontFamily / content / zIndex / lineClamp / textUnderlineOffset
    //         / filter / backdropFilter / gridTemplate(+ Columns/Rows 简写拆分)
    // -------------------------------------------------------------------
    match property_name.as_str() {
      "fontFamily" => {
        final_properties.push(StyleValueType::Expr(Expr::new(
          CSSPropertyType::FontFamily,
          {
            // 直接从 Property::FontFamily 提取原始值，避免 CSS 转义
            let font_family_str = match value {
              Property::FontFamily(font_families) => {
                // 提取所有字体名称，用逗号连接，不进行 CSS 转义
                font_families
                  .iter()
                  .map(|family| {
                    match family {
                      lightningcss::properties::font::FontFamily::FamilyName(name) => name.as_ref().to_string(),
                      lightningcss::properties::font::FontFamily::Generic(generic) => generic.to_css_string(PrinterOptions::default()).unwrap(),
                    }
                  })
                  .collect::<Vec<_>>()
                  .join(", ")
              },
              _ => {
                // 如果不是 FontFamily 类型，回退到 CSS 字符串方式，但去掉转义
                value.value_to_css_string(PrinterOptions::default())
                  .map(|s| s.replace("\\", "").replace("\"", ""))
                  .unwrap_or_else(|e| {
                    eprintln!("fontFamily value_to_css_string failed: {:?}, value: {:?}", e, value);
                    String::new()
                  })
              }
            };
            generate_expr_lit_str!(font_family_str)
          }
        )));
      }
      "gridTemplate" => {
        // grid-template 简写：无 areas 时拆出 rows/columns 两条独立声明
        if let lightningcss::properties::Property::GridTemplate(template) = value {
          if let lightningcss::properties::grid::GridTemplateAreas::None = &template.areas {
            final_properties.push(StyleValueType::GridTemplate(GridTemplate::from((
              "gridTemplateRows".to_string(),
              &lightningcss::properties::Property::GridTemplateRows(template.rows.clone()),
            ))));
            final_properties.push(StyleValueType::GridTemplate(GridTemplate::from((
              "gridTemplateColumns".to_string(),
              &lightningcss::properties::Property::GridTemplateColumns(template.columns.clone()),
            ))));
          }
          // 带 areas 的 grid-template 属于方案 2，不支持时静默跳过
        }
      }
      "gridTemplateColumns" | "gridTemplateRows" => {
        final_properties.push(StyleValueType::GridTemplate(GridTemplate::from((
          property_name.to_string(),
          value,
        ))));
      }
      "gridAutoRows" | "gridAutoColumns" => {
        // 与 template 同构的 track sizing 列表，复用 GridTemplate 数值槽
        final_properties.push(StyleValueType::GridTemplate(GridTemplate::from((
          property_name.to_string(),
          value,
        ))));
      }
      // place-* 简写：编译期展开为两条长属性（align-* 既有 id + justify-* grid id），
      // 运行时无需感知简写
      "placeItems" => {
        if let Property::PlaceItems(place) = value {
          let align_prop = Property::AlignItems(place.align.clone(), VendorPrefix::None);
          let justify_prop = Property::JustifyItems(place.justify.clone());
          final_properties.push(StyleValueType::AlignItems(ItemAlign::from((
            "alignItems".to_string(),
            &align_prop,
          ))));
          final_properties.push(StyleValueType::AlignItems(ItemAlign::from((
            "justifyItems".to_string(),
            &justify_prop,
          ))));
        }
      }
      "placeSelf" => {
        if let Property::PlaceSelf(place) = value {
          let align_prop = Property::AlignSelf(place.align.clone(), VendorPrefix::None);
          let justify_prop = Property::JustifySelf(place.justify.clone());
          final_properties.push(StyleValueType::AlignItems(ItemAlign::from((
            "alignSelf".to_string(),
            &align_prop,
          ))));
          final_properties.push(StyleValueType::AlignItems(ItemAlign::from((
            "justifySelf".to_string(),
            &justify_prop,
          ))));
        }
      }
      "placeContent" => {
        if let Property::PlaceContent(place) = value {
          let align_prop = Property::AlignContent(place.align.clone(), VendorPrefix::None);
          let justify_prop = Property::JustifyContent(place.justify.clone(), VendorPrefix::None);
          final_properties.push(StyleValueType::FlexAlign(FlexAlign::from((
            "alignContent".to_string(),
            &align_prop,
          ))));
          final_properties.push(StyleValueType::FlexAlign(FlexAlign::from((
            "justifyContent".to_string(),
            &justify_prop,
          ))));
        }
      }
      "content" => {
        // 判断content内容是否是空字符串
        let content_value = value
          .value_to_css_string(PrinterOptions::default())
          .unwrap()
          .trim()
          .to_string();
        if content_value != "\"\"" {
          // 替换字符串，将左右两边的"干掉
          let content_value = content_value.trim_matches('"');
          final_properties.push(StyleValueType::Normal(Normal::new(
            CSSPropertyType::Content,
            content_value.to_string(),
          )));
        }
      }
      "zIndex" => {
        final_properties.push(StyleValueType::Normal(Normal::new(
          CSSPropertyType::ZIndex,
          value
            .value_to_css_string(PrinterOptions::default())
            .unwrap(),
        )));
      }
      "lineClamp" => {
        final_properties.push(StyleValueType::Normal(Normal::new(
          CSSPropertyType::WebkitLineClamp,
          value
            .value_to_css_string(PrinterOptions::default())
            .unwrap(),
        )));
      }
      "textUnderlineOffset" => {
        final_properties.push(StyleValueType::Expr(Expr::new(
          CSSPropertyType::TextUnderlineOffset,
          generate_expr_lit_str!(value.value_to_css_string(PrinterOptions::default()).unwrap()),
        )));
      }
      "filter" => {
        // 吐出字符串
        final_properties.push(StyleValueType::Expr(Expr::new(
          CSSPropertyType::Filter,
          generate_expr_lit_str!(value.value_to_css_string(PrinterOptions::default()).unwrap()),
        )));
      }
      "backdropFilter" => {
        final_properties.push(StyleValueType::Expr(Expr::new(
          CSSPropertyType::BackdropFilter,
          generate_expr_lit_str!(value.value_to_css_string(PrinterOptions::default()).unwrap()),
        )));
      }
      _ => {
        // position、zIndex等... 会自动处理 单位、数字等相关信息（注册表已覆盖大多数场景）
        // final_properties.push(StyleValueType::Normal(Normal::new(id.to_string(), value.value_to_css_string(PrinterOptions::default()).unwrap())));
      }
    }
  }

  DeclsAndVars {
    has_env: has_env,
    vars: variable_properties,
    decls: final_properties,
  }
}
