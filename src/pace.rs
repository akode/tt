use anyhow::{anyhow, Result};
use core::fmt;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Pace {
    minutes: u8,
    seconds: u8,
}

impl Pace {
    // Parses a pace of the form 'mm:ss' or 'm::ss' into an instance of 'Pace'
    pub fn from_str(str_pace: &str) -> Result<Self> {
        match str_pace.split_once(":").ok_or("Can't split on ':'") {
            Ok((str_minutes, str_seconds)) => {
                let minutes = u8::from_str(str_minutes)?;
                let seconds = u8::from_str(str_seconds)?;
                Ok(Pace { minutes, seconds })
            }
            _ => Err(anyhow!("Failed to parse the pace")),
        }
    }
}

impl fmt::Display for Pace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{:02}", self.minutes, self.seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_str() {
        let test_pace = Pace::from_str("3:45").unwrap_or(Pace {
            minutes: 0,
            seconds: 0,
        });
        assert_eq!(
            test_pace,
            Pace {
                minutes: 3,
                seconds: 45
            }
        )
    }
}
