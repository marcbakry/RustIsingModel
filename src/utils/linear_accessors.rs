//! This module implements the conversion between linear indexing 
//! and cartesian indexing in both direction.

/// This structure simply holds the dimensions of the cartesian grid.
pub struct LinearAccessor2d {
    ni: usize,
    nj: usize,
}

impl LinearAccessor2d {
    /// Create a new `LinearAccessor`.
    pub fn new(ni: usize, nj: usize) -> Self {
        Self { ni: ni, nj: nj }
    }

    /// Cartesian coordinates to row-major linear index
    pub fn cart2lin_rm(&self, co: &[usize]) -> usize {
        co[1] + self.nj*co[0]
    }

    /// Cartesian coordinates to column-major linear index
    pub fn cart2lin_cm(&self, co: &[usize]) -> usize {
        co[0] + self.ni*co[1]
    }

    /// Linear index to row-major cartesian coordinates
    pub fn lin2cart_rm(&self, lin: usize) -> [usize;2] {
        let j = lin % self.nj;
        [(lin-j)/self.nj,j]
    }

    /// Linear index to column-major cartesian coordinates
    pub fn lin2cart_cm(&self, lin: usize) -> [usize;2] {
        let i = lin % self.ni;
        [i, (lin-i)/self.ni]
    }
}

#[cfg(test)]
mod linear_accessors_tests {
    use super::*;

    #[test]
    fn la_rm_test() {
        let (ni,nj) = (4,5);
        let la = LinearAccessor2d::new(ni, nj);
        assert_eq!(la.cart2lin_rm(&[1,3]),8);
        assert_eq!(la.lin2cart_rm(8),[1,3]);
        assert_eq!(la.cart2lin_rm(&[3,4]),19);
        assert_eq!(la.lin2cart_rm(19),[3,4]);
        assert_eq!(la.cart2lin_rm(&[0,0]),0);
        assert_eq!(la.lin2cart_rm(0),[0,0]);
    }

    #[test]
    fn la_cm_test() {
        let (ni,nj) = (4,5);
        let la = LinearAccessor2d::new(ni, nj);
        assert_eq!(la.cart2lin_cm(&[1,3]),13);
        assert_eq!(la.lin2cart_cm(13),[1,3]);
        assert_eq!(la.cart2lin_cm(&[3,4]),19);
        assert_eq!(la.lin2cart_cm(19),[3,4]);
        assert_eq!(la.cart2lin_cm(&[0,0]),0);
        assert_eq!(la.lin2cart_cm(0),[0,0]);
    }
}