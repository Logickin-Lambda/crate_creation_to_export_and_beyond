use std::ops::{Range};

pub fn merge_sort(input: &Vec<i32>) -> Vec<i32> {

    let mut segment_size = 2;
    let mut input_clone = input.clone();

    while segment_size / 2 <= input.len() {

        let mut idx: usize = segment_size;

        while idx < input.len() + segment_size{

            let list_l_start = idx - segment_size;
            let list_r_start = (idx - (segment_size / 2)).clamp(0, input.len());
            let list_r_end = idx.clamp(0, input.len());

            input_clone = merge(&mut input_clone, list_l_start..list_r_start, list_r_start..list_r_end);
            
            idx += segment_size;
        }

        segment_size *= 2;
    }
    
    input_clone
}

fn merge(input: &mut Vec<i32>, list_l_rng: Range<usize>, list_r_rng: Range<usize>) -> Vec<i32>{

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

    input.to_vec()
}