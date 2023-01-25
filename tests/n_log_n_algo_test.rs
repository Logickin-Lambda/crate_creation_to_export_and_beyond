#[cfg(test)]    
mod testing_utils;

mod n_log_n_algo_test{
    use crate_creation_to_export_and_beyond::n_log_n;
    use crate::testing_utils::array_generators::{generate_random_n_ctrl};
    
    #[test]
    fn test_quick_sort(){
        // Random array test
        for i in 0..64 {

            let test_data_rnd = generate_random_n_ctrl(16 << (i % 7));
            let rand_arr_test = n_log_n::quick_sort(&test_data_rnd.testing);
    
            assert_eq!(test_data_rnd.control, rand_arr_test);
        }
    }

    #[test]
    fn test_merge_sort(){
        // Random array test
        for i in 0..64 {

            let test_data_rnd = generate_random_n_ctrl(17 << (i % 7));
            let rand_arr_test = n_log_n::merge_sort(&test_data_rnd.testing);
            
            assert_eq!(test_data_rnd.control, rand_arr_test);
        }
    }
}