//! This module implements the 2-dimensional Ising model using the Wolff algorithm.


pub struct IsingModel {
    dims: (usize,usize), // dimensions of the computation grid
    grid: Vec<bool>, // the grid, stored as a vector of boolean
}