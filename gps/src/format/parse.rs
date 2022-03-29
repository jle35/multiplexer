use super::{Latitude, Longitude, Cardinal};
use super::errors::ParseResult;
use super::ParseItem;

use super::errors::{IMPOSSIBLE, ANCHOR_TWICE};

use super::scan::{scan_number, scan_cardinal};

pub struct Parsed {
    degrees: Option<usize>,
    minutesInt: Option<usize>,
    minutesFloat: Option<usize>,
    secondsInt: Option<usize>,
    secondsFloat: Option<usize>,
    cardinalShortUp: Option<Cardinal>,
    cardinalShortLower: Option<Cardinal>,
    cardinalCap: Option<Cardinal>,
    cardinalUp: Option<Cardinal>,
    cardinalLower: Option<Cardinal>,
}

impl Default for Parsed {
    fn default() -> Parsed {
        Parsed {
            degrees: None,
            minutesInt: None,
            minutesFloat: None,
            secondsInt: None,
            secondsFloat: None,
            cardinalShortUp: None,
            cardinalShortLower: None,
            cardinalCap: None,
            cardinalUp: None,
            cardinalLower: None,
        }
    }
}

impl Parsed{
    pub fn new() -> Parsed {
        Parsed::default()
    }

    fn set_if_not_exist<T: PartialEq>(v: &mut Option<T>, n: T) -> ParseResult<()> {
        match v {
            Some(v) => {
                if *v != n {
                    return Err(ANCHOR_TWICE);
                }
            },
            None => *v = Some(n),
        }
        Ok(())
    }

    fn set_degrees(&mut self, n: usize) -> ParseResult<()> {
        Parsed::set_if_not_exist(&mut self.degrees, n)
    }
    
    fn set_minutes_int(&mut self, n: usize) -> ParseResult<()> {
        Parsed::set_if_not_exist(&mut self.minutesInt, n)
    }
    
    fn set_minutes_float(&mut self, n: usize) -> ParseResult<()> {
        Parsed::set_if_not_exist(&mut self.minutesFloat, n)
    }
    
    fn set_seconds_int(&mut self, n: usize) -> ParseResult<()> {
        Parsed::set_if_not_exist(&mut self.secondsInt, n)
    }
    
    fn set_seconds_float(&mut self, n: usize) -> ParseResult<()> {
        Parsed::set_if_not_exist(&mut self.secondsFloat, n)
    }
    
    fn set_cardinal_short_up(&mut self, n: Cardinal) -> ParseResult<()> {
        Parsed::set_if_not_exist(&mut self.cardinalShortUp, n)
    }
    
    fn set_cardinal_short_lower(&mut self, n: Cardinal) -> ParseResult<()> {
        Parsed::set_if_not_exist(&mut self.cardinalShortLower, n)
    }
    
    fn set_cardinal_cap(&mut self, n: Cardinal) -> ParseResult<()> {
        Parsed::set_if_not_exist(&mut self.cardinalCap, n)
    }
    
    fn set_cardinal_up(&mut self, n: Cardinal) -> ParseResult<()> {
        Parsed::set_if_not_exist(&mut self.cardinalUp, n)
    }
    
    fn set_cardinal_lower(&mut self, n: Cardinal) -> ParseResult<()> {
        Parsed::set_if_not_exist(&mut self.cardinalLower, n)
    }
    
    // Why to_coordonne<T: coordonne> -> PArseResult<T> desent work
    //pub fn to_coordonnee(&self) -> Result<Latitude, &str  > {
    //Latitude::new(self.degrees.unwrap(), self.minutes.unwrap(), self.seconds.unwrap(), self.cardinal.unwrap())
    //}
}

// TODO understand the Borrow<Item>
pub fn parse(mut s: &str, items: &mut dyn Iterator<Item = ParseItem>) -> ParseResult<Parsed>
{
    macro_rules! try_consume {
        ($e:expr) => {{
            match $e {
                Ok((s_, v)) => {
                    s = s_;
                    v
                },
                Err(e) => return Err(e),
            }
        }}
    }
    let mut parsed: Parsed = Parsed::new();
    for item in items {
        match item {
            ParseItem::Char(_) => {
                s = &s[1..];
            }
            ParseItem::NumFix(ref i) => {
                type Setter = fn(&mut Parsed, usize) -> ParseResult<()>;
                let (width, setter): (usize, Setter) = match i {
                    Degrees => (3, Parsed::set_degrees),
                    MinutesInt => (2, Parsed::set_minutes_int),
                    MinutesFloat => (5, Parsed::set_minutes_float),
                    SecondsInt => (2, Parsed::set_seconds_int),
                    SecondsFloat => (5, Parsed::set_seconds_float),
                };
                let v = try_consume!(scan_number(s, width));
                setter(&mut parsed, v).map_err(|e| (parsed, e));
            },
            ParseItem::Card(ref i) => {
                type Setter = fn(&mut Parsed, Cardinal) -> ParseResult<()>;
                let (min, max, setter): (usize, usize, Setter) = match i {
                    CardinalShortUp => (1, 1, Parsed::set_cardinal_short_up),
                    CardinalShortLower => (1, 1, Parsed::set_cardinal_short_lower),
                    CardinalCap => (4, 5, Parsed::set_cardinal_cap),
                    CardinalUp => (4, 5, Parsed::set_cardinal_up),
                    CardinalLower => (4, 5, Parsed::set_cardinal_lower),
                };
                let v = try_consume!(scan_cardinal(s, min, max));
                setter(&mut parsed, v).map_err(|e| (parsed, e));
            },
            _ => return Err(IMPOSSIBLE)
        };
    }
    Ok(parsed)
}

//#[cfg(test)]
//use super::*;
//
//#[test]
//fn test_parse() {
//    let lat = Latitude::new(08, 09, 7.407, North).unwrap();
//    let parsed = parse("0809.12345N", &mut StrToItems::new("%D%M.%m%O"));
//    let lat_from_parse: Latitude = parsed.to_coordonnee().unwrap();
//
//    assert_eq!(lat_from_parse, lat);
//}
