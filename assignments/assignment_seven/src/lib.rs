mod test;

pub mod ques1 {
    pub mod pattern_search;
    pub mod substring_generate;
}

pub fn main_fn() -> bool {
    let string = "Pankaj Chaudhary";
    let pattern = "Cha";
    ques1::pattern_search::pattern_check(string, pattern);
    ques1::substring_generate::sub_string_find("ab");

    true
}
