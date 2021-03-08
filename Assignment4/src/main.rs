mod check_palindrome;
mod duplicate_characters;
mod match_string;

///Main function is used to call all the other functions.
fn main() {
    println!("Strings : {}", match_string::check_str_rev("abcd", "dcba"));
    println!("String is : {}", check_palindrome::check_palindrome("abba"));
    println!(
        "Duplicate Characters : {}",
        duplicate_characters::find_rep_char("Hello World")
    );
}
