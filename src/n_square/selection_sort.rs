/// Selection sort picks the smallest item to the next most part of the array.
///
/// # Examples
/// ```
/// use crate_creation_to_export_and_beyond::n_square;
/// 
/// let arg = vec![6,1,8,2,9];
/// let result = n_square::selection_sort(&arg);
/// 
/// assert_eq!(result, vec![1,2,6,8,9]);
/// ```
pub fn selection_sort<T>(input: &Vec<T>) -> Vec<T> 
where T: Clone + PartialOrd
{
    
    let mut sorted = input.clone();

    for start_pt in 0..sorted.len(){
        
        let mut minimum_idx = start_pt;

        for current_num in (start_pt + 1)..sorted.len(){
            if sorted[current_num] < sorted[minimum_idx] {
                minimum_idx = current_num;
            }
        }

        sorted.swap(start_pt, minimum_idx);
    }

    sorted
}