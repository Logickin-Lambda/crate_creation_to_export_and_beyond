#[cfg(test)]    
mod testing_utils;

mod n_log_n_algo_test{
    use crate_creation_to_export_and_beyond::n_log_n;
    use crate::testing_utils::array_generators::*;
    
    #[test]
    fn test_quick_sort(){
        for i in 0..64 {
            // Random array test
            let test_data_rnd = generate_random_n_ctrl(16 << (i % 7));
            let rand_arr_test = n_log_n::quick_sort(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);

            // Nearly sorted array test
            let test_data_rnd = generate_nearly_sort_at_back_n_ctrl(16 << (i % 7), i*2);
            let rand_arr_test = n_log_n::quick_sort(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);
        }
    }

    #[test]
    fn test_merge_sort(){
        for i in 0..64 {   
            // Random array test
            let test_data_rnd = generate_random_n_ctrl(17 << (i % 7));
            let rand_arr_test = n_log_n::merge_sort(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);

            // Nearly sorted array test
            let test_data_rnd = generate_nearly_sort_at_back_n_ctrl(16 << (i % 7), i*2);
            let rand_arr_test = n_log_n::merge_sort(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);
        }
    }
}