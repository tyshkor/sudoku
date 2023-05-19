use std::{collections::HashSet, ops::Range};

use anyhow::{Error, Result};

const FULL_RANGE: Range<usize> = 0..9;
const RANGE: Range<usize> = 0..3;
const SIZE: usize = 9;
const ZERO: u8 = 0;
