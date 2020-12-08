use std::convert::TryFrom;
use std::ops::Range;

use anyhow;

mod day1;

static INPUT: &'static str = include_str!("day2.input");

struct PasswordRule {
    allowed: Range<usize>,
    character: char,
}
impl PasswordRule {
    fn new(allowed: Range<usize>, character: char) -> Self {
        Self { allowed, character }
    }
    fn is_valid<T: AsRef<str>>(&self, password: &T) -> bool {
        let pw = password.as_ref();
        self.allowed
            .contains(&pw.chars().filter(|c| c == &self.character).count())
    }
}
// See https://github.com/rust-lang/rust/issues/50133 for context on the wrapper
struct PasswordRuleWrapper<T: AsRef<str>>(T);
impl<T> TryFrom<PasswordRuleWrapper<T>> for PasswordRule
where
    T: AsRef<str>,
{
    type Error = anyhow::Error;

    fn try_from(value: PasswordRuleWrapper<T>) -> Result<Self, Self::Error> {
        let val = Some(value.0.as_ref().trim().split_whitespace());
        val
            // ensure we've got two values
            .and_then(|mut v| v.next().and_then(|range| Some((range, v.next()?))))
            // convert the second value to a char
            .and_then(|(r, c)| Some((r, c.chars().next()?)))
            // split the first value on the dash
            .map(|(r, c)| (r.split('-'), c))
            // ensure the first value contained two items
            .and_then(|(mut r, c)| {
                r.next().and_then(|start| Some(((start, r.next()?), c)))
            })
            .ok_or(anyhow::anyhow!("Bad input"))
            // convert the items of the first value to integers
            // we add 1 to the end b/c Range is [), and the test is []
            .and_then(|((start, end), c)| {
                Ok(((start.parse::<usize>()?, end.parse::<usize>()? + 1), c))
            })
            // construct the password rule
            .map(|((start, end), c)| PasswordRule::new(Range { start, end }, c))
    }
}

fn day_two_solution_one() -> anyhow::Result<usize> {
    INPUT
        .lines()
        // for each line
        .map(|ln| {
            // split it on the colon
            let mut split = ln.split(':');
            split
                // grab the first item of the split
                .next()
                // remove leading and trailing whitespace
                .map(str::trim)
                // convert from option to result
                .ok_or(anyhow::anyhow!("Input must contain a :"))
                // attempt to convert to password rule
                .and_then(|pw_rule| {
                    PasswordRule::try_from(PasswordRuleWrapper(pw_rule))
                })
                // if we succeeded, chain to getting the password
                .and_then(|pw_rule| {
                    split
                        // grab the second item out of the split
                        .next()
                        // trim it of whitespace
                        .map(str::trim)
                        // convert from option to result
                        .ok_or(anyhow::anyhow!(
                            "Input must contain a password after the :"
                        ))
                        // return a tuple of rule and password
                        .map(|pw| (pw_rule, pw))
                })
        })
        // collect passwords into a vec of (rule, pw)
        .collect::<Result<Vec<_>, anyhow::Error>>()
        .map(|vals| {
            // if that succeeded, iterate over values
            vals.iter()
                // return only those with valid password
                .filter(|(pw_rule, pw)| pw_rule.is_valid(pw))
                // count them
                .count()
        })
}

fn main() -> anyhow::Result<()> {
    dbg!(day1::day_one_solution_one());
    dbg!(day1::day_one_solution_two());
    dbg!(day_two_solution_one()?);
    Ok(())
}
