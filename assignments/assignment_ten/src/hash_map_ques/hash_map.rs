use std::collections::HashMap;

/// sum_conditionals function matches a particular pattern in Key of hashmap.
///
/// #Arguments
///
/// map : its a Hashmap have key as string type and values as i32 type.
///
/// str : pattern that need to be searched in key of hashmap.
///
/// #Return
///
/// Returns i32 that tells total of the age of persons have that particular pattern in it.

pub fn sum_conditional(map: &HashMap<String, i32>, str: String) -> i32 {
    let mut sum_age = 0;
    let mut key_set: Vec<String> = Vec::new();
    for key in map.keys() {
        key_set.push(key.to_string());
    }
    let mut counter = 0;
    while counter < key_set.len() {
        if key_set[counter].contains(&str.to_string()) {
            let age = map.get(&key_set[counter]);
            match age {
                Some(age) => {
                    sum_age += age;
                }
                None => println!("Not found"),
            }
        }
        counter += 1;
    }
    log::info!("Sum : {}", sum_age);
    sum_age
}
