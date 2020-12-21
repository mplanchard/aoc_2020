use anyhow::{anyhow, Error};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, convert::TryFrom};

static INPUT: &'static str = include_str!("day4.input");

#[derive(Debug)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}
impl TryFrom<&str> for EyeColor {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "amb" => Ok(EyeColor::Amber),
            "blu" => Ok(EyeColor::Blue),
            "brn" => Ok(EyeColor::Brown),
            "gry" => Ok(EyeColor::Gray),
            "grn" => Ok(EyeColor::Green),
            "hzl" => Ok(EyeColor::Hazel),
            "oth" => Ok(EyeColor::Other),
            _ => Err(anyhow!("Invalid eye color {}", value)),
        }
    }
}

#[derive(Debug)]
enum HeightUnit {
    Centimeters,
    Inches,
}

#[derive(Debug)]
struct Height {
    number: usize,
    unit: HeightUnit,
}
impl TryFrom<&str> for Height {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();

        // Parse the unit. Input specifies case-sensitvity
        let unit = if value.ends_with("cm") {
            Some(HeightUnit::Centimeters)
        } else if value.ends_with("in") {
            Some(HeightUnit::Inches)
        } else {
            None
        }
        .ok_or(anyhow!("Value {} does not have a unit", value))?;

        // grab the numeric part of the string and convert it to a usize
        let number = value
            .get(0..value.len() - 2)
            // if the .get() failed we're missing a number in front of the unit
            .ok_or(anyhow!(
                "Value {} does not contain a number and a unit",
                value
            ))?
            .parse::<usize>()
            .map_err(Error::from)?;

        // check for valid height for a given unit
        match unit {
            HeightUnit::Centimeters => match number {
                150..=193 => Ok(Self { number, unit }),
                _ => Err(anyhow!("Height {:?} is too large or too small", number)),
            },
            HeightUnit::Inches => match number {
                59..=76 => Ok(Self { number, unit }),
                _ => Err(anyhow!("Height {:?} is too large or too small", number)),
            },
        }
    }
}

#[derive(Debug)]
struct ValidPassport {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: Height,
    hair_color: String,
    eye_color: EyeColor,
    passport_id: String,
    country_id: Option<String>,
}
impl TryFrom<Passport> for ValidPassport {
    type Error = Error;

    fn try_from(value: Passport) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref HEIGHT_RE: Regex = Regex::new(r"^#[\da-f]{6}$").unwrap();
            static ref PASSPORT_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
        Ok(Self {
            birth_year: value
                .birth_year
                .parse::<usize>()
                .map_err(Error::from)
                .and_then(|i| match i {
                    1920..=2002 => Ok(i),
                    _ => Err(anyhow!("Invalid birth year {}", i)),
                })?,
            issue_year: value
                .issue_year
                .parse::<usize>()
                .map_err(Error::from)
                .and_then(|i| match i {
                    2010..=2020 => Ok(i),
                    _ => Err(anyhow!("Invalid issue year {}", i)),
                })?,
            expiration_year: value
                .expiration_year
                .parse::<usize>()
                .map_err(Error::from)
                .and_then(|i| match i {
                    2020..=2030 => Ok(i),
                    _ => Err(anyhow!("Invalid expiration year {}", i)),
                })?,
            height: Height::try_from(value.height.as_ref())?,
            hair_color: Ok(value.hair_color.trim()).and_then(
                |string| match HEIGHT_RE.is_match(string) {
                    true => Ok(string.to_owned()),
                    false => Err(anyhow!("Invalid hair color {}", string)),
                },
            )?,
            eye_color: EyeColor::try_from(value.eye_color.as_ref())?,
            passport_id: Ok(value.passport_id.trim()).and_then(|string| {
                match PASSPORT_RE.is_match(string) {
                    true => Ok(string.to_owned()),
                    false => Err(anyhow!("Invalid passport ID {}", string)),
                }
            })?,
            country_id: value.country_id.clone(),
        })
    }
}

#[derive(Debug)]
struct Passport {
    // Storing everything as a string for the moment because the question doesn't
    // imply anything about valid data, only the presence or absence of keys
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<String>,
}
impl TryFrom<&HashMap<&str, &str>> for Passport {
    type Error = Error;

    fn try_from(value: &HashMap<&str, &str>) -> Result<Self, Self::Error> {
        Ok(Self {
            birth_year: value
                .get("byr")
                .map(|i| String::from(*i))
                .ok_or(anyhow!("no birth_year"))?,
            issue_year: value
                .get("iyr")
                .map(|i| String::from(*i))
                .ok_or(anyhow!("no issue_year"))?,
            expiration_year: value
                .get("eyr")
                .map(|i| String::from(*i))
                .ok_or(anyhow!("no expiration_year"))?,
            height: value
                .get("hgt")
                .map(|i| String::from(*i))
                .ok_or(anyhow!("no height"))?,
            hair_color: value
                .get("hcl")
                .map(|i| String::from(*i))
                .ok_or(anyhow!("no hair_color"))?,
            eye_color: value
                .get("ecl")
                .map(|i| String::from(*i))
                .ok_or(anyhow!("no eye_color"))?,
            passport_id: value
                .get("pid")
                .map(|i| String::from(*i))
                .ok_or(anyhow!("no passport_id"))?,
            country_id: value.get("cid").map(|i| String::from(*i)),
        })
    }
}
impl TryFrom<&str> for Passport {
    type Error = Error;

    /// Convert a passport entry into
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // A passport entry looks like:
        // cid:124 byr:1935 eyr:2020 ecl:blu
        // hcl:#a97842 pid:666776663 iyr:2010
        // hgt:68in
        value
            // split on spaces or newlines
            .split_whitespace()
            // split each item into a k, v pair on :
            // returns Result<(k, v), Error>, where the error is in case there's
            // not two items in the split.
            .map(|v| {
                let iter = v.split(':');
                let parts = iter.take(2).collect::<Vec<&str>>();
                match parts[..] {
                    [i, j] => Ok((i, j)),
                    _ => Err(anyhow!(dbg!(v.to_owned()))),
                }
            })
            // fold over the results, short-circuiting if any of them were an
            // error, or inserting the result into a hashmap otherwise.
            .try_fold(HashMap::new(), |mut items, res| {
                let (k, v) = res?;
                items.insert(k, v);
                Ok(items)
            })
            .and_then(|items| Passport::try_from(&items))
    }
}

pub fn day_four_solution_one() -> usize {
    INPUT
        .split("\n\n")
        .map(Passport::try_from)
        .filter(|r| r.is_ok())
        .count()
}

pub fn day_four_solution_two() -> usize {
    INPUT
        .split("\n\n")
        .map(|i| Passport::try_from(i).and_then(ValidPassport::try_from))
        .filter(Result::is_ok)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    static BAD_INPUT: &'static str = include_str!("day4.input.bad");

    #[test]
    fn bad_input_is_bad() {
        assert_eq!(
            BAD_INPUT
                .split("\n\n")
                .map(|i| { Passport::try_from(i).and_then(ValidPassport::try_from) })
                .filter(Result::is_ok)
                .count(),
            0
        )
    }
}
