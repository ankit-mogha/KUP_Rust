mod test;

pub mod even_odd {
    pub mod odd_even;
}

pub fn main_fn() -> bool {
    even_odd::odd_even::check_num(10);

    true
}
