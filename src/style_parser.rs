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
      background::Background,
      background_color::BackgroundColor,
      background_image::{BackgroundImage, BackgroundImageKind},
      background_position::BackgroundImagePosition,
      background_size::BackgroundImageSize,
      linear_gradient::LinearGradient,
    },
    border_radius::BorderRadius,
    flex_options::{
      flex_align::FlexAlign, flex_direction::FlexDirection, flex_options::FlexOptions,
      flex_wrap::FlexWrap, item_align::ItemAlign,
    },
    flex_size::{
      flex_basis::FlexBasis, flex_grow::FlexGrow, flex_shrink::FlexShrink, flex_size::FlexSize,
    },
    margin_padding::MarginPadding,
    style_value_type::StyleValueType,
    text_decoration::TextDecoration,
    transform::transform::Transform,
  },
  utils::to_camel_case,
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
  pub all_style: Rc<RefCell<Vec<(String, Vec<StyleDeclaration<'i>>)>>>,
  pub document: &'i JSXDocument,
}

impl<'i> StyleVisitor<'i> {
  pub fn new(
    document: &'i JSXDocument,
    all_style: Rc<RefCell<Vec<(String, Vec<StyleDeclaration<'i>>)>>>,
  ) -> Self {
    StyleVisitor {
      all_style,
      document,
    }
  }
}

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

  let mut text_decoration = None;
  let mut color = None;
  let mut flex_options = FlexOptions::new();

