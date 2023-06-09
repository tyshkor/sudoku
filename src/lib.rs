use std::{collections::HashSet, fmt::Display, ops::Range, str::FromStr};

use anyhow::{Error, Result};
use consts::{FULL_RANGE, SIZE, RANGE, ZERO};

mod consts;

#[derive(Debug)]
pub struct Sudoku {
    grid: [[u8; SIZE]; SIZE],
}

impl Sudoku {
    pub fn new(grid: [[u8; SIZE]; SIZE]) -> Self {
        Sudoku { grid }
    }

    pub fn valid(&self) -> bool {
        // Check rows
        for row in &self.grid {
            if !Self::is_valid_unit(row) {
                return false;
            }
        }

        // Check columns
        for col in FULL_RANGE {
            let column_vec: Vec<_> = self.grid.iter().map(|row| row[col]).collect();
            let column: [u8; SIZE] = column_vec.try_into().unwrap();
            if !Self::is_valid_unit(&column) {
                return false;
            }
        }

        // Check 3x3 grids
        for row_start in FULL_RANGE.step_by(3) {
            for col_start in FULL_RANGE.step_by(3) {
                let grid_vec: Vec<_> = RANGE
                    .flat_map(|i| RANGE.map(move |j| self.grid[row_start + i][col_start + j]))
                    .collect();
                let grid: [u8; SIZE] = grid_vec.try_into().unwrap();
                if !Self::is_valid_unit(&grid) {
                    return false;
                }
            }
        }

        true
    }

    fn is_valid_unit(unit: &[u8]) -> bool {
        let set: HashSet<u8> = unit.iter().copied().collect();
        // Zero value is not allowed in Sudoku
        // Checking for the fact that each number existed just once in the unit
        if set.contains(&ZERO) || set.len() != SIZE {
            return false;
        }
        true
    }
}

impl FromStr for Sudoku {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<u8>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .filter(|&c| c.is_ascii_digit())
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        if grid.len() != 9 || grid.iter().any(|row| row.len() != 9) {
            return Err(Error::msg("wrong grid size"));
        }

        let mut array = [[0; SIZE]; SIZE];
        for (i, row) in grid.iter().enumerate() {
            array[i].copy_from_slice(&row[..SIZE]);
        }

        Ok(Sudoku::new(array))
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for &num in row {
                write!(f, "{num}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sudoku() {
        let sudoku: Sudoku = "534678912\n\
                              672195348\n\
                              198342567\n\
                              859761423\n\
                              426853791\n\
                              713924856\n\
                              961537284\n\
                              287419635\n\
                              345286179"
            .parse()
            .unwrap();
        println!("{}", sudoku);
        assert!(sudoku.valid());
    }

    #[test]
    fn test_invalid_sudoku_vertical() {
        let sudoku: Sudoku = "534678912\n\
                              672195348\n\
                              198342567\n\
                              859761423\n\
                              426853791\n\
                              713924856\n\
                              861537284\n\
                              287419635\n\
                              345286179"
            .parse()
            .unwrap();
        println!("{}", sudoku);
        assert!(!sudoku.valid());
    }

    #[test]
    fn test_invalid_sudoku_horizontal() {
        let sudoku: Sudoku = "534678912\n\
                              672195348\n\
                              198342567\n\
                              859761433\n\
                              426853791\n\
                              713924856\n\
                              961537284\n\
                              287419635\n\
                              345286179"
            .parse()
            .unwrap();
        println!("{}", sudoku);
        assert!(!sudoku.valid());
    }

    #[test]
    fn test_invalid_sudoku_box() {
        let sudoku: Sudoku = "534678912\n\
                          672195348\n\
                          198342567\n\
                          859761423\n\
                          426855791\n\
                          713924856\n\
                          961537284\n\
                          287419635\n\
                          345286179"
            .parse()
            .unwrap();
        println!("{}", sudoku);
        assert!(!sudoku.valid());
    }

    #[test]
    fn test_invalid_sudoku_duplicate_numbers() {
        let sudoku: Sudoku = "534678912\n\
                          672195348\n\
                          198342567\n\
                          859761423\n\
                          426853791\n\
                          713924856\n\
                          961537284\n\
                          287419635\n\
                          345286111"
            .parse()
            .unwrap();
        println!("{}", sudoku);
        assert!(!sudoku.valid());
    }

    #[test]
    fn test_invalid_sudoku_incomplete_grid() {
        let sudoku = "534678912\n\
                          672195348\n\
                          198342567\n\
                          859761423\n\
                          426853791\n\
                          713924856\n\
                          961537284\n\
                          287419635"
            .parse::<Sudoku>();
        assert!(sudoku.is_err());
    }
}
