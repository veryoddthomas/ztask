use chrono::TimeDelta;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError(String),
}

const SECONDS_PER_MINUTE: i64 = 60;
const SECONDS_PER_HOUR: i64 = SECONDS_PER_MINUTE * 60;
const SECONDS_PER_DAY: i64 = SECONDS_PER_HOUR * 24;
const SECONDS_PER_WEEK: i64 = SECONDS_PER_DAY * 7;

pub fn parse(s: &str) -> Result<chrono::TimeDelta, Error> {
    lazy_static! {
        static ref DURATION_REGEX: Regex =
            Regex::new(r"(?P<value>\d+) *(?P<unit>[[:alpha:]\p{Greek}]*)").unwrap();
    }

    let sign_multiplier = if s.starts_with('-') { -1 } else { 1 };
    let mut results = vec![];
    for cap in DURATION_REGEX.captures_iter(s) {
        let value = cap.name("value").map_or("", |m| m.as_str());
        let unit = cap.name("unit").map_or("", |m| m.as_str());
        results.push((value, unit));
    }

    if results.is_empty() {
        return Err(Error::ParseError(format!("invalid duration: '{s}'")));
    }

    let mut duration = 0;
    for (value, unit) in results {
        let value = value.parse::<i64>().unwrap();
        let unit_multiplier = match unit {
            "s" => 1,
            "m" => SECONDS_PER_MINUTE,
            "h" => SECONDS_PER_HOUR,
            "d" => SECONDS_PER_DAY,
            "w" => SECONDS_PER_WEEK,
            _ => {
                return Err(Error::ParseError(format!(
                    "invalid duration units: '{unit}'"
                )));
            }
        };
        duration += value * unit_multiplier;
    }
    Ok(TimeDelta::seconds(duration * sign_multiplier))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() -> Result<(), Error> {
        assert_eq!(parse("2s")?.num_seconds(), 2);
        assert_eq!(parse("104s")?.num_seconds(), 104);
        assert_eq!(parse("2m")?.num_seconds(), 2 * SECONDS_PER_MINUTE);
        assert_eq!(parse("3m10s")?.num_seconds(), (3 * SECONDS_PER_MINUTE) + 10);
        assert_eq!(
            parse("3m 10s")?.num_seconds(),
            (3 * SECONDS_PER_MINUTE) + 10
        );
        assert_eq!(parse("3h")?.num_seconds(), 3 * SECONDS_PER_HOUR);
        assert_eq!(parse("-2s")?.num_seconds(), -2);
        assert_eq!(
            parse("").unwrap_err(),
            Error::ParseError(String::from("invalid duration: ''")),
        );
        assert_eq!(
            parse("12").unwrap_err(),
            Error::ParseError(String::from("invalid duration units: ''")),
        );
        assert_eq!(
            parse("1q").unwrap_err(),
            Error::ParseError(String::from("invalid duration units: 'q'")),
        );
        Ok(())
    }
}
