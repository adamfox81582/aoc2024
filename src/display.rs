
use std::{collections::HashMap, fmt};

pub fn tostr_vec_i32(vec: &Vec<i32>) -> String {
    let formatted: Vec<String> = vec
        .iter()
        .map(|x| format!("{}", x))
        .collect();
    format!("[{}]", formatted.join(", "))
}

pub fn tostr_vec_tuple_i32_2(vec: &Vec<(i32, i32)>) -> String {
    let formatted: Vec<String> = vec
        .iter()
        .map(|(x, y)| format!("({}, {})", x, y))
        .collect();
    format!("[{}]", formatted.join(", "))
}

pub fn tostr_hashmap_i32_i32(map: &HashMap<i32, i32>) -> String {
    let formatted: Vec<String> = map
        .iter()
        .map(|(x, y)| format!("({}, {})", x, y))
        .collect();

    format!("[{}]", formatted.join(", "))
}
