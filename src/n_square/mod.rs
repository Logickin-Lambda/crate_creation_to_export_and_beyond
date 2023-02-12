//! They are the slower algorithms, but these functions actually offer better perfomance over small set of items
//! because they generally have a much simpler structure to do swap quicker than the more complex functions.
pub mod bubble_sort;
pub mod insertion_sort;
pub mod selection_sort;

pub use bubble_sort::bubble_sort;
pub use insertion_sort::{insertion_sort, insertion_sort_unstable};
pub use selection_sort::selection_sort;