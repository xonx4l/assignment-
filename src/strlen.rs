fn get_string_length(s: &str) -> usize {
    s.chars().count()
}

fn main() {
    let my_string = String::from("Hello, world!");
    let length = get_string_length_chars(&my_string);
    println!("The number of characters in the string is:{}", length);
}
