use super::Cardinal;
use super::{Coordonnee, Latitude, Longitude};
use super::{East, North, South, West};
use core::borrow::Borrow;
use std::fmt;

mod errors;

use errors::ParseResult;
use errors::ItemErrorKind;

mod str_to_items;
use str_to_items::StrToItems;

#[derive(Debug, PartialEq)]
pub enum Item<'a> {
    Slice(&'a str),
    Anchor(Anchor),
    Error(ItemErrorKind),
}

#[derive(Debug, PartialEq)]
pub enum Anchor {
    // 1° 2' 3.4"N
    //
    // degrees => 1
    // %D
    Degrees,

    // Minutes without decimal => 2
    // %M
    MinutesInt,

    // Seconds as minutes decimal e.g MinutesInt.MinutesFloat => 056666667 (3.4 / 60)
    // %m
    MinutesFloat,

    // Seconds without decimal => 3
    // %S
    SecondsInt,

    // Seconds decimal => 4
    // %s
    SecondsFloat,

    // Single cardinal letter uppercase => W
    // %C
    CardinalShortUp,

    // Single cardinal letter loawercase => w
    // %c
    CardinalShortLower,

    // Complete cardinal name capitalize => West
    // %A
    CardinalCap,

    // Complete cardinal name uppercase => WEST
    // %O
    CardinalUp,

    // Complete cardinal name lowercase => west
    // %o
    CardinalLower,
}
