use std::{cell::RefCell, collections::HashMap, convert::Infallible, hash::Hash, rc::Rc, vec};

use lightningcss::{
  declaration::DeclarationBlock,
  properties::Property,
  rules::CssRule,
  stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
  targets::{Features, Targets},
  traits::ToCss,
  visit_types,
  visitor::{Visit, VisitTypes, Visitor},
};

use crate::{
  document::JSXDocument,
  style_transform::{
    background::{
      // background::Background,
      background_image::{BackgroundImage, BackgroundImageKind},
      background_position::BackgroundPosition,
      background_size::BackgroundSize,
      linear_gradient::LinearGradient, background_repeat::BackgroundRepeat, background_color::BackgroundColor, background::Background,
    },
    flex_options::{
      flex_align::FlexAlign, flex_direction::FlexDirection,
      flex_wrap::FlexWrap, item_align::ItemAlign,
    },
    flex_size::{
      flex_basis::FlexBasis, flex_grow::FlexGrow, flex_shrink::FlexShrink, flex_size::FlexSize,
    },
    margin_padding::MarginPadding,
    style_value_type::StyleValueType,
    transform::{transform::Transform, transform_origin::TransformOrigin}, border::{border_width::BorderWidth, border_color::BorderColor, border_radius::BorderRadius, border_style::{BorderStyle, BorderStyleType}, border::Border}, text::{line_height::LineHeight, letter_spacing::LetterSpacing, text_align::TextAlign, text_overflow::TextOverflow, font_weight::FontWeight, font_style::FontStyle, text_decoration::TextDecoration},
  },
  utils::{
    to_camel_case,
    prefix_style_key, color_string
  },
  visitor::SpanKey,
};

pub type StyleValue = HashMap<String, StyleValueType>;

pub struct StyleData<'i> {
  pub style_record: Rc<RefCell<HashMap<SpanKey, Vec<(String, Property<'i>)>>>>,
  pub all_style: Rc<RefCell<HashMap<String, StyleValue>>>,
}

#[derive(Debug, Clone)]
pub struct StyleDeclaration<'i> {
  pub specificity: u32,
  pub declaration: DeclarationBlock<'i>,
}

pub struct StyleVisitor<'i> {
  pub all_style: Rc<RefCell<Vec<(String, Vec<StyleDeclaration<'i>>)>>>
}

impl<'i> StyleVisitor<'i> {
  pub fn new(
    all_style: Rc<RefCell<Vec<(String, Vec<StyleDeclaration<'i>>)>>>,
  ) -> Self {
    StyleVisitor {
      all_style
    }
  }
}

// 收集所有的样式到 all_style 中，以元祖的形式存在 (selector, vec[declaration1, declaration2, ...])
impl<'i> Visitor<'i> for StyleVisitor<'i> {
  type Error = Infallible;
  const TYPES: VisitTypes = visit_types!(RULES);
  fn visit_rule(&mut self, rule: &mut CssRule<'i>) -> Result<(), Self::Error> {
    match rule {
      CssRule::Style(style) => {
        let selectors_str = style.selectors.to_string();
        let selectors = selectors_str.split(",").collect::<Vec<&str>>();
        for index in 0..selectors.len() {
          let selector = selectors[index].trim().replace(".", "");
          let mut all_style = self.all_style.borrow_mut();
          let decorations = all_style.iter_mut().find(|(id, _)| id == &selector);
          if let Some((_, declarations)) = decorations {
            declarations.push(StyleDeclaration {
              specificity: style.selectors.0.get(index).unwrap().specificity(),
              declaration: style.declarations.clone(),
            });
          } else {
            all_style.push((
              selector.clone(),
              vec![StyleDeclaration {
                specificity: style.selectors.0.get(index).unwrap().specificity(),
                declaration: style.declarations.clone(),
              }],
            ));
          }
        }
      }
      _ => {}
    }
    Ok(())
  }
}

