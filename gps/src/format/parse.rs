mod errors;
use errors::ParseResult;

use 

pub struct Parsed {
    degrees: Option<u8>,
    minutes: Option<u8>,
    seconds: Option<f32>,
    cardinal: Option<Cardinal>,
}

impl Default for Parsed {
    fn default() -> Parsed {
        Parsed {
            degrees: None,
            minutes: None,
            seconds: None,
            cardinal: None,
        }
    }
}

impl Parsed {
    pub fn new() -> Parsed {
        Parsed::default()
    }
    
    // Why to_coordonne<T: coordonne> -> PArseResult<T> desent work
    pub fn to_coordonnee(&self) -> Result<Latitude, &str  > {
    Latitude::new(self.degrees.unwrap(), self.minutes.unwrap(), self.seconds.unwrap(), self.cardinal.unwrap())
    }
}

// TODO understand the Borrow<Item>
pub fn parse<'a, I>(parsed: &mut Parsed, mut s: &str, items: I) -> ParseResult<()>
where
    I: Iterator<Item = Item<'a>>,
{
    let ret: ParseResult<()> = Ok(());
    for item in items {
        let ret = match item {
            Item::Slice(_) => {
                s = &s[1..];
                Ok(())
            }
            Item::Anchor(ref i) => {
                let (min_width, max_width) = match i {
                    Degree => (3, 3),
                    MinutesInt => (2, 2),
                    MinutesFloat => (5, 5),
                    SecondsInt => (2, 2),
                    SecondsFloat => (5, 5),
                    CardinalShortUp => (1, 1),
                    CardinalShortLower => (1, 1),
                    CardinalCap => (4, 5),
                    CardinalUp => (4, 5),
                    CardinalLower => (4, 5),
                };
                Ok(())
            }
            _ => Err(_),
        };
    }
    ret
}

#[cfg(test)]
use super::*;

#[test]
fn test_parse() {
    let lat = Latitude::new(08, 09, 7.407, North).unwrap();
    let mut parsed: Parsed = Parsed::new();
    parse(&mut parsed, "0809.12345N", StrToItems::new("%D%M.%m%O"));
    let lat_from_parse: Latitude = parsed.to_coordonnee().unwrap();

    assert_eq!(lat_from_parse, lat);
}
