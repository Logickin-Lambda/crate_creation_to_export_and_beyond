/// Being the simpliest algorithm, all it does is to swap to the adjacient item if they are smaller then the current item.
///
/// # Examples
/// ```
/// use crate_creation_to_export_and_beyond::n_square;
/// 
/// let arg = vec![6,1,8,2,9];
/// let result = n_square::bubble_sort(&arg);
/// 
/// assert_eq!(result, vec![1,2,6,8,9]);
/// ```
pub fn bubble_sort<T> ( input: &Vec<T> ) -> Vec<T> 
where T: Clone + PartialOrd
{
    
    let mut output = input.clone();

    for _ in 0..output.len() {
        for j in 0..output.len() -1{
            if output[j] > output[j+1]{
                output.swap(j, j + 1);
            }
        }
    }

    output
}