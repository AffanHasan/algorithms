fn main() {
    let numbers: [usize; 10] = [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ];
    println!("Sum = {}", sum(&numbers[0..0]));
}

fn sum(slice: &[usize]) -> usize {
    //base case
    if slice.len() == 0 {
        0
    } else if slice.len() == 1 {
        slice[0]
    } else { // recursive case
        slice[0] + sum(&slice[1..])
    }        
}

