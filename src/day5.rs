use anyhow::{anyhow, Error};
use std::cmp::max;
use std::convert::TryFrom;

static INPUT: &'static str = include_str!("day5.input");

#[derive(Debug)]
struct SearchSpecification {
    lower: char,
    upper: char,
    length: u8,
}
impl SearchSpecification {
    const fn new(lower: char, upper: char, length: u8) -> Self {
        SearchSpecification {
            lower,
            upper,
            length,
        }
    }
    /// Parse the binary search specification into a binary number
    fn parse(&self, search: &str) -> Result<u8, Error> {
        if search.len() != self.length.into() {
            return Err(anyhow!(
                "Invalid length search {} for specification {:?}",
                search,
                self
            ));
        }
        search
            .chars()
            .enumerate()
            .try_fold(0u8, |acc, (idx, c)| match c {
                _ if c == self.lower => Ok(acc),
                _ if c == self.upper => Ok(acc | 1 << (self.length - 1) - idx as u8),
                _ => Err(anyhow!("Invalid character in search value: {}", search)),
            })
    }
}

#[cfg(test)]
mod test_search_spec {
    use super::*;

    #[test]
    fn test_specification_to_u8() {
        // do a three-bit thing so we can ensure we've tested all the options
        let spec = SearchSpecification::new('A', 'B', 3);
        let cases = [
            ("AAA", 0),
            ("AAB", 1),
            ("ABA", 2),
            ("ABB", 3),
            ("BAA", 4),
            ("BAB", 5),
            ("BBA", 6),
            ("BBB", 7),
        ];
        cases.iter().for_each(|case| {
            assert_eq!(spec.parse(case.0).unwrap(), case.1);
        });
    }
}

#[derive(Debug)]
struct Seat {
    row: u8,
    col: u8,
}
impl Seat {
    const ROW_SEARCH_SPEC: SearchSpecification = SearchSpecification::new('F', 'B', 7);
    const COL_SEARCH_SPEC: SearchSpecification = SearchSpecification::new('L', 'R', 3);

    fn id(&self) -> usize {
        self.row as usize * 8 + self.col as usize
    }
}
impl TryFrom<&str> for Seat {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        let row_search = value.get(0..7).ok_or(anyhow!("invalid input {}", &value))?;
        let col_search = value
            .get(7..10)
            .ok_or(anyhow!("invalid input {}", &value))?;
        // The trick here is the search specifications themselves are the SAME
        // as the binary representation of the column/row number. So LLL is 000,
        // corresponding to row 0, RRR is 111 corresponding to row 7, LLR is 001
        // corresponding to row 1, and so on.
        Ok(Self {
            row: Seat::ROW_SEARCH_SPEC.parse(row_search)?,
            col: Seat::COL_SEARCH_SPEC.parse(col_search)?,
        })
    }
}

pub fn day_five_solution_one() -> Result<usize, Error> {
    INPUT.lines().try_fold(0, |acc, ln| {
        Seat::try_from(ln).map(|seat| max(acc, seat.id()))
    })
}
