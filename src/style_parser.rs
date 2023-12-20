use std::{cell::RefCell, collections::HashMap, convert::Infallible, hash::Hash, rc::Rc, vec};

use lightningcss::{
  declaration::DeclarationBlock,
  properties::{Property, border::{LineStyle, BorderSideWidth}},
  rules::CssRule,
  stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
  targets::{Features, Targets},
  traits::ToCss,
  visit_types,
  visitor::{Visit, VisitTypes, Visitor}, values::color::CssColor,
};

use crate::{
  document::JSXDocument,
  style_transform::{
    background::{
      background::Background,
      background_color::BackgroundColor,
      background_image::{BackgroundImage, BackgroundImageKind},
      background_position::BackgroundImagePosition,
      background_size::BackgroundImageSize,
      linear_gradient::LinearGradient,
    },
    flex_options::{
      flex_align::FlexAlign, flex_direction::FlexDirection, flex_options::FlexOptions,
      flex_wrap::FlexWrap, item_align::ItemAlign,
    },
    flex_size::{
      flex_basis::FlexBasis, flex_grow::FlexGrow, flex_shrink::FlexShrink, flex_size::FlexSize,
    },
    margin_padding::MarginPadding,
    style_value_type::StyleValueType,
    transform::transform::Transform, constraint_size::ConstraintSize, border::{border_width::{BorderWidth, parse_border_width_item}, border_color::{BorderColor, parse_border_color_item}, border_radius::BorderRadius, border_style::{BorderStyle, parse_border_style_item}, border::Border}, text::{line_height::LineHeight, letter_spacing::LetterSpacing, text_align::TextAlign, text_overflow::TextOverflow, font_weight::FontWeight, font_style::FontStyle, text_decoration::{TextDecoration}},
  },
  utils::{
    to_camel_case,
    prefix_style_key
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

  let mut flex_options = FlexOptions::new();
  let mut constrant_size = ConstraintSize::new();

  for (id, value) in properties.iter() {
    let property_name = id.as_str();
    match property_name {
      "margin" => {
        final_properties.insert(
          prefix_style_key("margin"),
          StyleValueType::MarginPadding(MarginPadding::from(value)),
        );
      }
      "marginTop" | "marginRight" | "marginBottom" | "marginLeft" => {
        let margin = final_properties
          .entry(prefix_style_key("margin"),)
          .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
        if let StyleValueType::MarginPadding(margin) = margin {
          let val = value
            .value_to_css_string(PrinterOptions::default())
            .unwrap();
          match property_name {
            "marginTop" => margin.set_top(val.as_str()),
            "marginRight" => margin.set_right(val.as_str()),
            "marginBottom" => margin.set_bottom(val.as_str()),
            "marginLeft" => margin.set_left(val.as_str()),
            _ => {}
          }
        }
      }
      "padding" => {
        final_properties.insert(
          prefix_style_key("padding"),
          StyleValueType::MarginPadding(MarginPadding::from(value)),
        );
      }
      "paddingTop" | "paddingRight" | "paddingBottom" | "paddingLeft" => {
        let padding = final_properties
          .entry(prefix_style_key("padding"))
          .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
        if let StyleValueType::MarginPadding(padding) = padding {
          let val = value
            .value_to_css_string(PrinterOptions::default())
            .unwrap();
          match property_name {
            "paddingTop" => padding.set_top(val.as_str()),
            "paddingRight" => padding.set_right(val.as_str()),
            "paddingBottom" => padding.set_bottom(val.as_str()),
            "paddingLeft" => padding.set_left(val.as_str()),
            _ => {}
          }
        }
      },
      "border" => {
        let border = Border::from(value);
        let border_width = final_properties
          .entry(prefix_style_key("borderWidth"))
          .or_insert(StyleValueType::BorderWidth(BorderWidth::new()));
        if let StyleValueType::BorderWidth(border_width) = border_width {
          if let Some(width) = border.width.top {
            border_width.set_all(width.as_str());
          }
        }
        let border_color = final_properties
          .entry(prefix_style_key("borderColor"))
          .or_insert(StyleValueType::BorderColor(BorderColor::new()));
        if let StyleValueType::BorderColor(border_color) = border_color {
          if let Some(color) = border.color.top {
            border_color.set_all(color.as_str());
          }
        }
        let border_style = final_properties
          .entry(prefix_style_key("borderStyle"))
          .or_insert(StyleValueType::BorderStyle(BorderStyle::new()));
        if let StyleValueType::BorderStyle(border_style) = border_style {
          if let Some(style) = border.style.top {
            border_style.set_all(style.as_str());
          }
        }
      },
      "borderLeft" | "borderRight" | "borderTop" | "borderBottom" => {
        let mut width: Option<BorderSideWidth> = None;
        let mut style: Option<LineStyle> = None;
        let mut color: Option<CssColor> = None;
        match &value {
          Property::BorderLeft(value) => {
            width = Some(value.width.to_owned());
            style = Some(value.style);
            color = Some(value.color.to_owned());
          },
          Property::BorderRight(value) => {
            width = Some(value.width.to_owned());
            style = Some(value.style);
            color = Some(value.color.to_owned());
          },
          Property::BorderTop(value) => {
            width = Some(value.width.to_owned());
            style = Some(value.style);
            color = Some(value.color.to_owned());
          },
          Property::BorderBottom(value) => {
            width = Some(value.width.to_owned());
            style = Some(value.style);
            color = Some(value.color.to_owned());
          },
          _ => {}
        };

        if let Some(width) = width {
          let width_item = parse_border_width_item(&width);
          if let Some(width_item) = width_item {
            let border_width = final_properties
              .entry(prefix_style_key("borderWidth"))
              .or_insert(StyleValueType::BorderWidth(BorderWidth::new()));
            if let StyleValueType::BorderWidth(border_width) = border_width {
              match property_name {
                "borderLeft" => border_width.set_left(width_item.left.unwrap().as_str()),
                "borderRight" => border_width.set_right(width_item.right.unwrap().as_str()),
                "borderTop" => border_width.set_top(width_item.top.unwrap().as_str()),
                "borderBottom" => border_width.set_bottom(width_item.bottom.unwrap().as_str()),
                _ => {}
              }
            }
          }
          
        }
        if let Some(color) = color {
          let color_item = parse_border_color_item(&color);
          if let Some(color_item) = color_item {
            let border_color = final_properties
              .entry(prefix_style_key("borderColor"))
              .or_insert(StyleValueType::BorderColor(BorderColor::new()));
            if let StyleValueType::BorderColor(border_color) = border_color {
              match property_name {
                "borderLeft" => border_color.set_left(color_item.left.unwrap().as_str()),
                "borderRight" => border_color.set_right(color_item.right.unwrap().as_str()),
                "borderTop" => border_color.set_top(color_item.top.unwrap().as_str()),
                "borderBottom" => border_color.set_bottom(color_item.bottom.unwrap().as_str()),
                _ => {}
              }
            }
          }
          
        }
        if let Some(style) = style {
          let style_item = parse_border_style_item(&style);
          if let Some(style_item) = style_item {
              let border_style = final_properties
                .entry(prefix_style_key("borderStyle"))
                .or_insert(StyleValueType::BorderStyle(BorderStyle::new()));
            if let StyleValueType::BorderStyle(border_style) = border_style {
              match property_name {
                "borderLeft" => border_style.set_left(style_item.left.unwrap().as_str()),
                "borderRight" => border_style.set_right(style_item.right.unwrap().as_str()),
                "borderTop" => border_style.set_top(style_item.top.unwrap().as_str()),
                "borderBottom" => border_style.set_bottom(style_item.bottom.unwrap().as_str()),
                _ => {}
              }
            }
          }
        }
      },
      "borderWidth" => {
        let border_width = BorderWidth::from(value);
        if border_width.is_zero() {
          final_properties.remove("borderWidth");
        } else {
          final_properties.insert(
            prefix_style_key("borderWidth"),
            StyleValueType::BorderWidth(border_width)
          );
        }
      },
      "borderTopWidth" | "borderRightWidth" | "borderBottomWidth" | "borderLeftWidth" => {
        let border_width = final_properties
          .entry(prefix_style_key("borderWidth"))
          .or_insert(StyleValueType::BorderWidth(BorderWidth::new()));
        if let StyleValueType::BorderWidth(border_width) = border_width {
          let val = value
            .value_to_css_string(PrinterOptions::default())
            .unwrap();
          match property_name {
            "borderTopWidth" => border_width.set_top(val.as_str()),
            "borderRightWidth" => border_width.set_right(val.as_str()),
            "borderBottomWidth" => border_width.set_bottom(val.as_str()),
            "borderLeftWidth" => border_width.set_left(val.as_str()),
            _ => {}
          }
        }
      },
      "borderColor" => {
        let border_color = BorderColor::from(value);
        if border_color.is_zero() {
          final_properties.remove("borderColor");
        } else {
          final_properties.insert(
            prefix_style_key("borderColor"),
            StyleValueType::BorderColor(border_color)
          );
        }
      },
      "borderTopColor" | "borderRightColor" | "borderBottomColor" | "borderLeftColor" => {
        let border_color: &mut StyleValueType = final_properties
          .entry(prefix_style_key("borderColor"))
          .or_insert(StyleValueType::BorderColor(BorderColor::new()));
        if let StyleValueType::BorderColor(border_color) = border_color {
          let val = value
            .value_to_css_string(PrinterOptions {
              minify: false,
              targets: Targets {
                include: Features::HexAlphaColors,
                ..Targets::default()
              },
              ..PrinterOptions::default()
            })
            .unwrap();
          match property_name {
            "borderTopColor" => border_color.set_top(val.as_str()),
            "borderRightColor" => border_color.set_right(val.as_str()),
            "borderBottomColor" => border_color.set_bottom(val.as_str()),
            "borderLeftColor" => border_color.set_left(val.as_str()),
            _ => {}
          }
        }
      },
      "borderStyle" => {
        let border_style: BorderStyle = BorderStyle::from(value);
        if border_style.is_zero() {
          final_properties.remove("borderStyle");
        } else {
          final_properties.insert(
            prefix_style_key("borderStyle"),
            StyleValueType::BorderStyle(border_style)
          );
        }
      },
      "borderTopStyle" | "borderRightStyle" | "borderBottomStyle" | "borderLeftStyle" => {
        let border_style = final_properties
          .entry(prefix_style_key("borderStyle"))
          .or_insert(StyleValueType::BorderStyle(BorderStyle::new()));
        if let StyleValueType::BorderStyle(border_style) = border_style {
          let val = value
            .value_to_css_string(PrinterOptions::default())
            .unwrap();
          match property_name {
            "borderTopStyle" => border_style.set_top(val.as_str()),
            "borderRightStyle" => border_style.set_right(val.as_str()),
            "borderBottomStyle" => border_style.set_bottom(val.as_str()),
            "borderLeftStyle" => border_style.set_left(val.as_str()),
            _ => {}
          }
        }
      },
      "borderRadius" => {
        let border_radius = BorderRadius::from(value);
        if border_radius.is_zero() {
          final_properties.remove("borderRadius");
        } else {
          final_properties.insert(
            prefix_style_key("borderRadius"),
            StyleValueType::BorderRadius(border_radius),
          );
        }
      }
      "borderTopLeftRadius" | "borderTopRightRadius" | "borderBottomLeftRadius" | "borderBottomRightRadius" => {
        let border_radius = final_properties
          .entry(prefix_style_key("borderRadius"))
          .or_insert(StyleValueType::BorderRadius(BorderRadius::new()));
        if let StyleValueType::BorderRadius(border_radius) = border_radius {
          let val = value
            .value_to_css_string(PrinterOptions::default())
            .unwrap();
          match property_name {
            "borderTopLeftRadius" => border_radius.set_top_left(val.as_str()),
            "borderTopRightRadius" => border_radius.set_top_right(val.as_str()),
            "borderBottomLeftRadius" => border_radius.set_bottom_left(val.as_str()),
            "borderBottomRightRadius" => border_radius.set_bottom_right(val.as_str()),
            _ => {}
          }
        }
      }
      "lineHeight" => {
        let line_height = LineHeight::from(value);
        match line_height {
          LineHeight::Px(_) => {
            final_properties.insert(
              prefix_style_key("lineHeight"),
              StyleValueType::LineHeight(line_height),
            );
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
        let text_decoration = TextDecoration::from(value);
        final_properties.insert(
          prefix_style_key("decoration"),
          StyleValueType::TextDecoration(text_decoration),
        );
      }
      "color" => {
        final_properties.insert(
          prefix_style_key(id),
          StyleValueType::Color(
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
      "background" => {
        let mut background = Background::from(value);
        let mut images = vec![];
        let mut linear_gradient = vec![];
        for item in background.image.0.iter() {
          if let BackgroundImageKind::String(_) = &item.image {
            images.push(item.clone());
          } else if let BackgroundImageKind::LinearGradient(gradient) = &item.image {
            linear_gradient.push(gradient.clone());
          }
        }
        final_properties.remove("background");
        final_properties.remove("linearGradient");
        if images.len() > 0 {
          background.image = BackgroundImage(images);
          final_properties.insert(prefix_style_key(id), StyleValueType::Background(background));
        } else if background.color.0 != "" {
          final_properties.insert(
            prefix_style_key(id),
            StyleValueType::Background(Background {
              color: background.color,
              image: BackgroundImage(vec![]),
              position: BackgroundImagePosition(vec![]),
              size: BackgroundImageSize(vec![]),
            }),
          );
        }
        if linear_gradient.len() > 0 {
          final_properties.insert(
            prefix_style_key("linearGradient"),
            StyleValueType::LinearGradient(LinearGradient(linear_gradient)),
          );
        }
      }
      "backgroundColor" => {
        let background = final_properties
          .entry(prefix_style_key("background"))
          .or_insert(StyleValueType::Background(Background::new()));
        if let StyleValueType::Background(background) = background {
          let color = BackgroundColor::from(value);
          if color.0 != "" {
            background.color = color;
          }
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
          let background = final_properties
            .entry(prefix_style_key("background"))
            .or_insert(StyleValueType::Background(Background::new()));
          if let StyleValueType::Background(background) = background {
            background.image = BackgroundImage(images);
          }
        }
        if linear_gradient.len() > 0 {
          final_properties.insert(
            prefix_style_key("linearGradient"),
            StyleValueType::LinearGradient(LinearGradient(linear_gradient)),
          );
        }
      }
      "backgroundPosition" => {
        let background_position = BackgroundImagePosition::from(value);
        if background_position.0.len() > 0 {
          let background = final_properties
            .entry(prefix_style_key("background"))
            .or_insert(StyleValueType::Background(Background::new()));
          if let StyleValueType::Background(background) = background {
            background.position = background_position;
          }
        }
      }
      "backgroundSize" => {
        let background_size = BackgroundImageSize::from(value);
        if background_size.0.len() > 0 {
          let background = final_properties
            .entry(prefix_style_key("background"))
            .or_insert(StyleValueType::Background(Background::new()));
          if let StyleValueType::Background(background) = background {
            background.size = background_size
          }
        }
      }
      "backgroundRepeat" => {}
      "flexDirection" => flex_options.direction = Some(FlexDirection::from(value)),
      "flexWrap" => flex_options.wrap = Some(FlexWrap::from(value)),
      "justifyContent" => flex_options.justify_content = Some(FlexAlign::from(value)),
      "alignItems" => {
        let value = ItemAlign::from(value);
        flex_options.align_items = if value == ItemAlign::Ignore {
          None
        } else {
          Some(value)
        }
      }
      "alignContent" => flex_options.align_content = Some(FlexAlign::from(value)),
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
      "minHeight" => {
        constrant_size.min_height = Some(
          value.value_to_css_string(PrinterOptions::default()).unwrap(),
        );
      }
      "minWidth" => {
        constrant_size.min_width = Some(
          value.value_to_css_string(PrinterOptions::default()).unwrap(),
        );
      }
      "maxHeight" => {
        constrant_size.max_height = Some(
          value.value_to_css_string(PrinterOptions::default()).unwrap(),
        );
      }
      "maxWidth" => {
        constrant_size.max_width = Some(
          value.value_to_css_string(PrinterOptions::default()).unwrap(),
        );
      }
      "alignSelf" => {
        let align_self = ItemAlign::from(value);
        if align_self != ItemAlign::Ignore {
          final_properties.insert(prefix_style_key(id), StyleValueType::AlignSelf(align_self));
        }
      }
      "transform" => {
        let transform_origin = properties
          .iter()
          .find(|(id, _)| id == "transformOrigin")
          .and_then(|p| {
            if let Property::TransformOrigin(value, _) = &p.1 {
              Some(value)
            } else {
              None
            }
          });
        let transform = Transform::from((value, transform_origin));
        if transform.translate.0.len() > 0 {
          final_properties.insert(
            prefix_style_key("translate"),
            StyleValueType::Translates(transform.translate),
          );
        }
        if transform.rotate.0.len() > 0 {
          final_properties.insert(
            prefix_style_key("rotate"),
            StyleValueType::Rotates(transform.rotate),
          );
        }
        if transform.scale.0.len() > 0 {
          final_properties.insert(prefix_style_key("scale"), StyleValueType::Scales(transform.scale));
        }
        if transform.matrix.0.len() > 0 {
          final_properties.insert(
            prefix_style_key("matrix"),
            StyleValueType::Matrices(transform.matrix),
          );
        }
      },
      "height" | "width" | "fontSize" | "top" | "left" | "bottom" | "right" => {
        final_properties.insert(prefix_style_key(property_name), StyleValueType::Normal(
          value.value_to_css_string(PrinterOptions::default()).unwrap()
        ));
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

  if constrant_size.max_height.is_some()
    || constrant_size.max_width.is_some()
    || constrant_size.min_height.is_some()
    || constrant_size.min_width.is_some()
  {
    final_properties.insert(
      prefix_style_key("constraintSize"),
      StyleValueType::ConstraintSize(constrant_size),
    );
  }

  if flex_options.direction.is_some()
    || flex_options.wrap.is_some()
    || flex_options.justify_content.is_some()
    || flex_options.align_items.is_some()
    || flex_options.align_content.is_some()
  {
    final_properties.insert(
      prefix_style_key("flexOptions"),
      StyleValueType::FlexOptions(flex_options),
    );
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
