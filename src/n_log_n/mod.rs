//! Here are the sorting algorithms with the O(n log n) time complexity; usually, these are some of the most practical algorithms,
//! which they offers a reasonable running time with reasonable storage size.
pub mod quick_sort;
pub mod merge_sort;

pub use quick_sort::quick_sort;
pub use merge_sort::merge_sort;