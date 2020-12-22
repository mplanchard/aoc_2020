use anyhow::{anyhow, Error};
use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
};

static INPUT: &'static str = include_str!("day5.input");

// 128 rows on the plane (0-127)
// 8 columns (0-7)

struct Seat {
    row: usize,
    column: usize,
}
impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.column
    }
}
impl From<SeatSpecification> for Seat {
    // this is infallible, since we've already validated the input
    fn from(_: SeatSpecification) -> Self {
        todo!()
    }
}

#[derive(Debug)]
struct SearchState {
    min: u8,
    max: u8,
}
impl SearchState {
    fn new(total: u8) -> Self {
        Self { min: 1, max: total }
    }
    fn resolved(&self) -> bool {
        self.min == self.max
    }
}

struct BinaryPartition {
    // specification assumes that 0 means "lower half" and 1 means "upper half"
    specification: u8,
    num_values: u8,
}
impl BinaryPartition {
    fn new(specification: u8, num_values: u8) -> Self {
        Self {
            specification,
            num_values,
        }
    }
    fn calculate(&self) -> u8 {
        // How many binary digits we expect to need from the specification
        // is determined by how many binary digits we need to represent the
        // number of values. However, we can save time by knowing that the
        // MOST iterations we need for an 8-bit number is 8 iterations, so
        // we can iterate over that and exit early when we've narrowed in
        // on our value.
        (0..8)
            .try_fold(SearchState::new(self.num_values), |mut state, idx| {
                dbg!(&state);
                if state.min == state.max {
                    // No more state updates, we can end iteration
                    return Err(state.min);
                }

                // How much are we going to narrow the solution space
                let space_adjustment = (state.max - state.min) / 2 + 1;

                // We want the lower half if the bit at our index is 0
                let take_lower = self.specification & (1 << idx) == 0;

                dbg!(&take_lower);

                if take_lower {
                    if space_adjustment == 0 {
                        // No need to adjust solution space, return the min
                        return Err(state.min);
                    }
                    // adjust state to contain only the lower half
                    state.max = state.max - space_adjustment;
                } else {
                    if space_adjustment == 0 {
                        // No need to adjust solution space, return the max
                        return Err(state.max);
                    }
                    // adjust state to contain only the upper half
                    state.min = state.min + space_adjustment;
                }
                // keep iterating b/c we haven't hit an end condition
                Ok(state)
            })
            .unwrap_err()
    }
}

#[cfg(test)]
mod test_binary_partition {
    use super::*;

    #[test]
    fn test_simple() {
        let cases = [
            // bottom: 1-2
            // bottom: 1
            (0b00, 4, 1),
            // bottom: 1-2
            // top: 2
            (0b10, 4, 2),
            // top: 3-4
            // bottom: 3
            (0b01, 4, 3),
            // top: 3-4
            // top: 4
            (0b11, 4, 4),
            // is there a pattern??? sure I could look this up, but it's more
            // fun to think about it. It's tantalizingly pattern-like.
            // 0
            (0b000, 8, 1),
            // 4
            (0b100, 8, 2),
            // 2
            (0b010, 8, 3),
            // 6
            (0b110, 8, 4),
            // 1
            (0b001, 8, 5),
            // 5
            (0b101, 8, 6),
            // 3
            (0b011, 8, 7),
            // 7
            (0b111, 8, 8),
        ];
        cases.iter().for_each(|case| {
            let partition = BinaryPartition::new(case.0, case.1);
            let result = partition.calculate();
            assert_eq!(result, case.2);
        })
    }
}

struct SeatSpecification {
    // The search is represented with letters in the input, but they're binary
    // essentially
    row_search: u8,
    col_search: u8,
}
impl SeatSpecification {
    fn nth_seat(&self, total_rows: usize, total_columns: usize) -> Seat {
        todo!();
        // iterate through each binary digit of the row_search to get which
        // row it is.
        // let mut row_search = SearchState::new(128);
        // let nth_row = (0..7).scan(row_search, |state, idx| {
        //     let state_diff = state.max - state.min;
        //     // if the index of the binary number is 0, it's a front seat
        //     let is_front = (2u8).pow(idx) & self.row_search == 0;
        //     if is_front {
        //         // our max seat is closer to the front (a smaller number)
        //         state.max = state.max - state_diff / 2;
        //     } else {
        //         // our min seat is closer to the back (a larger number)
        //         state.min = state.min + state_diff / 2;
        //     }
        //     Some(state)
        // });
    }
}
impl Display for SeatSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SeatSearch {{ row: {:07b}, col: {:03b} }}",
            self.row_search, self.col_search
        )
    }
}
impl TryFrom<&str> for SeatSpecification {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        // Ensure we've got the number of chars we expect.
        let row_value = value.get(0..7).ok_or(anyhow!("invalid input {}", &value))?;
        let col_value = value.get(7..).ok_or(anyhow!("invalid input {}", &value))?;

        // turn the row input into a 7-bit number by converting front to 0
        // and back to 1
        let row_search =
            row_value
                .chars()
                .enumerate()
                .try_fold(0u8, |acc, (i, c)| match c {
                    // if we're an F, we don't need to do anything, since the
                    // position is already 0.
                    'F' => Ok(acc),
                    // if we're a B, "insert a 1" at the right position in the
                    // binary number by OR'ing with 2^i
                    'B' => Ok(acc | (2u8).pow(i.try_into().unwrap())),
                    _ => Err(anyhow!("Invalid input! {}", &value)),
                })?;

        // turn the col input into a 3-bit number by converting left to 0 and
        // right to 1
        let col_search =
            col_value
                .chars()
                .enumerate()
                .try_fold(0u8, |acc, (i, c)| match c {
                    // don't need to do anything for left
                    'L' => Ok(acc),
                    // insert a 1 at the right index for a right
                    'R' => Ok(acc | (2u8).pow(i.try_into().unwrap())),
                    _ => Err(anyhow!("Invalid input! {}", &value)),
                })?;

        Ok(Self {
            row_search,
            col_search,
        })
    }
}

pub fn day_five_solution_one() {
    INPUT
        .lines()
        .map(SeatSpecification::try_from)
        .for_each(|i| {
            println!("{}", i.unwrap());
        });
}
