use std::{collections::HashSet, ops::Range};

use anyhow::{Error, Result};

const FULL_RANGE: Range<usize> = 0..9;
const RANGE: Range<usize> = 0..3;
const SIZE: usize = 9;
const ZERO: u8 = 0;

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
        let set: HashSet<u8> = unit.to_vec().into_iter().collect();
        if set.contains(&ZERO) || set.len() != SIZE {
            return false;
        }
        true
    }
}
