pub fn bubble_sort<T> ( input: &Vec<T> ) -> Vec<T> 
where T: Clone + PartialOrd
{
    
    let mut output = input.clone();

    for _ in 0..output.len() {
        for j in 0..output.len() -1{
            if output[j] > output[j+1]{
                output.swap(j, j + 1);
            }
        }
    }

    output
}