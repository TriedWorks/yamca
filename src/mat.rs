#![allow(dead_code)]

use crate::traits::MathComponent;
use crate::vec::Vector;
use std::ops::{Add, Sub};

pub type SquareMatrix<T, const N: usize> = Matrix<T, { N }, { N }>;

#[derive(Debug)]
pub struct Matrix<T, const N: usize, const M: usize> {
    data: [[T; M]; N],
}

impl<T, const N: usize, const M: usize> Matrix<T, { N }, { M }> {
    pub fn new(data: [[T; M]; N]) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        M * N
    }
}

impl<T: MathComponent<T> + Copy, const N: usize, const M: usize> Matrix<T, { N }, { M }> {
    pub fn zero() -> Self {
        Self {
            data: [[<T>::zero(); M]; N],
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> SquareMatrix<T, { N }> {
    pub fn new_identity() -> Self {
        let mut data = [[<T>::zero(); N]; N];
        for i in 0..N {
            data[i][i] = <T>::one();
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize, const M: usize> Add for Matrix<T, { N }, { M }> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [[<T>::default(); M]; N];
        for i in 0..N {
            for j in 0..M {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize, const M: usize> Sub for Matrix<T, { N }, { M }> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [[<T>::default(); M]; N];
        for i in 0..N {
            for j in 0..M {
                data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        Self { data }
    }
}

impl<T: Copy, const N: usize, const M: usize> Matrix<T, { N }, { M }> {
    pub fn to_vectors(&self) -> Vec<Vector<T, { M }>> {
        self.data.iter().map(|col| Vector::new(*col)).collect()
    }
}
