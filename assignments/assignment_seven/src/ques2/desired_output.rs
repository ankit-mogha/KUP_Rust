/// desired_output_fn function gets desired output for even and odd position of strings.
///
/// #Arguments
///
/// first_str - String type argument.
/// second_str - String type argument.
/// third_str - String type argument.
///
/// #Return
///
/// Returns a String.

pub fn desired_output_fn(first_str: &str, second_str: &str, third_str: &str) -> String {
    let mut count = 0;
    let mut index: usize = 0;
    let mut result_string: String = String::new();

    while index < first_str.len() && index < second_str.len() && index < second_str.len() {
        match count % 2 == 0 {
            true => {
                let comp_res_str1_str2 =
                    if first_str.chars().nth(index) < second_str.chars().nth(index) {
                        first_str.chars().nth(index)
                    } else {
                        second_str.chars().nth(index)
                    };
                let res_char = if comp_res_str1_str2 < third_str.chars().nth(index) {
                    comp_res_str1_str2
                } else {
                    third_str.chars().nth(index)
                };
                if let Some(_t) = res_char {
                    result_string.push(res_char.unwrap())
                }
            }

            false => {
                let comp_res_str1_str2 =
                    if first_str.chars().nth(index) < second_str.chars().nth(index) {
                        second_str.chars().nth(index)
                    } else {
                        first_str.chars().nth(index)
                    };
                let result = if comp_res_str1_str2 > third_str.chars().nth(index) {
                    comp_res_str1_str2
                } else {
                    third_str.chars().nth(index)
                };
                if let Some(_t) = result {
                    result_string.push(result.unwrap())
                }
            }
        }

        index += 1;
        count += 1;
    }
    result_string
}