  for (id, value) in properties.iter() {
    match id.as_str() {
      "margin" => {
        let margin = MarginPadding::from(value);
        if margin.is_zero() {
          final_properties.remove("margin");
        } else {
          final_properties.insert("margin".to_string(), StyleValueType::MarginPadding(margin));
        }
      }
      "marginLeft" => {
        let margin = final_properties
          .entry("margin".to_string())
          .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
        if let StyleValueType::MarginPadding(margin) = margin {
          margin.set_left(
            value
              .value_to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          )
        }
      }
      "marginRight" => {
        let margin = final_properties
          .entry("margin".to_string())
          .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
        if let StyleValueType::MarginPadding(margin) = margin {
          margin.set_right(
            value
              .value_to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          )
        }
      }
      "marginTop" => {
        let margin = final_properties
          .entry("margin".to_string())
          .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
        if let StyleValueType::MarginPadding(margin) = margin {
          margin.set_top(
            value
              .value_to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          )
        }
      }
      "marginBottom" => {
        let margin = final_properties
          .entry("margin".to_string())
          .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
        if let StyleValueType::MarginPadding(margin) = margin {
          margin.set_bottom(
            value
              .value_to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          )
        }
      }
      "padding" => {
        let padding = MarginPadding::from(value);
        if padding.is_zero() {
          final_properties.remove("padding");
        } else {
          final_properties.insert(
            "padding".to_string(),
            StyleValueType::MarginPadding(padding),
          );
        }
      }
      "paddingLeft" => {
        let padding = final_properties
          .entry("padding".to_string())
          .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
        if let StyleValueType::MarginPadding(padding) = padding {
          padding.set_left(
            value
              .value_to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          )
        }
      }
      "paddingRight" => {
        let padding = final_properties
          .entry("padding".to_string())
          .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
        if let StyleValueType::MarginPadding(padding) = padding {
          padding.set_right(
            value
              .value_to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          )
        }
      }
      "paddingTop" => {
        let padding = final_properties
          .entry("padding".to_string())
          .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
        if let StyleValueType::MarginPadding(padding) = padding {
          padding.set_top(
            value
              .value_to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          )
        }
      }
      "paddingBottom" => {
        let padding = final_properties
          .entry("padding".to_string())
          .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
        if let StyleValueType::MarginPadding(padding) = padding {
          padding.set_bottom(
            value
              .value_to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          )
        }
      }
      "borderRadius" => {
        let border_radius = BorderRadius::from(value);
        if border_radius.is_zero() {
          final_properties.remove("borderRadius");
        } else {
          final_properties.insert(
            "borderRadius".to_string(),
            StyleValueType::BorderRadius(border_radius),
          );
        }
      }
      "borderTopLeftRadius" => {
        let border_radius = final_properties
          .entry("borderRadius".to_string())
          .or_insert(StyleValueType::BorderRadius(BorderRadius::new()));
        if let StyleValueType::BorderRadius(border_radius) = border_radius {
          border_radius.set_top_left(
            value
              .value_to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          )
        }
      }
      "borderTopRightRadius" => {
        let border_radius = final_properties
          .entry("borderRadius".to_string())
          .or_insert(StyleValueType::BorderRadius(BorderRadius::new()));
        if let StyleValueType::BorderRadius(border_radius) = border_radius {
          border_radius.set_top_right(
            value
              .value_to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          )
        }
      }
      "borderBottomLeftRadius" => {
        let border_radius = final_properties
          .entry("borderRadius".to_string())
          .or_insert(StyleValueType::BorderRadius(BorderRadius::new()));
        if let StyleValueType::BorderRadius(border_radius) = border_radius {
          border_radius.set_bottom_left(
            value
              .value_to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          )
        }
      }
      "borderBottomRightRadius" => {
        let border_radius = final_properties
          .entry("borderRadius".to_string())
          .or_insert(StyleValueType::BorderRadius(BorderRadius::new()));
        if let StyleValueType::BorderRadius(border_radius) = border_radius {
          border_radius.set_bottom_right(
            value
              .value_to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          )
        }
      }
      "textDecoration" => {
        text_decoration = Some(value);
      }
      "color" => {
        color = Some(value);
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
          final_properties.insert(id.to_string(), StyleValueType::Background(background));
        }
        if linear_gradient.len() > 0 {
          final_properties.insert(
            "linearGradient".to_string(),
            StyleValueType::LinearGradient(LinearGradient(linear_gradient)),
          );
        }
      }
      "backgroundColor" => {
        let background = final_properties
          .entry("background".to_string())
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
            .entry("background".to_string())
            .or_insert(StyleValueType::Background(Background::new()));
          if let StyleValueType::Background(background) = background {
            background.image = BackgroundImage(images);
          }
        }
        if linear_gradient.len() > 0 {
          final_properties.insert(
            "linearGradient".to_string(),
            StyleValueType::LinearGradient(LinearGradient(linear_gradient)),
          );
        }
      }
      "backgroundPosition" => {
        let background_position = BackgroundImagePosition::from(value);
        if background_position.0.len() > 0 {
          let background = final_properties
            .entry("background".to_string())
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
            .entry("background".to_string())
            .or_insert(StyleValueType::Background(Background::new()));
          if let StyleValueType::Background(background) = background {
            background.size = background_size;
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
          final_properties.insert("flexGrow".to_string(), StyleValueType::FlexGrow(flex_grow));
        }
        if let Some(flex_shrink) = flex_shrink {
          final_properties.insert(
            "flexShrink".to_string(),
            StyleValueType::FlexShrink(flex_shrink),
          );
        }
        if let Some(flex_basis) = flex_basis {
          final_properties.insert(
            "flexBasis".to_string(),
            StyleValueType::FlexBasis(flex_basis),
          );
        }
      }
      "flexGrow" => {
        let flex_grow = FlexGrow::from(value);
        final_properties.insert(id.to_string(), StyleValueType::FlexGrow(flex_grow));
      }
      "flexShrink" => {
        let flex_shrink = FlexShrink::from(value);
        final_properties.insert(id.to_string(), StyleValueType::FlexShrink(flex_shrink));
      }
      "flexBasis" => {
        let flex_basis = FlexBasis::from(value);
        final_properties.insert(id.to_string(), StyleValueType::FlexBasis(flex_basis));
      }
      "alignSelf" => {
        let align_self = ItemAlign::from(value);
        if align_self != ItemAlign::Ignore {
          final_properties.insert(id.to_string(), StyleValueType::AlignSelf(align_self));
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
            "translate".to_string(),
            StyleValueType::Translates(transform.translate),
          );
        }
        if transform.rotate.0.len() > 0 {
          final_properties.insert(
            "rotate".to_string(),
            StyleValueType::Rotates(transform.rotate),
          );
        }
        if transform.scale.0.len() > 0 {
          final_properties.insert("scale".to_string(), StyleValueType::Scales(transform.scale));
        }
        if transform.matrix.0.len() > 0 {
          final_properties.insert(
            "matrix".to_string(),
            StyleValueType::Matrices(transform.matrix),
          );
        }
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
  if let Some(text_decoration) = text_decoration {
    let text_decoration = TextDecoration::from((text_decoration, color));
    final_properties.insert(
      "textDecoration".to_string(),
      StyleValueType::TextDecoration(text_decoration),
    );
  }

  final_properties.insert(
    "flexOptions".to_string(),
    StyleValueType::FlexOptions(flex_options),
  );
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
    let mut style_visitor = StyleVisitor::new(self.document, Rc::clone(&self.all_style));
    stylesheet.visit(&mut style_visitor).unwrap();
  }

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
          style_value
            .iter_mut()
            .reduce(|a, b| {
              for (key, value) in b.iter() {
                let has_property_index = a
                  .iter()
                  .position(|property| property.0 == key.to_owned());
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