pub fn parse_style_properties(properties: &Vec<(String, Property<'_>)>) -> StyleValue {
  let mut final_properties = HashMap::new();

  for (id, value) in properties.iter() {
    let property_name = id.as_str();
    match property_name {
      "margin" => {
        let margin = MarginPadding::from(value);
        margin.top.map(|top| {
          final_properties.insert(prefix_style_key("marginTop"), top);
        });
        margin.bottom.map(|bottom| {
          final_properties.insert(prefix_style_key("marginBottom"), bottom);
        });
        margin.left.map(|left| {
          final_properties.insert(prefix_style_key("marginLeft"), left);
        });
        margin.right.map(|right| {
          final_properties.insert(prefix_style_key("marginRight"), right);
        });
      }
      "marginTop" | "marginRight" | "marginBottom" | "marginLeft" => {
        let css_string = value.value_to_css_string(PrinterOptions::default()).unwrap();
        final_properties.insert(prefix_style_key(property_name), StyleValueType::Length(css_string));
      }
      "padding" => {
        let padding = MarginPadding::from(value);
        padding.top.map(|top| {
          final_properties.insert(prefix_style_key("paddingTop"), top);
        });
        padding.bottom.map(|bottom| {
          final_properties.insert(prefix_style_key("paddingBottom"), bottom);
        });
        padding.left.map(|left| {
          final_properties.insert(prefix_style_key("paddingLeft"), left);
        });
        padding.right.map(|right| {
          final_properties.insert(prefix_style_key("paddingRight"), right);
        });
      }
      "paddingTop" | "paddingRight" | "paddingBottom" | "paddingLeft" => {
        let css_string = value.value_to_css_string(PrinterOptions::default()).unwrap();
        final_properties.insert(prefix_style_key(property_name), StyleValueType::Length(css_string));
      }
      "border" => {
        let border = Border::from(value);
        border.width.top.map(|top| {
          final_properties.insert(prefix_style_key("borderTopWidth"), top);
        });
        border.width.bottom.map(|bottom| {
          final_properties.insert(prefix_style_key("borderBottomWidth"), bottom);
        });
        border.width.left.map(|left| {
          final_properties.insert(prefix_style_key("borderLeftWidth"), left);
        });
        border.width.right.map(|right| {
          final_properties.insert(prefix_style_key("borderRightWidth"), right);
        });
        border.color.top.map(|top| {
          final_properties.insert(prefix_style_key("borderTopColor"), top);
        });
        border.color.bottom.map(|bottom| {
          final_properties.insert(prefix_style_key("borderBottomColor"), bottom);
        });
        border.color.left.map(|left| {
          final_properties.insert(prefix_style_key("borderLeftColor"), left);
        });
        border.color.right.map(|right| {
          final_properties.insert(prefix_style_key("borderRightColor"), right);
        });
        border.style.top.map(|top| {
          final_properties.insert(prefix_style_key("borderTopStyle"), top);
        });
        border.style.bottom.map(|bottom| {
          final_properties.insert(prefix_style_key("borderBottomStyle"), bottom);
        });
        border.style.left.map(|left| {
          final_properties.insert(prefix_style_key("borderLeftStyle"), left);
        });
        border.style.right.map(|right| {
          final_properties.insert(prefix_style_key("borderRightStyle"), right);
        });
      }
      "borderLeft" | "borderRight" | "borderTop" | "borderBottom" => {
        let border: Border = Border::from(value);
        match property_name {
          "borderLeft" => {
            border.width.left.map(|left| {
              final_properties.insert(prefix_style_key("borderLeftWidth"), left);
            });
            border.color.left.map(|left| {
              final_properties.insert(prefix_style_key("borderLeftColor"), left);
            });
            border.style.left.map(|left| {
              final_properties.insert(prefix_style_key("borderLeftStyle"), left);
            });
          },
          "borderRight" => {
            border.width.right.map(|right| {
              final_properties.insert(prefix_style_key("borderRightWidth"), right);
            });
            border.color.right.map(|right| {
              final_properties.insert(prefix_style_key("borderRightColor"), right);
            });
            border.style.right.map(|right| {
              final_properties.insert(prefix_style_key("borderRightStyle"), right);
            });
          },
          "borderTop" => {
            border.width.top.map(|top| {
              final_properties.insert(prefix_style_key("borderTopWidth"), top);
            });
            border.color.top.map(|top| {
              final_properties.insert(prefix_style_key("borderTopColor"), top);
            });
            border.style.top.map(|top| {
              final_properties.insert(prefix_style_key("borderTopStyle"), top);
            });
          },
          "borderBottom" => {
            border.width.bottom.map(|bottom| {
              final_properties.insert(prefix_style_key("borderBottomWidth"), bottom);
            });
            border.color.bottom.map(|bottom| {
              final_properties.insert(prefix_style_key("borderBottomColor"), bottom);
            });
            border.style.bottom.map(|bottom| {
              final_properties.insert(prefix_style_key("borderBottomStyle"), bottom);
            });
          },
          _ => {}
            
        }
      }
      "borderWidth" => {
        let border = BorderWidth::from(value);
        border.top.map(|top| {
          final_properties.insert(prefix_style_key("borderTopWidth"), top);
        });
        border.bottom.map(|bottom| {
          final_properties.insert(prefix_style_key("borderBottomWidth"), bottom);
        });
        border.left.map(|left| {
          final_properties.insert(prefix_style_key("borderLeftWidth"), left);
        });
        border.right.map(|right| {
          final_properties.insert(prefix_style_key("borderRightWidth"), right);
        });
      }
      "borderTopWidth" | "borderRightWidth" | "borderBottomWidth" | "borderLeftWidth" => {
        let css_string = value.value_to_css_string(PrinterOptions::default()).unwrap();
        final_properties.insert(prefix_style_key(property_name), StyleValueType::Length(css_string));
      }
      "borderColor" => {
        let border_color = BorderColor::from(value);
        border_color.top.map(|top| {
          final_properties.insert(prefix_style_key("borderTopColor"), top);
        });
        border_color.bottom.map(|bottom| {
          final_properties.insert(prefix_style_key("borderBottomColor"), bottom);
        });
        border_color.left.map(|left| {
          final_properties.insert(prefix_style_key("borderLeftColor"), left);
        });
        border_color.right.map(|right| {
          final_properties.insert(prefix_style_key("borderRightColor"), right);
        });
      }
      "borderTopColor" | "borderRightColor" | "borderBottomColor" | "borderLeftColor" => {
        let css_string = value.value_to_css_string(PrinterOptions {
          minify: false,
          targets: Targets {
            include: Features::HexAlphaColors,
            ..Targets::default()
          },
          ..PrinterOptions::default()
        }).unwrap();
        final_properties.insert(prefix_style_key(property_name), StyleValueType::Color(css_string));
      }
      "borderStyle" => {
        let border_style: BorderStyle = BorderStyle::from(value);
        border_style.top.map(|top| {
          final_properties.insert(prefix_style_key("borderTopStyle"), top);
        });
        border_style.bottom.map(|bottom| {
          final_properties.insert(prefix_style_key("borderBottomStyle"), bottom);
        });
        border_style.left.map(|left| {
          final_properties.insert(prefix_style_key("borderLeftStyle"), left);
        });
        border_style.right.map(|right| {
          final_properties.insert(prefix_style_key("borderRightStyle"), right);
        });
      }
      "borderTopStyle" | "borderRightStyle" | "borderBottomStyle" | "borderLeftStyle" => {
        let border_style = BorderStyleType::from(value);
        final_properties.insert(prefix_style_key(property_name), StyleValueType::BorderStyleType(border_style));
      }
      "borderRadius" => {
        let border_radius = BorderRadius::from(value);
        border_radius.top_left.map(|top_left| {
          final_properties.insert(prefix_style_key("borderTopLeftRadius"), top_left);
        });
        border_radius.top_right.map(|top_right| {
          final_properties.insert(prefix_style_key("borderTopRightRadius"), top_right);
        });
        border_radius.bottom_left.map(|bottom_left| {
          final_properties.insert(prefix_style_key("borderBottomLeftRadius"), bottom_left);
        });
        border_radius.bottom_right.map(|bottom_right| {
          final_properties.insert(prefix_style_key("borderBottomRightRadius"), bottom_right);
        });
      }
      "borderTopLeftRadius" | "borderTopRightRadius" | "borderBottomLeftRadius" | "borderBottomRightRadius" => {
        let css_string = value.value_to_css_string(PrinterOptions::default()).unwrap();
        final_properties.insert(prefix_style_key(property_name), StyleValueType::Length(css_string));
      }
      "lineHeight" => {
        let line_height = LineHeight::from(value);
        match line_height {
          LineHeight::Px(_) => {
            final_properties.insert(prefix_style_key(property_name),StyleValueType::LineHeight(line_height));
          }
          _ => {}
        }
      }
      "letterSpacing" => {
        let letter_spacing = LetterSpacing::from(value);
        match letter_spacing {
          LetterSpacing::Px(_) => {
            final_properties.insert(
              prefix_style_key("letterSpacing"),
              StyleValueType::LetterSpacing(letter_spacing),
            );
          }
          _ => {}
        }
      }
      "textAlign" => {
        let text_align = TextAlign::from(value);
        final_properties.insert(
          prefix_style_key("textAlign"),
          StyleValueType::TextAlign(text_align),
        );
      }
      "textOverflow" => {
        let text_overflow = TextOverflow::from(value);
        final_properties.insert(
          prefix_style_key("textOverflow"),
          StyleValueType::TextOverflow(text_overflow),
        );
      }
      "fontWeight" => {
        let font_weight = FontWeight::from(value);
        final_properties.insert(
          prefix_style_key("fontWeight"),
          StyleValueType::FontWeight(font_weight),
        );
      }
      "fontStyle" => {
        let font_style = FontStyle::from(value);
        final_properties.insert(
          prefix_style_key("fontStyle"),
          StyleValueType::FontStyle(font_style),
        );
      }
      "textDecoration" => {
        final_properties.insert(
          prefix_style_key(property_name),
          StyleValueType::TextDecoration(TextDecoration::from(value)),
        );
      }
      "color" => {
        final_properties.insert(
          prefix_style_key(property_name),
          StyleValueType::Color(color_string(value))
        );
      }
      "background" => {
        let background = Background::from(value);
        final_properties.remove("background");

        if let Some(image) = background.image {
          final_properties.insert(prefix_style_key("backgroundImage"), StyleValueType::BackgroundImage(image));
          if let Some(size) = background.size {
            final_properties.insert(prefix_style_key("backgroundSize"), StyleValueType::BackgroundSize(size));
          }
          if let Some(position) = background.position {
            final_properties.insert(prefix_style_key("backgroundPosition"), StyleValueType::BackgroundPosition(position));
          }
          if let Some(repeat) = background.repeat {
            final_properties.insert(prefix_style_key("backgroundRepeat"), StyleValueType::BackgroundRepeat(repeat));
          }
        } else if let Some(color) = background.color {
          final_properties.insert(prefix_style_key("backgroundColor"), StyleValueType::BackgroundColor(color));
        }
      }
      "backgroundColor" => {
        let background_color = BackgroundColor::from(value);
        if background_color.0.len() > 0 {
          final_properties.insert(
            prefix_style_key(property_name),
            StyleValueType::BackgroundColor(background_color)
          );
        }
      }
      "backgroundImage" => {
        let mut repeat = None;
        if let Some(value) = properties.iter().find(|(id, _)| id == "backgroundRepeat") {
          if let Property::BackgroundRepeat(_) = &value.1 {
            repeat = Some(&value.1);
          }
        }

        let background_image = BackgroundImage::from((value, repeat));
        let mut images = vec![];
        let mut linear_gradient = vec![];
        for item in background_image.0.iter() {
          if let BackgroundImageKind::String(_) = &item.image {
            images.push(item.clone());
          } else if let BackgroundImageKind::LinearGradient(gradient) = &item.image {
            linear_gradient.push(gradient.clone());
          }
        }
        if images.len() > 0 {
          final_properties.insert(
            prefix_style_key(property_name),
            StyleValueType::BackgroundImage(background_image)
          );
        }
        if linear_gradient.len() > 0 {
          final_properties.insert(
            prefix_style_key(property_name),
            StyleValueType::LinearGradient(LinearGradient(linear_gradient)),
          );
        }
      }
      "backgroundPosition" => {
        let background_position = BackgroundPosition::from(value);
        if background_position.0.len() > 0 {
          final_properties.insert(
            prefix_style_key(property_name),
            StyleValueType::BackgroundPosition(background_position)
          );
        }
      }
      "backgroundSize" => {
        let background_size = BackgroundSize::from(value);
        if background_size.0.len() > 0 {
          final_properties.insert(
            prefix_style_key(property_name),
            StyleValueType::BackgroundSize(background_size)
          );
        }
      }
      "backgroundRepeat" => {
        let background_repeat = BackgroundRepeat::from(value);
        if background_repeat.0.len() > 0 {
          final_properties.insert(
            prefix_style_key(property_name),
            StyleValueType::BackgroundRepeat(background_repeat)
          );
        }
      }
      "flexDirection" => {
        let flex_direction = FlexDirection::from(value);
        final_properties.insert(prefix_style_key(property_name), StyleValueType::FlexDirection(flex_direction));
      }
      "flexWrap" => {
        let flex_wrap = FlexWrap::from(value);
        final_properties.insert(prefix_style_key(property_name), StyleValueType::FlexWrap(flex_wrap));
      }
      "justifyContent" => {
        let justify_content = FlexAlign::from(value);
        final_properties.insert(prefix_style_key(property_name), StyleValueType::JustifyContent(justify_content));
      }
      "alignItems" => {
        let align_items = ItemAlign::from(value);
        final_properties.insert(prefix_style_key(property_name), StyleValueType::ItemAlign(align_items));
      }
      "alignContent" => {
        let align_content = FlexAlign::from(value);
        final_properties.insert(prefix_style_key(property_name), StyleValueType::AlignContent(align_content));

      }
      "flex" => {
        let flex_size = FlexSize::from(value);
        let flex_grow = flex_size.grow;
        let flex_shrink = flex_size.shrink;
        let flex_basis = flex_size.basis;
        if let Some(flex_grow) = flex_grow {
          final_properties.insert(prefix_style_key("flexGrow"), StyleValueType::FlexGrow(FlexGrow::from(flex_grow)));
        }
        if let Some(flex_shrink) = flex_shrink {
          final_properties.insert(
            prefix_style_key("flexShrink"),
            StyleValueType::FlexShrink(FlexShrink::from(flex_shrink)),
          );
        }
        if let Some(flex_basis) = flex_basis {
          final_properties.insert(
            prefix_style_key("flexBasis"),
            StyleValueType::FlexBasis(FlexBasis::from(flex_basis)),
          );
        }
      }
      "flexGrow" => {
        let flex_grow = FlexGrow::from(value);
        final_properties.insert(prefix_style_key(id), StyleValueType::FlexGrow(flex_grow));
      }
      "flexShrink" => {
        let flex_shrink = FlexShrink::from(value);
        final_properties.insert(prefix_style_key(id), StyleValueType::FlexShrink(flex_shrink));
      }
      "flexBasis" => {
        let flex_basis = FlexBasis::from(value);
        final_properties.insert(prefix_style_key(id), StyleValueType::FlexBasis(flex_basis));
      }
      "alignSelf" => {
        let align_self = ItemAlign::from(value);
        if align_self != ItemAlign::Ignore {
          final_properties.insert(prefix_style_key(id), StyleValueType::AlignSelf(align_self));
        }
      }
      "transform" => {
        final_properties.insert(
          prefix_style_key("transform"),
          StyleValueType::Transform(Transform::from(value)),
        );
      }
      "transformOrigin" => {
        final_properties.insert(
          prefix_style_key("transformOrigin"),
          StyleValueType::TransformOrigin(TransformOrigin::from(value)),
        );
      }
      "height" | "width" | "minHeight" | "maxHeight" | "minWidth" | "maxWidth" | "fontSize" | "top" | "left" | "bottom" | "right" => {
        final_properties.insert(prefix_style_key(property_name), StyleValueType::Px(
          value.value_to_css_string(PrinterOptions::default()).unwrap()
        ));
      }
      _ => {
        final_properties.insert(
          id.to_string(),
          StyleValueType::Normal(
            value
              .value_to_css_string(PrinterOptions {
                minify: false,
                targets: Targets {
                  include: Features::HexAlphaColors,
                  ..Targets::default()
                },
                ..PrinterOptions::default()
              })
              .unwrap(),
          ),
        );
      }
    }
  }

  final_properties
}

pub struct StyleParser<'i> {
  pub all_style: Rc<RefCell<Vec<(String, Vec<StyleDeclaration<'i>>)>>>,
  pub document: &'i JSXDocument,
}

impl<'i> StyleParser<'i> {
  pub fn new(document: &'i JSXDocument) -> Self {
    StyleParser {
      all_style: Rc::new(RefCell::new(vec![])),
      document,
    }
  }

  pub fn parse(&mut self, css: &'i str) {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default()).expect("解析样式失败");
    let mut style_visitor = StyleVisitor::new(Rc::clone(&self.all_style));
    stylesheet.visit(&mut style_visitor).unwrap();
  }

  // 合并相同类型的 style，比如 .a { color: red } .a { color: blue } => .a { color: blue }，并且 !important 的优先级高于普通的
  fn calc_style_record<T: Hash + Eq + Clone>(
    &self,
    style_record: &mut Vec<(T, Vec<StyleDeclaration<'i>>)>,
  ) -> Vec<(T, StyleDeclaration<'i>)> {
    let mut final_style_record = vec![];
    for (id, declarations) in style_record.iter_mut() {
      declarations.sort_by(|a, b| a.specificity.cmp(&b.specificity));
      let mut final_properties: Vec<Property<'i>> = Vec::new();
      for declaration in declarations.iter() {
        let declaration = &declaration.declaration;
        let declarations = &declaration.declarations;
        for declaration in declarations.iter() {
          let has_property_index = final_properties
            .iter()
            .position(|property| property.property_id() == declaration.property_id());
          if let Some(index) = has_property_index {
            final_properties[index] = declaration.clone();
          } else {
            final_properties.push(declaration.clone());
          }
        }
      }
      for declaration in declarations.iter() {
        let declaration = &declaration.declaration;
        let important_declarations = &declaration.important_declarations;
        for declaration in important_declarations.iter() {
          let has_property_index = final_properties
            .iter()
            .position(|property| property.property_id() == declaration.property_id());
          if let Some(index) = has_property_index {
            final_properties[index] = declaration.clone();
          } else {
            final_properties.push(declaration.clone());
          }
        }
      }
      final_style_record.push((
        (*id).clone(),
        StyleDeclaration {
          specificity: 0,
          declaration: DeclarationBlock {
            declarations: final_properties,
            important_declarations: vec![],
          },
        },
      ));
    }
    final_style_record
  }

  pub fn calc(&self) -> StyleData<'i> {
    // 遍历 style_record，计算每个节点的最终样式
    let mut all_style = self.all_style.borrow_mut();
    let mut style_record = HashMap::new();
    let mut final_all_style = self.calc_style_record(&mut all_style);

    // final_all_style 转换为驼峰命名
    let mut final_all_style = final_all_style
      .iter_mut()
      .map(|(selector, style_value)| {
        let properties = style_value
          .declaration
          .declarations
          .iter()
          .map(|property| {
            (
              to_camel_case(
                property
                  .property_id()
                  .to_css_string(PrinterOptions::default())
                  .unwrap()
                  .as_str(),
                false,
              ),
              property.clone(),
            )
          })
          .collect::<Vec<(_, _)>>();
        // (selector.to_owned(), parse_style_properties(&properties))
        (selector.to_owned(), properties)
      })
      .collect::<Vec<(_, _)>>();

    // 将所有相同 selector 的 style 块合并，所有 style 块的 declarations 都放在都以 selector 对应的 element span 作为 key 放在 style_record 这个 HashMap 中
    for (selector, style_value) in final_all_style.iter_mut() {
      let elements = self.document.select(selector);
      for element in elements {
        let declarations = style_record.entry(element.span).or_insert(vec![]);
        declarations.push(style_value.clone());
      }
    }
    let final_style_record = style_record
      .iter_mut()
      .map(|(selector, style_value)| {
        (
          *selector,
          // calc_style_record 函数里只是做单个 style 块的去重，这里需要对所有相同 selector 的 style 块进行去重
          style_value
            .iter_mut()
            .reduce(|a, b| {
              for (key, value) in b.iter() {
                let has_property_index = a.iter().position(|property| property.0 == key.to_owned());
                if let Some(index) = has_property_index {
                  a[index] = (key.to_owned(), value.clone());
                } else {
                  a.push((key.to_owned(), value.clone()));
                }
              }
              a
            })
            .unwrap()
            .to_owned(),
        )
      })
      .collect::<HashMap<_, _>>();
    // 进行样式解析优化，提前解析 ArkUI 的样式，减少运行时的计算
    let final_all_style = final_all_style
      .iter_mut()
      .map(|(selector, properties)| {
        (
          selector.to_owned(),
          parse_style_properties(
            &properties
              .iter()
              .map(|(k, v)| (k.to_owned(), v.clone()))
              .collect::<Vec<_>>(),
          ),
        )
      })
      .collect::<HashMap<_, _>>();
    StyleData {
      style_record: Rc::new(RefCell::new(final_style_record)),
      all_style: Rc::new(RefCell::new(final_all_style)),
    }
  }
}
