//! This module handles tiled grid, ia grid which are separated in smaller
//! grids of equal sizes (as much as possible). It provides mappings from
//! the global index to the local index and *vice versa*.
use super::circular_index::*;


/// This structure implements a tiled grid  accessor in 1d.
pub struct TiledGridAccessor1d {
    cindex: CircularIndex,
    lengths: Vec<usize>,
}

impl TiledGridAccessor1d {
    /// Creates a new `TiledGridAccessor1d` with `n` tiles.
    pub fn new(lb: usize, ub: usize, n: usize) -> Self {
        assert!(n > 0,"TiledGridAccessor1d::new: The partition must feature at least one interval (n > 0).");
        let cindex = CircularIndex::new(lb, ub);
        let delta = ub - lb + 1;
        assert!(delta > 0,"TiledGridAccessor1d::new: Invalid interval.");
        let mut ntarget = delta/n;
        assert!(ntarget > 0,"TiledGridAccessor1d:new: The subintervals are empty.");
        let lengths = if ntarget*n == delta {
            vec![ntarget; n]
        } else {
            ntarget += 1;
            let mut lengths = vec![ntarget; n];
            *lengths.last_mut().unwrap() = delta - ntarget*(n-1);
            lengths
        };
        Self { cindex: cindex, lengths: lengths }
    }

    /// Accessor to the subdomain lengths
    pub fn sublengths(&self) -> &Vec<usize> {
        &self.lengths
    }

    /// Global to local index, subinterval number.
    pub fn g2l(&self, iglob: usize) -> (usize,usize) {
        let iglob = self.cindex.at(iglob);
        // iglob = k*le + iloc
        (iglob % self.lengths.first().unwrap(),iglob/self.lengths.first().unwrap())
    }

    /// Local to global index
    pub fn l2g(&self, iloc: usize, iint: usize) -> usize {
        assert!(iloc < self.lengths[iint]);
        iint*self.lengths.first().unwrap() + iloc
    }
}

#[cfg(test)]
mod tiled_grid_1d_tests {
    use super::*;

    #[test]
    fn gridding_test() {
        let (lb,ub) = (2,13);
        let n = 4;
        let tga = TiledGridAccessor1d::new(lb, ub, n);
        let lengths = tga.sublengths();
        assert_eq!(*lengths,vec![3;4]);

        let (lb,ub) = (2,15);
        let n = 4;
        let tga = TiledGridAccessor1d::new(lb, ub, n);
        let lengths = tga.sublengths();
        assert_eq!(*lengths,vec![4,4,4,2]);
    }

    #[test]
    fn accessors_test() {
        let (lb, ub) = (0,9); // length = 10
        let n = 3; // 3 subintervals => lengths == vec![4,4,2];
        let tga = TiledGridAccessor1d::new(lb, ub, n);

        // global to local
        let iglob = 3;
        let (iloc,iint) = tga.g2l(iglob);
        assert_eq!((iloc,iint),(3,0));
        // 
        let iglob = 5;
        let (iloc,iint) = tga.g2l(iglob);
        assert_eq!((iloc,iint),(1,1));
        // 
        let iglob = 9;
        let (iloc,iint) = tga.g2l(iglob);
        assert_eq!((iloc,iint),(1,2));

        // local to global
        let iloc_iint = [(3,0),(1,1),(1,2)];
        let iglob_ref = [3,5,9];
        let _ = iloc_iint.into_iter().zip(iglob_ref.into_iter()).map(|(il_ii,ig)| assert_eq!(tga.l2g(il_ii.0, il_ii.1),ig)).collect::<Vec<_>>();
    }
}