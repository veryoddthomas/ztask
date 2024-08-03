// use chrono;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError(String),
}

pub fn parse_duration(s: &str) -> Result<i64, Error> {
    let re = Regex::new(r"(?P<value>\d+) *(?P<unit>[[:alpha:]\p{Greek}]*)").unwrap();
    let sign_multiplier = if s.starts_with("-") { -1 } else { 1 };
    let mut results = vec![];
    for cap in re.captures_iter(s) {
        let value = cap.name("value").map(|m| m.as_str()).unwrap_or("");
        let unit = cap.name("unit").map(|m| m.as_str()).unwrap_or("");
        results.push((value, unit));
    }

    if results.is_empty() {
        return Err(Error::ParseError(format!("invalid duration: '{}'", s)));
    }

    let mut duration = 0;
    for (value, unit) in results {
        let value = value.parse::<i64>().unwrap();
        let unit_multiplier = match unit {
            "s" => 1,
            "m" => 60,
            "h" => 60 * 60,
            "d" => 60 * 60 * 24,
            "w" => 60 * 60 * 24 * 7,
            _ => {
                return Err(Error::ParseError(format!(
                    "invalid duration units: '{unit}'"
                )));
            }
        };
        duration += value * unit_multiplier;
    }
    Ok(duration * sign_multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() -> Result<(), Error> {
        assert_eq!(parse_duration("2s")?, 2);
        assert_eq!(parse_duration("104s")?, 104);
        assert_eq!(parse_duration("2m")?, 2 * 60);
        assert_eq!(parse_duration("3m10s")?, (3 * 60) + 10);
        assert_eq!(parse_duration("3m 10s")?, (3 * 60) + 10);
        assert_eq!(parse_duration("3h")?, 3 * 60 * 60);
        assert_eq!(parse_duration("-2s")?, -2);
        assert_eq!(
            parse_duration("").unwrap_err(),
            Error::ParseError(String::from("invalid duration: ''")),
        );
        assert_eq!(
            parse_duration("12").unwrap_err(),
            Error::ParseError(String::from("invalid duration units: ''")),
        );
        assert_eq!(
            parse_duration("1q").unwrap_err(),
            Error::ParseError(String::from("invalid duration units: 'q'")),
        );
        Ok(())
    }
}
