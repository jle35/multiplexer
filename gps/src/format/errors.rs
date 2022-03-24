use core::fmt;

// Errors when parsing string
#[derive(Debug)]
enum ParseErrorKind {
    BAD,
}

pub type ParseResult<T> = Result<T, ParseErrorKind>;

impl fmt::Display for ParseErrorKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
        ParseErrorKind::BAD => write!(f, "BAD !!"),
    }
  }
}

// Errors of items iterator
#[derive(Debug, PartialEq)]
pub enum ItemErrorKind {
    UNKNOWN_ANCHOR(char),
    MISSING_ANCHOR,
}

pub type ItemResult<T> = Result<T, ItemErrorKind>;

impl fmt::Display for ItemErrorKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
      ItemErrorKind::UNKNOWN_ANCHOR(a) => write!(f, "Unknown anchor: %{}", a),
      ItemErrorKind::MISSING_ANCHOR => write!(f, "Missing anchor after `%`"),
      }

  }
}

// Items errors
pub const MISSING_ANCHOR: ItemErrorKind = ItemErrorKind::MISSING_ANCHOR;
