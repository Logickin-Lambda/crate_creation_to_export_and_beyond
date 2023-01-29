pub fn selection_sort(input: &Vec<i32>) -> Vec<i32> {
    
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