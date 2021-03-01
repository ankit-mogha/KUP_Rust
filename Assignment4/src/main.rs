mod check_palindrome;
mod duplicate_characters;
mod match_string;

fn main() {
    println!("Strings : {}",match_string::check_str_rev("abcd","dcba"));
    println!("String is : {}",check_palindrome::check_palindrome("acba"));
    println!("Duplicate Characters : {}",duplicate_characters::find_rep_char("Hello World"));
}
