//! This modules implements a linear circular index which loops over itself when the
//! upper or lower bound is overshot.

/// The CircularIndex structure holds a lower bound and an upper bound.
pub struct CircularIndex {
    lower_bound: usize,
    upper_bound: usize,
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