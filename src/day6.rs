use std::{collections::HashSet, ops::RangeInclusive};

static INPUT: &'static str = include_str!("day6.input");

struct CustomsForm {
    // A vec of sets, where each set are the questions one individual in the
    // group answered.
    questions_answered_yes: Vec<HashSet<char>>,
}
impl CustomsForm {
    const ALLOWED_CHARS: RangeInclusive<char> = 'a'..='z';

    fn any_yes_count(&self) -> usize {
        // the length of the union set of yes answers from all members of the
        // group
        self.questions_answered_yes
            .iter()
            .fold(HashSet::new(), |acc, set| {
                acc.union(set).map(|i| *i).collect::<HashSet<char>>()
            })
            .len()
    }

    fn every_yes_count(&self) -> usize {
        // the length of the intersection set of yes answers from all members
        // of the group
        let accumulator: Option<HashSet<char>> = None;
        self.questions_answered_yes
            .iter()
            .fold(accumulator, |acc, set| match acc {
                Some(last_set) => Some(
                    last_set
                        .intersection(set)
                        .map(|i| *i)
                        .collect::<HashSet<char>>(),
                ),
                None => Some(set.clone()),
            })
            // we assume there will be at least one answer from one person
            .unwrap()
            .len()
    }
}
// We have no reason to assume the input might be bad, so we'll treat it
// as infallible and use From
impl From<&str> for CustomsForm {
    fn from(value: &str) -> Self {
        CustomsForm {
            questions_answered_yes: value
                .lines()
                // convert each line into a set
                .map(|ln| {
                    ln.chars()
                        .filter(|c| CustomsForm::ALLOWED_CHARS.contains(c))
                        .fold(HashSet::new(), |mut acc, c| {
                            acc.insert(c);
                            acc
                        })
                })
                .collect(),
        }
    }
}

pub fn day_six_solution_one() -> usize {
    INPUT
        .split("\n\n")
        .map(CustomsForm::from)
        .map(|form| form.any_yes_count())
        .sum()
}

pub fn day_six_solution_two() -> usize {
    INPUT
        .split("\n\n")
        .map(CustomsForm::from)
        .map(|form| form.every_yes_count())
        .sum()
}
