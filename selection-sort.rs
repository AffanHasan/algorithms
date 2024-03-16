fn main() {
    let mut names: [&str; 11] = [ "Zaeem", "Aasim", "Aazim", "Aabid", "Affan", "Bahseer", "Babu", "Khanani", "Kaalia", "Asad", "Ahmer" ];
    let mut start_index = 0;
    let mut smallest_element_index;
    loop {
        let mut loop_index = start_index;
        smallest_element_index = start_index;
        // find greatest name
        while loop_index < 11  {
            if names[loop_index] < names[smallest_element_index] {
                smallest_element_index = loop_index;
            }
            loop_index += 1;
        }
       // swap smallest element
       let tmp = names[smallest_element_index];
       names[smallest_element_index] = names[start_index];
       names[start_index] = tmp;
        // shorten the list size
        start_index += 1;
        if start_index == 11 {
            break;
        }
    }

    for element in names {
        println!("{element}");
    }
}
