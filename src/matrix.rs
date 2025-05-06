use crate::rules::Ruleset;
use colored::{Color, Colorize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops;

/// A simple matrix struct for 2D grid representation
pub struct Matrix {
    pub cols: usize,
    pub rows: usize,
    pub data: Vec<Vec<i8>>,
}

const BLOCK_CHAR: &str = "  ";
const FALLBACK_COLORS: [Color; 12] = [
    Color::Blue,
    Color::Yellow,
    Color::Green,
    Color::Red,
    Color::Magenta,
    Color::Cyan,
    Color::BrightRed,
    Color::BrightGreen,
    Color::BrightYellow,
    Color::BrightBlue,
    Color::BrightMagenta,
    Color::BrightCyan,
];

impl Matrix {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            data: vec![vec![-1; cols]; rows],
        }
    }

    /// Print the matrix with colors for different values
    pub fn display_colorful(&self, ruleset: &Ruleset) {
        let mut field_colors = HashMap::new();
        let mut fallback_index = 0;

        for &field in ruleset.all_fields().iter() {
            if let Some(color) = ruleset.get_color_for_field(field) {
                field_colors.insert(field, Color::from(color));
            } else {
                field_colors.insert(field, FALLBACK_COLORS[fallback_index]);
                fallback_index = (fallback_index + 1) % FALLBACK_COLORS.len();
            }
        }

        for y in 0..self.rows {
            for x in 0..self.cols {
                let field = self[(x, y)];
                if let Some(&color) = field_colors.get(&field) {
                    print!("{}", BLOCK_CHAR.on_color(color));
                } else {
                    print!("{}", BLOCK_CHAR.on_white()); // Default color for unknown fields
                }
            }
            println!();
        }
    }
}

impl ops::Index<(usize, usize)> for Matrix {
    type Output = i8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (col, row) = index;
        &self.data[row][col]
    }
}

impl ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (col, row) = index;
        &mut self.data[row][col]
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.cols * self.rows > 10_000 {
            return write!(f, "Matrix is too large to display.");
        }
        for y in 0..self.rows {
            for x in 0..self.cols {
                write!(f, "{:1} ", self[(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix_init() {
        let m = Matrix::new(3, 3);
        assert_eq!(m.rows, 3);
        assert_eq!(m.cols, 3);
        assert_eq!(m.data.len(), 3);
        assert_eq!(m.data[0].len(), 3);
        assert!(m.data.iter().all(|row| row.iter().all(|&x| x == -1)));
        println!("matrix init:\n{}", m);
    }

    #[test]
    fn test_index_operator() {
        let mut m = Matrix::new(3, 3);
        m[(0, 0)] = 1;
        m[(1, 1)] = 2;
        m[(2, 2)] = 3;
        assert_eq!(m[(0, 0)], 1);
        assert_eq!(m[(1, 1)], 2);
        assert_eq!(m[(2, 2)], 3);
        println!("index operator:\n{}", m);
    }
}
