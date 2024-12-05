pub mod aoc_data;
pub mod display;

use std::collections::HashMap;

fn sort_individual_dimensions_2vec(tuples: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut first_nums = tuples.clone().into_iter().map(|(first, _)| first).collect::<Vec<i32>>();
    let mut second_nums = tuples.clone().into_iter().map(|(_, second)| second).collect::<Vec<i32>>();
    first_nums.sort();
    second_nums.sort();
    let recombined: Vec<(i32, i32)> = first_nums.into_iter().zip(second_nums.into_iter()).collect();
    return recombined
}

fn calc_dist_with_presorted(presorted: Vec<(i32, i32)>) -> i32 {
    let mut dist = 0;
    for element in presorted {
        dist += (element.1 - element.0).abs();
    }
    return dist;
}

fn day1_part1(datafile: &str) -> i32 {
    match aoc_data::read_tuples_from_file(datafile) {
        Ok(tuples) => {
            let sorted = sort_individual_dimensions_2vec(tuples);
            let dist = calc_dist_with_presorted(sorted);
            return dist;
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return -1;
        }
    }
}

fn make_occurences_map(vec: Vec<i32>) -> HashMap<i32, i32> {
    let mut occurences: HashMap<i32, i32> = HashMap::new();
    for value in vec {
        *occurences.entry(value).or_insert(0) += 1;
    }
    occurences
}

fn similarity_score(l: Vec<i32>, r: Vec<i32>) -> i32 {
    let occurrences_map = make_occurences_map(r.clone());
    let mut similarity_score = 0;
    for value in l.clone() {
        if occurrences_map.contains_key(&value) {
            similarity_score += value * occurrences_map[&value];
        }
    }

    similarity_score
}

fn day1_part2(datafile: &str) -> i32 {
    match aoc_data::read_tuples_from_file(datafile) {
        Ok(tuples) => {
            let first_nums = tuples.clone().into_iter().map(|(first, _)| first).collect::<Vec<i32>>();
            let second_nums = tuples.clone().into_iter().map(|(_, second)| second).collect::<Vec<i32>>();
            return similarity_score(first_nums, second_nums);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return -1;
        }
    }
}

fn datafile_path(is_sample_data: bool) -> String {
    aoc_data::datafile_path(1, is_sample_data)
}

fn main() {
    println!("Day 1:");
    println!("  Part 1:");
    println!("    Sample answer: {}", day1_part1(&datafile_path(true)));
    println!("    Real answer:   {}", day1_part1(&datafile_path(false)));
    println!("  Part 2:");
    println!("    Sample answer: {}", day1_part2(&datafile_path(true)));
    println!("    Real answer:   {}", day1_part2(&datafile_path(false)));
}

#[cfg(test)]
mod tests {
    #[test]
    fn day1_part1_sample_data() {
        let p1dist_sample = super::day1_part1(&super::datafile_path(true));
        assert!(p1dist_sample == 11);
    }

    #[test]
    fn day1_part1_user_data() {
        let p1dist_sample = super::day1_part1(&super::datafile_path(false));
        assert!(p1dist_sample == 765748);
    }

    #[test]
    fn day1_part2_sample_data() {
        let p2result = super::day1_part2(&super::datafile_path(true));
        assert!(p2result == 31);
    }

    #[test]
    fn day1_part2_real_data() {
        let p2result = super::day1_part2(&super::datafile_path(false));
        assert!(p2result == 27732508);
    }
}
