mod odd_even;
mod test;

pub fn main_fn() -> bool {
    let num = 10;
    odd_even::check_num(num);

    true
}
