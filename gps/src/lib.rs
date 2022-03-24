pub mod format;

enum Cardinal {
    CardinalLat,
    CardinalLong
}

#[derive(Debug, PartialEq)]
enum CardinalLat {
    North,
    South
}

#[derive(Debug, PartialEq)]
enum CardinalLong {
    East,
    West
}

const North: CardinalLat = CardinalLat::North;
const South: CardinalLat = CardinalLat::South;
const East: CardinalLong = CardinalLong::East;
const West: CardinalLong = CardinalLong::West;

pub trait Coordonnee {
    fn parse_from_str(fmt: &str, s: &str) -> Result<Latitude, &'static str>;
}

#[derive(Debug, PartialEq)]
pub struct Latitude {
    degrees: u8,
    minutes: u8,
    seconds: f32,
    cardinal: CardinalLat,
}

#[derive(Debug, PartialEq)]
pub struct Longitude {
    degrees: u8,
    minutes: u8,
    seconds: f32,
    cardinal: CardinalLong, // true is North, false is South
}

impl Latitude {
    fn new(
        degrees: u8,
        minutes: u8,
        seconds: f32,
        cardinal: CardinalLat,
    ) -> Result<Latitude, &'static str> {
        if degrees > 90 {
            return Err("Degrees must be between 0 and 90.");
        } else if minutes > 60 {
            return Err("Minutes must be between 0 and 60.");
        } else if seconds > 60.0 || seconds < 0 as f32 {
            return Err("Seconds must be between 0 and 60.");
        }
        Ok(Latitude {
            degrees,
            minutes,
            seconds,
            cardinal,
        })
    }
}

impl Coordonnee for Latitude {
    fn parse_from_str(fmt: &str, s: &str) -> Result<Latitude, &'static str>  {
        Latitude::new(1, 2, 3.0, CardinalLat::North)
    }
}

impl Longitude {
    fn new(
        degrees: u8,
        minutes: u8,
        seconds: f32,
        cardinal: CardinalLong,
    ) -> Result<Longitude, &'static str> {
        if degrees > 179 {
            return Err("Degrees must be between 0 and 179.");
        } else if minutes > 60 {
            return Err("Minutes must be between 0 and 60.");
        } else if seconds > 60.0 || seconds < 0f32 {
            return Err("Seconds must be between 0 and 60.");
        }
        Ok(Longitude {
            degrees,
            minutes,
            seconds,
            cardinal,
        })
    }
}

pub struct GpsPoint {
    longitude: Longitude,
    latitude: Latitude,
}

impl GpsPoint {
    pub fn new(latitude: Latitude, longitude: Longitude) -> GpsPoint {
        GpsPoint {
            latitude,
            longitude,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_longitude() {
        assert!(Longitude::new(1, 2, 3f32, East).is_ok());

        assert!(Longitude::new(181, 2, 3f32, West).is_err());
        assert!(Longitude::new(1, 62, 3f32, West).is_err());
        assert!(Longitude::new(1, 2, 60.01, West).is_err());
        assert!(Longitude::new(1, 2, -0.01, West).is_err());
    }

    #[test]
    fn tes_new_latitude() {
        assert!(Latitude::new(1, 2, 3f32, North).is_ok());

        assert!(Latitude::new(91, 2, 3f32, South).is_err());
        assert!(Latitude::new(1, 62, 3f32, South).is_err());
        assert!(Latitude::new(1, 2, 60.01, South).is_err());
        assert!(Latitude::new(1, 2, -0.01, North).is_err());
    }
}
