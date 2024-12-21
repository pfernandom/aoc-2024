use colored::Colorize;
use ndarray::Array2;

use crate::{parse_u32_matrix, InputData};

#[derive(Debug, Clone)]
pub struct Map2D {
    data: Array2<u32>,
}

impl Map2D {
    pub fn from_input(_input: InputData) -> Map2D {
        Map2D {
            data: parse_u32_matrix(_input.lines().collect::<Vec<_>>()),
        }
    }

    pub fn rows(&self) -> usize {
        self.data.dim().0
    }

    pub fn cols(&self) -> usize {
        self.data.dim().1
    }

    pub fn get(&self, coord: (usize, usize)) -> Option<&u32> {
        self.data.get(coord)
    }

    pub fn get_next_possible_hv(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(ir, ic)| (pos.0 as isize + ir, pos.1 as isize + ic))
            .filter(|(r, c)| {
                *r >= 0 && *c >= 0 && *r < self.rows() as isize && *c < self.cols() as isize
            })
            .map(|(ir, ic)| (ir as usize, ic as usize))
            .collect::<Vec<_>>()
    }

    pub fn draw_path(&self, path: &Vec<(usize, usize)>) {
        let mut buff = String::new();
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                if let Some(val) = self.data.get((r, c)) {
                    if path.contains(&(r, c)) {
                        buff.push_str(format!("{}", *val).blue().to_string().as_str());
                    } else {
                        buff.push_str(format!("{}", *val).white().to_string().as_str());
                    }
                }
            }
            buff.push('\n');
        }

        println!("{}", buff)
    }
}
