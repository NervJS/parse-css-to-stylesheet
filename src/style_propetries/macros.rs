#[macro_export]
macro_rules! generate_expr_lit_str {
  ($var:expr) => {
    swc_ecma_ast::Expr::Lit(swc_ecma_ast::Lit::Str($var.into()))
  };
}

#[macro_export]
macro_rules! generate_expr_lit_num {
  ($var:expr) => {
    swc_ecma_ast::Expr::Lit(swc_ecma_ast::Lit::Num(swc_ecma_ast::Number::from($var)))
  };
}

#[macro_export]
macro_rules! generate_expr_ident {
  ($var:expr) => {
    swc_ecma_ast::Expr::Ident(swc_ecma_ast::Ident::new($var.into(), DUMMY_SP))
  };
}

#[macro_export]
macro_rules! generate_expr_based_on_platform {
  ($platform:expr, $value:expr) => {
      match $platform {
          Platform::ReactNative => $value.to_rn_expr().into(),
          _ => $value.to_expr().into(),
      }
  };
}