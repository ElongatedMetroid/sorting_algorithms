#![feature(array_windows)]
pub mod bubble_sort;

pub trait SortingAlgorithm {
    /// Do one step in sorting, should return true when completed sorting
    fn step(&mut self) -> bool;
    /// Repeadedly step until fully sorted
    fn sort(&mut self);
}
