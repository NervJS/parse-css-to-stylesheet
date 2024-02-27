#[macro_export]
macro_rules! generate_flex_number {
  ($class:ident, $property_name:ident) => {
    #[derive(Debug, Clone)]
    pub struct $class(pub CSSNumber);

    impl ToExpr for $class {
      fn to_expr(&self) -> Expr {
        Expr::Lit(Lit::Num(Number {
          span: DUMMY_SP,
          value: self.0 as f64,
          raw: None,
        }))
        .into()
      }
    }

    impl From<&Property<'_>> for $class {
      fn from(value: &Property<'_>) -> Self {
        let mut result = $class(0.0);
        match value {
          Property::$property_name(value, _) => {
            result = $class(*value);
          }
          _ => {}
        }
        result
      }
    }
  };
}

#[macro_export]
macro_rules! impl_to_expr_for_transform_mem {
  ($class:ty; $($name:ident),*; $($var:ident),*) => {
    impl ToExpr for $class {
      fn to_expr(&self) -> Expr {
        let mut props = vec![];
        $(
          if let Some(ref value) = self.$name {
            props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new(to_camel_case(stringify!($name), false).into(), DUMMY_SP)),
              value: value.to_expr().into(),
            }))));
          }
        )*
        $(
          props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(Ident::new(to_camel_case(stringify!($var), false).into(), DUMMY_SP)),
            value: self.$var.to_expr().into(),
          }))));
        )*
        Expr::Object(ObjectLit {
          span: DUMMY_SP,
          props: vec![
            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new("type".into(), DUMMY_SP)),
              value: Expr::Lit(Lit::Str(swc_ecma_ast::Str {
                span: DUMMY_SP,
                value: stringify!($class).into(),
                raw: None
              })).into(),
            }))),
            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new("value".into(), DUMMY_SP)),
              value: Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props,
              })
              .into(),
            }))),
          ]
        })
      }
    }
  };
}

#[macro_export]
macro_rules! generate_transform_item {
  ($class:ident, $item:ty) => {
    #[derive(Debug, Clone)]
    pub struct $class(pub Vec<$item>);

    impl $class {
      pub fn new() -> Self {
        $class(vec![])
      }
    }

    impl ToExpr for $class {
      fn to_expr(&self) -> Expr {
        let mut items = vec![];
        for item in self.0.iter() {
          items.push(Some(item.to_expr().into()));
        }
        Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: items,
        })
      }
    }
  };
}
