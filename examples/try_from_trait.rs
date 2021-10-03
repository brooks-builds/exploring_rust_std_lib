use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug)]
struct Location {
    x: f32,
    y: f32,
}

#[derive(Debug, Error)]
enum CustomErrors {
    #[error("Cannot convert into target type")]
    CannotConvertInto,
}

impl TryFrom<[f32; 2]> for Location {
    type Error = anyhow::Error;

    fn try_from(value: [f32; 2]) -> Result<Self, Self::Error> {
        if value[0] < 0.0 || value[1] < 0.0 {
            return Err(CustomErrors::CannotConvertInto.into());
        }

        Ok(Self {
            x: value[0],
            y: value[1],
        })
    }
}

impl TryFrom<(f64, f64)> for Location {
    type Error = anyhow::Error;

    fn try_from(value: (f64, f64)) -> Result<Self, Self::Error> {
        let x = if value.0 > f64::from(f32::MAX) {
            return Err(anyhow::anyhow!("Value too large to convert into location"));
        } else if value.0 < f64::from(f32::MIN) {
            return Err(anyhow::anyhow!("Value too small to convert into location"));
        } else {
            value.0 as f32
        };
        let y = if value.1 > f64::from(f32::MAX) {
            return Err(anyhow::anyhow!("Value too large to convert into location"));
        } else if value.1 < f64::from(f32::MIN) {
            return Err(anyhow::anyhow!("Value too small to convert into location"));
        } else {
            value.1 as f32
        };
        Ok(Location { x, y })
    }
}

fn main() {
    let location = Location::try_from((55.0_f64, 10.0_f64)).unwrap();
    dbg!(location);
}
