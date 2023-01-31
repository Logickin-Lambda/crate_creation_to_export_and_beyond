#[cfg(test)]
mod testing_utils;

mod n_or_smaller_algo_test{
    use crate_creation_to_export_and_beyond::counting_sort;
    use crate::testing_utils::array_generators::*;

    #[test]
    fn counting_sort_test(){
        for i in 0..64{
            // Random array test
            let test_data_rnd = generate_random_n_ctrl(16 << (i % 7));
            let rand_arr_test = counting_sort(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);

            // Nearly sorted array test
            let test_data_rnd = generate_nearly_sort_at_back_n_ctrl(16 << (i % 7), i*2);
            let rand_arr_test = counting_sort(&test_data_rnd.test_data);
            assert_eq!(test_data_rnd.ctrl_data, rand_arr_test);           
        }       
    }

}