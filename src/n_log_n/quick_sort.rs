// It is not necessary to have the struct, but I want to learn a little about how to actually implement one with implementations
struct PartitionPair{
    start: usize,
    end: usize
}

impl PartitionPair{
    pub fn new(start: usize, end:usize) -> Self {
        Self{
            start,
            end,
        }
    }
}

/**
 * split the array into larger and smaller part with the privot.
 * then returns the index of the privot point, and the updated array with the partially sorted version.
 * 
 * return: pivot index in usize and partially sorted array inside a tuple
 * 
 * This quick sort is a mess due to lack of understanding of &mut; I will update it in future once I have complete O(n) algos
 */
fn partitioning(unprocessed_array: &mut &Vec<i32>, start: usize, end: usize) -> (usize, Vec<i32>) {

    // Define where is the part we need to sort for this ieteration, and which is not so that we can construct a complete array
    let mut high_part = Vec::<i32>::new();
    let mut low_part  = Vec::<i32>::new();
    let mut unsort_low_part = Vec::<i32>::new();
    let mut unsort_high_part = Vec::<i32>::new();

    let pivot = unprocessed_array [start];
    let ranged_array = &unprocessed_array[start..=end];
    
    let _ = &unprocessed_array[0..start].clone_into(&mut unsort_low_part);
    let _ = &unprocessed_array[(end + 1)..(unprocessed_array.len())].clone_into(&mut unsort_high_part);

    // Sort the current portion, putting larger element to high_part and vise versa
    for current_number in ranged_array.iter(){

        if *current_number >= pivot {
            high_part.push( *current_number );
        }else{
            low_part.push( *current_number );
        }
    }

    // Summarise every piece of information for next iteration
    let low_part_cnt = low_part.len();

    let mut partially_sorted_array = Vec::<i32>::new();
    partially_sorted_array.append(&mut unsort_low_part);
    partially_sorted_array.append(&mut low_part);
    partially_sorted_array.append(&mut high_part);
    partially_sorted_array.append(&mut unsort_high_part);

    // privot must be after the low part of the whole array, which the equation is start index + number of elements in the low part vec
    (start + low_part_cnt, partially_sorted_array)
}

pub fn quick_sort(input: &Vec<i32>) -> Vec<i32>{

    let mut output = input.clone();

    let mut partition_pair_stack = Vec::<PartitionPair>::new();   // for storing unsorted-partition
    partition_pair_stack.push(PartitionPair::new( 0 as usize, input.len() - 1 as usize));

    while !partition_pair_stack.is_empty() {

        let current_pair = partition_pair_stack.pop().unwrap();
        let sort_result = partitioning(&mut &output, current_pair.start, current_pair.end);

        output = sort_result.1;
        let pivot_index = sort_result.0;

        // push a new index pair UNDER the partition if the remaining element has more then 2 items
        if pivot_index - current_pair.start > 1 {
            partition_pair_stack.push(PartitionPair::new( current_pair.start, pivot_index - 1));
        }

        // push a new index pair OVER the partition if the remaining element has more then 2 items
        if current_pair.end - pivot_index > 1 {
            partition_pair_stack.push(PartitionPair::new( pivot_index + 1, current_pair.end));
        }
    }

    output
}