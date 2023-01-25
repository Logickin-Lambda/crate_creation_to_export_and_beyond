use rand::Rng;

pub struct DataSet {
    pub control: Vec<i32>,
    pub testing: Vec<i32>
}

impl DataSet{
    pub fn new(control:Vec<i32> , testing: Vec<i32>) -> Self{
        Self { 
            control, 
            testing 
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

fn generate_ctrl(input_arr: Vec<i32>) -> Vec<i32>{

    let mut arr_clone = input_arr.clone();
    arr_clone.sort();
    
    arr_clone
}

pub fn generate_random_n_ctrl(size: i32) -> DataSet{

    let input = generate_random(size);
    let sorted = generate_ctrl(input.clone());

    DataSet::new(sorted, input)
}

