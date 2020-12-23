use std::{collections::HashSet, ops::RangeInclusive};

static INPUT: &'static str = include_str!("day6.input");

struct CustomsForm {
    questions_answered_yes: HashSet<char>,
}
impl CustomsForm {
    const ALLOWED_CHARS: RangeInclusive<char> = 'a'..='z';

    fn yes_count(&self) -> usize {
        self.questions_answered_yes.len()
    }
}
// We have no reason to assume the input might be bad, so we'll treat it
// as infallible and use From
impl From<&str> for CustomsForm {
    fn from(value: &str) -> Self {
        CustomsForm {
            questions_answered_yes: value
                .chars()
                .filter(|c| CustomsForm::ALLOWED_CHARS.contains(c))
                .fold(HashSet::new(), |mut acc, c| {
                    acc.insert(c);
                    acc
                }),
        }
    }
}

pub fn day_six_solution_one() -> usize {
    INPUT
        .split("\n\n")
        .map(CustomsForm::from)
        .map(|form| form.yes_count())
        .sum()
}
