use std::io;
use std::io::Write;
use std::fmt::Display;

// Prompt the user for an usize.
pub fn get_i32(prompt: &str) -> usize {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<usize>()
        .expect("Error parsing integer");
}

pub fn check_sorted<T>(values: &[T]) -> bool
    where T: PartialOrd
{
    for i in 1..values.len() {
        if values[i - 1] > values[i] {
            return false;
        }
    }
    true
}

pub fn print_vec<T>(vec: &Vec<T>, num_items: usize)
    where T: Copy + Display
{
    let mut max = vec.len();
    if max > num_items {
        max = num_items;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    println!("{string}");
}
