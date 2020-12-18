use anyhow::{anyhow, Error};
use std::{collections::HashMap, convert::TryFrom};

static INPUT: &'static str = include_str!("day4.input");

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
                // let parts = (
                //     iter.next().ok_or(anyhow!(&v))?,
                //     iter.next().ok_or(anyhow!(&v))?,
                // );
                // Ok(parts)
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
