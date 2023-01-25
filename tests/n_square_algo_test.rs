#[cfg(test)]
mod testing_utils;

mod n_square_algo_test{

    use crate_creation_to_export_and_beyond::n_square;
    use crate::testing_utils::array_generators::{generate_random_n_ctrl};

    #[test]
    fn dummy_test() {
        let addition = 1 + 3;
        assert_eq!(addition, 4);
    }

    #[test]
    fn bubble_sort_test(){
        // Random array test
        for i in 0..64 {

            let test_data_rnd = generate_random_n_ctrl(16 << (i % 7));
            let rand_arr_test = n_square::bubble_sort(&test_data_rnd.testing);
    
            assert_eq!(test_data_rnd.control, rand_arr_test);
        }
    }
    
    #[test]
    fn insertion_sort_test(){
        // Random array test
        for i in 0..64{

            let test_data_rnd = generate_random_n_ctrl(16 << (i % 7));
            let rand_arr_test = n_square::insertion_sort(&test_data_rnd.testing);
    
            assert_eq!(test_data_rnd.control, rand_arr_test);
        }
    }

    #[test]
    fn insertion_sort_unstable_test(){
        // Random array test
        for i in 0..64{

            let test_data_rnd = generate_random_n_ctrl(16 << (i % 7));
            let rand_arr_test = n_square::insertion_sort_unstable(&test_data_rnd.testing);
    
            assert_eq!(test_data_rnd.control, rand_arr_test);
        }
    }
}