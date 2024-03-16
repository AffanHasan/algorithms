use std::io;

/*
 * Binary search has complexity (O)Log n
 * This program takes input a user name and searches it in a pre-sorted array
 */
fn main() {
    // a sorted list of names
    let names: [&str; 5] = ["Aazim", "Abdullah", "Abid", "Affan", "Ahmed"];
    let mut name: String = String::new();
    println!("Provide The Name To Search! ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed To Read");
    name = name.trim().to_string();
    let mut low: usize = 0;
    let mut high: usize = 4;
    let mut middle: usize = 0;
    while low <= high {
        middle = (low + high) / 2;
        if &name[..] < names[middle] {
            high = middle - 1;
        } else if &name[..] > names[middle] {
            low = middle + 1;
        } else if &name[..] == names[middle] {
            println!("Name: {name} found at index {middle}");
            break;
        } else {
            println!("Nothing FOund");
        }
    }
}
