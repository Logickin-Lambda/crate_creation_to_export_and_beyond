// no numeric generics for now until I have learnt unsafe rust

/// Counting sort generates a counter array based on the largest number,
/// then simply count all the items from the unsort list.
/// The result is obtained by coping the items n times according to the counter.
/// 
/// However, this counting sort cannot support generics and negative numbers.
/// # Examples
/// ```
/// use crate_creation_to_export_and_beyond::n_or_smaller;
/// 
/// let arg = vec![6,1,8,2,9];
/// let result = n_or_smaller::counting_sort(&arg);
/// 
/// assert_eq!(result, vec![1,2,6,8,9]);
/// 
/// ```
pub fn counting_sort(input: &Vec<i32>) -> Vec<i32>{

    let max_number = *input.iter().max().unwrap_or(&0) as usize + 1;
    let mut counter_vec = vec![0; max_number];

    for current_num in input{

        let cnt_idx = *current_num as usize;
        counter_vec[cnt_idx] += 1;
    }

    let mut output = Vec::<i32>::new();

    for enumerator in counter_vec.iter().enumerate(){
        let mut temp = vec![enumerator.0 as i32; *enumerator.1];
        output.append(&mut temp);
    }

    output
}