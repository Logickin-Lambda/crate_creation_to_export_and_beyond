pub fn insertion_sort<T>(input :&Vec<T>) -> Vec<T>
where T: Clone + PartialOrd
{
    let mut output = input.clone();

    for i in 0..output.len(){
        for j in (0..i as usize).rev(){
            if output[j+1] >= output[j] { break }
            output.swap(j, j+1);
        }
    }

    output
}

pub fn insertion_sort_unstable<T>(input :&Vec<T>) -> Vec<T>
where T: Clone + Copy + PartialOrd
{
    let mut output = Vec::<T>::new();

    for i in 0..input.len(){
        let index = find_insert_index_by_binary_search(&output, input[i]);
        
        // if the index points after the last element, push the element instead of insert
        if index == output.len() {
            output.push(input[i]);
        }else{
            output.insert(index, input[i]);
        }

    }

    output
}

fn find_insert_index_by_binary_search<T>(sorted_section :&Vec<T>, current_num: T) -> usize
where T: Clone + PartialOrd
{

    if sorted_section.len() == 0{
        return 0;
    }

    let mut low_index = 0 as usize;
    let mut high_index = sorted_section.len() as usize;
    let mut mid_point = 0 as usize;
    let mut prev_mid_point = 0 as usize;

    for _ in 0..16 {
        mid_point = ((low_index + high_index) / 2) as usize;
        if current_num == sorted_section[mid_point]{
            mid_point += 1;
            break;
        }else if prev_mid_point == mid_point && current_num < sorted_section[mid_point]{
            break;  // swap the current midpoint with the current number
        }else if prev_mid_point == mid_point && current_num > sorted_section[mid_point]{
            mid_point += 1;
            break;  // directly add the current number after the midpoint
        }else if current_num < sorted_section[mid_point] && mid_point > 0 {
            high_index = mid_point - 1;
        }else if current_num > sorted_section[mid_point] && mid_point < sorted_section.len() - 1 { // preventing index out of bound
            low_index = mid_point + 1;
        }

        prev_mid_point = mid_point;
    }

    mid_point
}


#[test]
fn find_insert_index_by_binary_search_test(){
    let search_arr = vec![2,3,3,5,6,7,7,9,10];

    println!("found, unique");
    let insert_index = find_insert_index_by_binary_search(&search_arr, 5);
    assert_eq!(insert_index, 4 as usize);

    println!("found, duplicated");
    let insert_index = find_insert_index_by_binary_search(&search_arr, 3);
    assert!((1..=2).contains(&insert_index));

    println!("not found, middle");
    let insert_index = find_insert_index_by_binary_search(&search_arr, 4);
    assert_eq!(insert_index, 3 as usize);

    println!("not found, smallest");
    let insert_index = find_insert_index_by_binary_search(&search_arr, 1);
    assert_eq!(insert_index, 0 as usize);

    println!("not found, largest");
    let insert_index = find_insert_index_by_binary_search(&search_arr, 11);
    assert_eq!(insert_index, 9 as usize);
}