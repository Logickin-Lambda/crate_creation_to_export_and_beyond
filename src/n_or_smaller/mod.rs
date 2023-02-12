//! The following contains some of the fastest sorting algorithms which the offers O (n) time complexity;
//! however, they are not the most practical solutions because they usually required more memory to run,
//! or they have limited type of data can be sorted. 
pub mod counting_sort;
pub mod radix_sort;

pub use counting_sort::counting_sort;
pub use radix_sort::radix_sort;