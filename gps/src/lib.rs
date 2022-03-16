use std::error;
use std::str::FromStr;

struct Latitude {
    degrees: u8,
    minutes: u8,
    seconds: f32,
    orientation: bool,
}

struct Longitude {
    degrees: u8,
    minutes: u8,
    seconds: f32,
    orientation: bool,
}

impl Latitude {
    fn new(
        degrees: u8,
        minutes: u8,
        seconds: f32,
        orientation: bool,
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
            orientation,
        })
    }
}

impl Longitude {
    fn new(
        degrees: u8,
        minutes: u8,
        seconds: f32,
        orientation: bool,
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
            orientation,
        })
    }
}

pub struct GpsPoint {
    longitude: Longitude,
    latitude: Latitude,
}

impl GpsPoint {
    fn new(latitude: Latitude, longitude: Longitude) -> GpsPoint {
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
    fn new_longitude_pass() {
        assert!(Longitude::new(1, 2, 3f32, true).is_ok())
    }

    #[test]
    fn new_longitude_fail() {
        assert!(Longitude::new(181, 2, 3f32, false).is_err());
        assert!(Longitude::new(1, 62, 3f32, false).is_err());
        assert!(Longitude::new(1, 2, 60.01, false).is_err());
        assert!(Longitude::new(1, 2, -0.01, false).is_err());
    }

    #[test]
    fn new_latitude_pass() {
        assert!(Latitude::new(1, 2, 3f32, true).is_ok())
    }

    #[test]
    fn new_latitude_fail() {
        assert!(Latitude::new(91, 2, 3f32, false).is_err());
        assert!(Latitude::new(1, 62, 3f32, false).is_err());
        assert!(Latitude::new(1, 2, 60.01, false).is_err());
        assert!(Latitude::new(1, 2, -0.01, false).is_err());
    }
}
