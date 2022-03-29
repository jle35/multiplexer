use core::fmt;
use super::parse::Parsed;

// Errors when parsing string
#[derive(Debug)]
pub enum ParseErrorKind {
    Impossible,
    AnchorTwice,
    TooShort,
    NotANumber,
    OutOfRange,
}

pub type ParseResult<T> = Result<T, ParseErrorKind>;

impl fmt::Display for ParseErrorKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
        ParseErrorKind::Impossible => write!(f, "Impossible to parse"),
        ParseErrorKind::AnchorTwice => write!(f, "An anchor has been implemented twice."),
        ParseErrorKind::TooShort => write!(f, "Value provided for is too short."),
        ParseErrorKind::NotANumber => write!(f, "Value is not a valid number."),
        ParseErrorKind::OutOfRange => write!(f, "Value is out of accepted range value."),
    }
  }
}

pub const IMPOSSIBLE: ParseErrorKind = ParseErrorKind::Impossible;
pub const ANCHOR_TWICE: ParseErrorKind = ParseErrorKind::AnchorTwice;
pub const TOO_SHORT: ParseErrorKind = ParseErrorKind::TooShort;
pub const NOT_A_NUMBER: ParseErrorKind = ParseErrorKind::NotANumber;
pub const OUT_OF_RANGE: ParseErrorKind = ParseErrorKind::OutOfRange;

// Errors of items iterator
#[derive(Debug, PartialEq)]
pub enum ItemErrorKind {
    UnknownAnchor(char),
    MissingAnchor,
}

pub type ItemResult<T> = Result<T, ItemErrorKind>;

impl fmt::Display for ItemErrorKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
      ItemErrorKind::UnknownAnchor(a) => write!(f, "Unknown anchor: %{}", a),
      ItemErrorKind::MissingAnchor => write!(f, "Missing anchor after `%`"),
      }

  }
}

// Items errors
pub const MISSING_ANCHOR: ItemErrorKind = ItemErrorKind::MissingAnchor;
