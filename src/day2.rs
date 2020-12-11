use std::convert::TryFrom;
use std::ops::Range;

use anyhow;

static INPUT: &'static str = include_str!("day2.input");

struct PasswordRule {
    allowed: Range<usize>,
    character: char,
}
impl PasswordRule {
    fn new(allowed: Range<usize>, character: char) -> Self {
        Self { allowed, character }
    }
    fn is_valid_first_question<T: AsRef<str>>(&self, password: &T) -> bool {
        let pw = password.as_ref();
        self.allowed
            .contains(&pw.chars().filter(|c| c == &self.character).count())
    }
    fn is_valid_second_question<T: AsRef<str>>(&self, password: &T) -> bool {
        let pw = password.as_ref();
        // our range is set to [x through y + 1]. They want to check the
        // xth and yth items, so we need index (start - 1) for the first item,
        // e.g. index 0 for item 1, and index (end - 2) for the second item,
        // e.g. index 1 for item 2, which would have a range end of 3.
        pw.chars()
            .nth(self.allowed.start - 1)
            .map(|x| x == self.character)
            .and_then(|x| if x { Some(true) } else { None })
            // careful, they say "exactly one" must match
            .xor(
                pw.chars()
                    .nth(self.allowed.end - 2)
                    .map(|x| x == self.character)
                    .and_then(|x| if x { Some(true) } else { None }),
            )
            // if neither was true, or both was true, we'd have a None here,
            // so we unwrap_or false
            .unwrap_or(false)
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

pub fn day_two_solution_one() -> anyhow::Result<usize> {
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
                .filter(|(pw_rule, pw)| pw_rule.is_valid_first_question(pw))
                // count them
                .count()
        })
}

// yeah yeah this is just copied from the first one and literaly the only
// change is calling pw_rule.is_valid_second_question instead of
// pw_rule.is_valid_first_question, and I could probably simplify, but
// why bother?
pub fn day_two_solution_two() -> anyhow::Result<usize> {
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
                .filter(|(pw_rule, pw)| pw_rule.is_valid_second_question(pw))
                // count them
                .count()
        })
}
