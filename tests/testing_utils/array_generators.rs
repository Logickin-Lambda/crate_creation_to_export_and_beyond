use rand::Rng;

pub struct DataSet {
    pub ctrl_data: Vec<i32>,
    pub test_data: Vec<i32>
}

impl DataSet{
    pub fn new(ctrl_data:Vec<i32> , test_data: Vec<i32>) -> Self{
        Self { 
            ctrl_data, 
            test_data 
        }
    }
}

fn generate_random(size: i32) -> Vec<i32>{
    
    let mut result: Vec<i32> = Vec::new();

    for _ in 0..size {
        result.push(rand::thread_rng().gen_range(0..999));
    }

    result
}

fn sort_for_ctrl(input_arr: Vec<i32>) -> Vec<i32>{

    let mut arr_clone = input_arr.clone();
    arr_clone.sort();
    
    arr_clone
}

pub fn generate_random_n_ctrl(size: i32) -> DataSet{

    let input = generate_random(size);
    let sorted = sort_for_ctrl(input.clone());

    DataSet::new(sorted, input)
}

pub fn generate_nearly_sort_at_back_n_ctrl(size:i32, no_of_unsort: i32) -> DataSet {

    let mut partially_sorted = sort_for_ctrl(generate_random((size - no_of_unsort).clamp(0, size + no_of_unsort)));
    let mut unsorted = generate_random(no_of_unsort);

    partially_sorted.append(&mut unsorted);
    let ctrl_data = sort_for_ctrl(partially_sorted.clone());

    DataSet::new(ctrl_data, partially_sorted)
}