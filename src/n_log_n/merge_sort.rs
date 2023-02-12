use std::ops::{Range};

/// Merge sort sorts thing by spliting the unsorted list into smaller sublist;
/// it groups the sublist by pairs then merging the them with taking the priority of smaller elements in one of the sublist.
/// 
/// For example, if the current sublist are [5, 7] [1, 8], the merge sequence will be the following:
/// ```text
/// [5, 7] [1, 8] -> []
/// [5, 7] [8]    -> [1]
/// [7] [8]       -> [1, 5]
/// [] [8]        -> [1, 5, 7]
/// [] []         -> [1, 5, 7, 8]
/// ```
/// # Examples
/// ```
/// use crate_creation_to_export_and_beyond::n_log_n;
/// 
/// let arg = vec![6,1,8,2,9];
/// let result = n_log_n::merge_sort(&arg);
/// 
/// assert_eq!(result, vec![1,2,6,8,9]);
/// 
/// ```
pub fn merge_sort<T>(input: &Vec<T>) -> Vec<T> 
where T: Clone + Copy + PartialOrd
{

    let mut segment_size = 2;
    let mut input_clone = input.clone();

    while segment_size / 2 <= input.len() {

        let mut idx: usize = segment_size;

        while idx < input.len() + segment_size{

            let list_l_start = idx - segment_size;
            let list_r_start = (idx - (segment_size / 2)).clamp(0, input.len());
            let list_r_end = idx.clamp(0, input.len());

            merge(&mut input_clone, list_l_start..list_r_start, list_r_start..list_r_end);
            
            idx += segment_size;
        }

        segment_size *= 2;
    }
    
    input_clone
}

fn merge<T>(input: &mut Vec<T>, list_l_rng: Range<usize>, list_r_rng: Range<usize>)
where T: Clone + Copy + PartialOrd
{

    let list_l = &input.clone()[list_l_rng.clone()];
    let list_r = &input.clone()[list_r_rng.clone()];

    let mut list_l_idx = 0 as usize;
    let mut list_r_idx = 0 as usize;

    for arr_idx in list_l_rng.start..list_r_rng.end{ 

        if list_l_idx >= list_l.len(){
            input[arr_idx] = list_r[list_r_idx];
            list_r_idx += 1;
            
        }else if list_r_idx >= list_r.len(){
            input[arr_idx] = list_l[list_l_idx];
            list_l_idx += 1;

        }else if list_l[list_l_idx] > list_r[list_r_idx]{
            input[arr_idx] = list_r[list_r_idx];
            list_r_idx += 1;
            
        }else if list_l[list_l_idx] <= list_r[list_r_idx]{
            input[arr_idx] = list_l[list_l_idx];
            list_l_idx += 1;

        }
    }
}