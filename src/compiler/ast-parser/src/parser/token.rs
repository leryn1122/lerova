use crate::token::Token;
use std::fmt::Debug;
use std::fmt::Formatter;
use vsp_span::span::Span;
use vsp_span::Locatable;

/// The final result of the lexical analysis, which are transferred to the AST parser.
pub type TokenStream = Vec<LocatableToken>;

/// A locatable token with span, as its start and end location in the source codes.
#[derive(Clone)]
pub struct LocatableToken {
  token: Token,
  span: Span,
}

impl LocatableToken {
  /// New token.
  pub fn new(token: Token, span: Span) -> Self {
    Self { token, span }
  }

  #[inline]
  pub fn token(&self) -> &Token {
    &self.token
  }

  #[inline]
  pub fn span(&self) -> &Span {
    &self.span
  }
}

impl Debug for LocatableToken {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let arr = self.span.expand_as_array();
    write!(
      f,
      "Token = [{:?}]:{},{}:{},{}",
      self.token, arr[0], arr[1], arr[2], arr[3],
    )
  }
}

impl Locatable for LocatableToken {
  fn get_span(&self) -> &Span {
    &self.span
  }
}