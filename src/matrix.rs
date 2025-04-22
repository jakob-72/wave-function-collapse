use std::fmt::{Display, Formatter};
use std::ops;

pub struct Matrix {
    pub cols: usize,
    pub rows: usize,
    pub data: Vec<Vec<i8>>,
}

impl Matrix {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            data: vec![vec![-1; cols]; rows],
        }
    }

    pub fn display_colorful(&self) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                let value = self[(x, y)];
                let color_code = match value {
                    1 => "\x1b[44m  \x1b[0m", // Blue for water
                    2 => "\x1b[43m  \x1b[0m", // Yellow for beach
                    3 => "\x1b[42m  \x1b[0m", // Green for grass
                    _ => "\x1b[47m  \x1b[0m", // White for undefined
                };
                print!("{}", color_code);
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
