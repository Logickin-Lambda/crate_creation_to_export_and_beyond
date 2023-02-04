// no numeric generics for now until I have learnt unsafe rust
pub fn radix_sort(input: &Vec<i32>) -> Vec<i32>{

    if input.len() == 0 {return input.clone();}
    
    let mut output = input.clone();
    let max_num = *input.iter().max().unwrap();
    let mut sig_digs = 1;

    while max_num > sig_digs {
        internal_counting_sort(&mut output, sig_digs);
        sig_digs *= 10;
    }

    output
}

fn internal_counting_sort(input: &mut Vec<i32>, sig_digs: i32){

    let input_clone = input.clone();
    let vec_size = input_clone.len() as usize;
    let mut digit_buffer = [0; 10];
    
    // extract digits
    let mut input_digit_vec = Vec::<i32>::new();
    for i in 0..vec_size{
        let digit = ( input_clone[i] / sig_digs ) % 10;
        input_digit_vec.push(digit);
    }

    // counting digits
    for i in 0..vec_size{
        let count_idx = input_digit_vec [i] as usize; 
        digit_buffer[count_idx] += 1;
    }

    // finding the cumulative count for the digit array
    for i in 1..10 as usize{
        digit_buffer[i] = digit_buffer[i - 1] + digit_buffer[i];
    }

    // return the partially sorted array
    for i in (0..vec_size).rev(){

        let cur_num = input_clone[i];
        let cur_digi = input_digit_vec[i] as usize;
        
        let new_idx = digit_buffer[cur_digi] as usize - 1;
        input[new_idx] = cur_num;
        digit_buffer[cur_digi] = new_idx;
    }
}