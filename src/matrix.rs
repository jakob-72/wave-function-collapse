use std::fmt::{Display, Formatter};
use std::ops;

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<i8>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![-1; rows * cols],
        }
    }
}

impl ops::Index<(usize, usize)> for Matrix {
    type Output = i8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.data[row * self.cols + col]
    }
}

impl ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        &mut self.data[row * self.cols + col]
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:2} ", self.data[i * self.cols + j])?;
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
        assert_eq!(m.data.len(), 9);
        assert!(m.data.iter().all(|&x| x == -1));
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
