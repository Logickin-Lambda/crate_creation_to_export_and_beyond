#[cfg(test)]

mod n_log_n_algo_test{
    use crate_creation_to_export_and_beyond::n_log_n;
    use rand::Rng;

    fn random_array_generator(size: i32) -> Vec<i32>{
        
        let mut result: Vec<i32> = Vec::new();

        for _ in 0..size {
            result.push(rand::thread_rng().gen_range(0..999));
        }

        result
    }

    #[test]
    fn test_quick_sort(){
        // Random array test
        for i in 0..64 {
            let input = random_array_generator(16 << (i % 7));
            let mut target = input.clone();
            
            let output = n_log_n::quick_sort(&input);
            target.sort(); // control test, using the original timsort from rust for comparison

            println!("control: {:?}", &target);
            println!("test   : {:?}", &output);
    
            assert_eq!(output, target);
        }
    }
}