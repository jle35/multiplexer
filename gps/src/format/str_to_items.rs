use super::Anchor;
use super::ParseItem;

use super::errors::ItemErrorKind;
use super::errors::{MISSING_ANCHOR};

pub struct StrToItems<'a> {
    s: &'a str,
    items: Vec<ParseItem>,
}

impl<'a> StrToItems<'a> {
    pub fn new(s: &'a str) -> StrToItems<'a> {
        StrToItems {
            s: s,
            items: Vec::new(),
        }
    }
}

impl<'a> Iterator for StrToItems<'a> {
    type Item = ParseItem;

    fn next(&mut self) -> Option<ParseItem> {
        match self.s.chars().next() {
            // End
            None => None,
            // if char is Anchor
            Some('%') => {
                // pass next char
                self.s = &self.s[1..];
                // if next char exist return, else Error
                match self.s.chars().next() {
                    Some(c) => {
                        // move next char (utf8 size in case of no 1 byte)
                        self.s = &self.s[c.len_utf8()..];
                        let item = match c {
                            'D' => ParseItem::NumFix(Anchor::Degrees),
                            'M' => ParseItem::NumFix(Anchor::MinutesInt),
                            'm' => ParseItem::NumFix(Anchor::MinutesFloat),
                            'S' => ParseItem::NumFix(Anchor::SecondsInt),
                            's' => ParseItem::NumFix(Anchor::SecondsFloat),
                            'C' => ParseItem::Card(Anchor::CardinalShortUp),
                            'c' => ParseItem::Card(Anchor::CardinalShortLower),
                            'A' => ParseItem::Card(Anchor::CardinalCap),
                            'O' => ParseItem::Card(Anchor::CardinalUp),
                            'o' => ParseItem::Card(Anchor::CardinalLower),
                            e => ParseItem::Error(ItemErrorKind::UnknownAnchor(e)),
                        };
                        Some(item)
                    }
                    // Error because previous char is %
                    None => return Some(ParseItem::Error(MISSING_ANCHOR)),
                }
            }
            Some(c) => {
                self.s = &self.s[1..];
                return Some(ParseItem::Char(c));
            }
        }
    }
}

#[cfg(test)]
use super::*;

#[test]
fn test_strtoitems() {
    fn parse_and_collect(s: &str) -> Vec<ParseItem> {
        let items = StrToItems::new(s);

        let items = items.map(|i| Some(i) );
        items
            .collect::<Option<Vec<_>>>()
            .unwrap()
    }
    assert_eq!(parse_and_collect(""), []);
    assert_eq!(parse_and_collect("%P"), [ParseItem::Error(ItemErrorKind::UnknownAnchor('P'))]);
    assert_eq!(parse_and_collect("%4"), [ParseItem::Error(ItemErrorKind::UnknownAnchor('4'))]);
    assert_ne!(parse_and_collect("%U"), [ParseItem::Error(ItemErrorKind::UnknownAnchor('S'))]);

    assert_eq!(parse_and_collect("%D"), [ParseItem::NumFix(Anchor::Degrees)]);
    assert_eq!(parse_and_collect("%M"), [ParseItem::NumFix(Anchor::MinutesInt)]);
    assert_eq!(
        parse_and_collect("%m"),
        [ParseItem::NumFix(Anchor::MinutesFloat)]
    );
    assert_eq!(parse_and_collect("%S"), [ParseItem::NumFix(Anchor::SecondsInt)]);
    assert_eq!(
        parse_and_collect("%s"),
        [ParseItem::NumFix(Anchor::SecondsFloat)]
    );
    assert_eq!(
        parse_and_collect("%C"),
        [ParseItem::Card(Anchor::CardinalShortUp)]
    );
    assert_eq!(
        parse_and_collect("%c"),
        [ParseItem::Card(Anchor::CardinalShortLower)]
    );
    assert_eq!(parse_and_collect("%A"), [ParseItem::Card(Anchor::CardinalCap)]);
    assert_eq!(parse_and_collect("%O"), [ParseItem::Card(Anchor::CardinalUp)]);
    assert_eq!(
        parse_and_collect("%o"),
        [ParseItem::Card(Anchor::CardinalLower)]
    );

    assert_eq!(parse_and_collect("%Av"), [ParseItem::Card(Anchor::CardinalCap), ParseItem::Char('v')]);
    assert_ne!(parse_and_collect("%Av"), [ParseItem::Card(Anchor::CardinalCap), ParseItem::Char('v'), ParseItem::Char('e')]);
}
