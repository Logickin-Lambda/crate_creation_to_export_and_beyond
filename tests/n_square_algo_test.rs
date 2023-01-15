#[cfg(test)]
mod n_square_algo_test{

    use crate_creation_to_export_and_beyond::n_square;
    use rand::Rng;

    fn random_array_generator(size: i32) -> Vec<i32>{
        
        let mut result: Vec<i32> = Vec::new();

        for _ in 0..size {
            result.push(rand::thread_rng().gen_range(0..999));
        }

        result
    }

    #[test]
    fn dummy_test() {
        let addition = 1 + 3;
        assert_eq!(addition, 4);
    }

    #[test]
    fn bubble_sort_test(){
        // Random array test
        for i in 0..64 {
            let input = random_array_generator(16 << (i % 7));
            let mut target = input.clone();
            
            let output = n_square::bubble_sort(&input);
            target.sort(); // control test, using the original timsort from rust for comparison
    
            assert_eq!(output, target);
        }
    }
    
    #[test]
    fn insertion_sort_test(){
        // Random array test
        for i in 0..64{
            let input = random_array_generator(16 << (i % 7));
            let mut target = input.clone();
            
            let output = n_square::insertion_sort(&input);
            target.sort(); // control test, using the original timsort from rust for comparison
    
            assert_eq!(output, target);
        }
    }

    #[test]
    fn insertion_sort_unstable_test(){
        // Random array test
        for i in 0..64{
            let input = random_array_generator(16 << (i % 7));
            let mut target = input.clone();
            
            let output = n_square::insertion_sort_unstable(&input);
            target.sort(); // control test, using the original timsort from rust for comparison
    
            assert_eq!(output, target);
        }
    }
}