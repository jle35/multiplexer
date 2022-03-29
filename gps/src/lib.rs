pub mod format;

#[derive(Debug, PartialEq)]
pub enum Cardinal {
    North,
    South,
    East,
    West,
}


pub const North: Cardinal = Cardinal::North;
pub const South: Cardinal = Cardinal::South;
pub const East: Cardinal = Cardinal::East;
pub const West: Cardinal = Cardinal::West;

pub trait Coordonnee {
    fn degrees(&self) -> u8;
    fn minutes(&self) -> u8;
    fn seconds(&self) -> f32;
    fn minutes_with_dec(&self) -> f32 {
        self.minutes() as f32 + (self.seconds() as f32 / 60.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct Latitude {
    degrees: u8,
    minutes: u8,
    seconds: f32,
    cardinal: Cardinal,
}

#[derive(Debug, PartialEq)]
pub struct Longitude {
    degrees: u8,
    minutes: u8,
    seconds: f32,
    cardinal: Cardinal, // true is North, false is South
}

impl Latitude {
    fn new(
        degrees: u8,
        minutes: u8,
        seconds: f32,
        cardinal: Cardinal,
    ) -> Result<Latitude, &'static str> {
        if degrees > 90 {
            return Err("Degrees must be between 0 and 90.");
        } else if minutes > 60 {
            return Err("Minutes must be between 0 and 60.");
        } else if seconds > 60.0 || seconds < 0 as f32 {
            return Err("Seconds must be between 0 and 60.");
        }
        match cardinal {
            Cardinal::North => {},
            Cardinal::South => {},
            Cardinal::East => return Err("Cardinal must be North or South."),
            Cardinal::West => return Err("Cardinal must be North or South."),
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
    fn degrees(&self) -> u8 {
        self.degrees
    }
    fn minutes(&self) -> u8 {
        self.minutes
    }
    fn seconds(&self) -> f32 {
        self.seconds
    }
}

impl Coordonnee for Longitude {
    fn degrees(&self) -> u8 {
        self.degrees
    }
    fn minutes(&self) -> u8 {
        self.minutes
    }
    fn seconds(&self) -> f32 {
        self.seconds
    }
}

impl Longitude {
    fn new(
        degrees: u8,
        minutes: u8,
        seconds: f32,
        cardinal: Cardinal,
    ) -> Result<Longitude, &'static str> {
        if degrees > 179 {
            return Err("Degrees must be between 0 and 179.");
        } else if minutes > 60 {
            return Err("Minutes must be between 0 and 60.");
        } else if seconds > 60.0 || seconds < 0f32 {
            return Err("Seconds must be between 0 and 60.");
        }
        match cardinal {
            Cardinal::North => return Err("Cardinal must be East or West."),
            Cardinal::South => return Err("Cardinal must be East or West."),
            Cardinal::East => {},
            Cardinal::West => {},
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
    fn test_new_latitude() {
        assert!(Latitude::new(1, 2, 3f32, North).is_ok());

        assert!(Latitude::new(91, 2, 3f32, South).is_err());
        assert!(Latitude::new(1, 62, 3f32, South).is_err());
        assert!(Latitude::new(1, 2, 60.01, South).is_err());
        assert!(Latitude::new(1, 2, -0.01, North).is_err());
    }

    #[test]
    fn test_minutes_with_dec(){
        let lat = Latitude::new(1, 2, 45.2, South);
        assert_eq!(lat.unwrap().minutes_with_dec(), 2.7533333);
    }
}
