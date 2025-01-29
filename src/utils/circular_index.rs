//! This modules implements a linear circular index which loops over itself when the
//! upper or lower bound is overshot.

/// The `CircularIndex`` structure holds a lower bound and an upper bound.
pub struct CircularIndex {
    lower_bound: usize,
    upper_bound: usize,
}

/// The `Circular2dIndex` structure is the 2d-grid equivalent of `CircularIndex`.
pub struct Circular2dIndex {
    i_index: CircularIndex,
    j_index: CircularIndex,
}

impl CircularIndex {
    /// Creates a new circular index between `lb` and `ub`. This function *will panic* if `lb >= ub`.
    pub fn new(lb: usize, ub: usize) -> Self {
        if lb >= ub {
            panic!("CircularIndex::new : Lower bound 'lb' is high than the upper bound 'ub'.");
        }
        Self {lower_bound: lb, upper_bound: ub}
    }

    /// Returns the circular index corresponding to `i`, merely `i`.
    pub fn at(&self, i: usize) -> usize {
        if i >= self.lower_bound && i <= self.upper_bound {
            return i;
        } else {
            panic!("CircularIndex::at : Index `i` is out of bounds.");
        }
    }

    /// Returns the next circular index which can be either `i+1` if `i <= upper_bound` or `lower_bound`.
    pub fn next(&self, i: usize) -> usize {
        if i < self.upper_bound {
            return i+1;
        } else {
            return self.lower_bound;
        }
    }

    /// Returns the previous circular index which can be either `i-1` if `i > lower_bound` or `lower_bound`.
    pub fn previous(&self, i: usize) -> usize {
        if i > self.lower_bound {
            return i-1;
        } else {
            return self.upper_bound;
        }
    }
}

impl Circular2dIndex {
    /// Builder for a `Circular2dIndex`.
    pub fn new(lb_i: usize, ub_i: usize, lb_j: usize, ub_j: usize) -> Self {
        Self { i_index: CircularIndex::new(lb_i, ub_i), j_index: CircularIndex::new(lb_j, ub_j) }
    }

    /// Get index.
    pub fn at(&self, i: usize, j: usize) -> (usize,usize) {
        (self.i_index.at(i), self.j_index.at(j))
    }

    /// Get next on first dimension.
    pub fn next_i(&self, i: usize, j: usize) -> (usize,usize) {
        (self.i_index.next(i),self.j_index.at(j))
    }

    /// Get previous index on first dimention.
    pub fn previous_i(&self, i: usize, j: usize) -> (usize,usize) {
        (self.i_index.previous(i), self.j_index.at(j))
    }

    /// Get next index on second dimension.
    pub fn next_j(&self, i: usize, j: usize) -> (usize,usize) {
        (self.i_index.at(i), self.j_index.next(j))
    }

    /// Get previous index on second dimension.
    pub fn previous_j(&self, i: usize, j: usize) -> (usize,usize) {
        (self.i_index.at(i), self.j_index.previous(j))
    }
}

#[cfg(test)]
mod circular_index_tests {
    use super::*;
    // 
    #[test]
    #[should_panic]
    fn create_one_element_index() {
        let (lb,ub) = (1,1);
        let _ci = CircularIndex::new(lb, ub);
    }

    #[test]
    #[should_panic]
    fn lower_higher_than_upper() {
        let (lb,ub) = (3,1);
        let _ci = CircularIndex::new(lb, ub);
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds() {
        let (lb,ub) = (2,5);
        let ci = CircularIndex::new(lb, ub);
        let _i = ci.at(0);
    }

    #[test]
    fn circular_index_test() {
        let (lb,ub) = (2,5);
        let ci = CircularIndex::new(lb, ub);
        assert_eq!(ci.at(4),4);

        assert_eq!(ci.next(4),5);
        assert_eq!(ci.previous(4),3);
        assert_eq!(ci.next(5),2);
        assert_eq!(ci.previous(2),5);
    }
}

#[cfg(test)]
mod circular_2d_index_tests {
    use super::*;

    #[test]
    fn circular_2d_index_test() {
        // 
        let (lb_i,ub_i,lb_j,ub_j) = (2,3,1,9);
        let ci2d = Circular2dIndex::new(lb_i, ub_i, lb_j, ub_j);
        // 
        assert_eq!(ci2d.at(2, 4),(2,4));
        assert_eq!(ci2d.next_i(2, 4),(3,4));
        assert_eq!(ci2d.next_i(3, 4),(2,4));
        assert_eq!(ci2d.previous_i(2, 1),(3,1));
        assert_eq!(ci2d.next_j(3, 9),(3,1));
        assert_eq!(ci2d.previous_j(2, 1),(2,9));
    }
}