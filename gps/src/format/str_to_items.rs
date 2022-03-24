use super::Anchor;
use super::Item;

use super::errors::ItemErrorKind;
use super::errors::{MISSING_ANCHOR};

pub struct StrToItems<'a> {
    s: &'a str,
    items: Vec<Item<'a>>,
}

impl<'a> StrToItems<'a> {
    fn new(s: &'a str) -> StrToItems<'a> {
        StrToItems {
            s: s,
            items: Vec::new(),
        }
    }
}

impl<'a> Iterator for StrToItems<'a> {
    type Item = Item<'a>;

    fn next(&mut self) -> Option<Item<'a>> {
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
                            'D' => Item::Anchor(Anchor::Degrees),
                            'M' => Item::Anchor(Anchor::MinutesInt),
                            'm' => Item::Anchor(Anchor::MinutesFloat),
                            'S' => Item::Anchor(Anchor::SecondsInt),
                            's' => Item::Anchor(Anchor::SecondsFloat),
                            'C' => Item::Anchor(Anchor::CardinalShortUp),
                            'c' => Item::Anchor(Anchor::CardinalShortLower),
                            'A' => Item::Anchor(Anchor::CardinalCap),
                            'O' => Item::Anchor(Anchor::CardinalUp),
                            'o' => Item::Anchor(Anchor::CardinalLower),
                            e => Item::Error(ItemErrorKind::UNKNOWN_ANCHOR(e)),
                        };
                        Some(item)
                    }
                    // Error because previous char is %
                    None => return Some(Item::Error(MISSING_ANCHOR)),
                }
            }
            Some(_) => {
                self.s = &self.s[1..];
                return Some(Item::Slice("."));
            }
        }
    }
}

#[cfg(test)]
use super::*;

#[test]
fn test_strtoitems() {
    fn parse_and_collect(s: &str) -> Vec<Item> {
        let items = StrToItems::new(s);

        let items = items.map(|i| Some(i) );
        items
            .collect::<Option<Vec<_>>>()
            .unwrap()
    }
    assert_eq!(parse_and_collect(""), []);
    assert_eq!(parse_and_collect("%P"), [Item::Error(ItemErrorKind::UNKNOWN_ANCHOR('P'))]);
    assert_eq!(parse_and_collect("%4"), [Item::Error(ItemErrorKind::UNKNOWN_ANCHOR('4'))]);
    assert_ne!(parse_and_collect("%U"), [Item::Error(ItemErrorKind::UNKNOWN_ANCHOR('S'))]);

    assert_eq!(parse_and_collect("%D"), [Item::Anchor(Anchor::Degrees)]);
    assert_eq!(parse_and_collect("%M"), [Item::Anchor(Anchor::MinutesInt)]);
    assert_eq!(
        parse_and_collect("%m"),
        [Item::Anchor(Anchor::MinutesFloat)]
    );
    assert_eq!(parse_and_collect("%S"), [Item::Anchor(Anchor::SecondsInt)]);
    assert_eq!(
        parse_and_collect("%s"),
        [Item::Anchor(Anchor::SecondsFloat)]
    );
    assert_eq!(
        parse_and_collect("%C"),
        [Item::Anchor(Anchor::CardinalShortUp)]
    );
    assert_eq!(
        parse_and_collect("%c"),
        [Item::Anchor(Anchor::CardinalShortLower)]
    );
    assert_eq!(parse_and_collect("%A"), [Item::Anchor(Anchor::CardinalCap)]);
    assert_eq!(parse_and_collect("%O"), [Item::Anchor(Anchor::CardinalUp)]);
    assert_eq!(
        parse_and_collect("%o"),
        [Item::Anchor(Anchor::CardinalLower)]
    );
}
