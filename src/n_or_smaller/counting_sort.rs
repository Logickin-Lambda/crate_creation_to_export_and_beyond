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