#[cfg(test)]
mod testing_utils;

mod n_square_algo_test{

    use crate_creation_to_export_and_beyond::n_square;
    use crate::testing_utils::array_generators::{generate_random_n_ctrl, generate_nearly_sort_at_back_n_ctrl};

    #[test]
    fn bubble_sort_test(){
        for i in 0..64 {
            // Random array test
            let test_data_rnd = generate_random_n_ctrl(16 << (i % 7));
            let rand_arr_test = n_square::bubble_sort(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);

            // Nearly sorted array test
            let test_data_rnd = generate_nearly_sort_at_back_n_ctrl(16 << (i % 7), i*2);
            let rand_arr_test = n_square::bubble_sort(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);
        }
    }
    
    #[test]
    fn insertion_sort_test(){
        for i in 0..64{
            // Random array test
            let test_data_rnd = generate_random_n_ctrl(16 << (i % 7));
            let rand_arr_test = n_square::insertion_sort(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);

            // Nearly sorted array test
            let test_data_rnd = generate_nearly_sort_at_back_n_ctrl(16 << (i % 7), i*2);
            let rand_arr_test = n_square::insertion_sort(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);
        }
    }

    #[test]
    fn insertion_sort_unstable_test(){
        for i in 0..64{        
            // Random array test
            let test_data_rnd = generate_random_n_ctrl(16 << (i % 7));
            let rand_arr_test = n_square::insertion_sort_unstable(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);

            // Nearly sorted array test
            let test_data_rnd = generate_nearly_sort_at_back_n_ctrl(16 << (i % 7), i*2);
            let rand_arr_test = n_square::insertion_sort_unstable(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);
        }
    }

    #[test]
    fn selection_sort_test(){
        for i in 0..64{
            // Random array test
            let test_data_rnd = generate_random_n_ctrl(16 << (i % 7));
            let rand_arr_test = n_square::selection_sort(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);

            // Nearly sorted array test
            let test_data_rnd = generate_nearly_sort_at_back_n_ctrl(16 << (i % 7), i*2);
            let rand_arr_test = n_square::selection_sort(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);           
        }       
    }
}