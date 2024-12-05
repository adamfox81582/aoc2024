pub mod aoc_data;
pub mod display;

fn is_safe(report: Vec<i32>) -> bool {
    if report.len() < 2 {
        return true
    }

    let mut prev_level = &report[0];
    let mut has_ascended = report[1] > report[0];
    let mut has_descended = report[1] < report[0];

    for level in &report[1..] {
        if has_ascended {
            if level < prev_level {
                return false
            }
        }
        else if has_descended {
            if level > prev_level {
                return false
            }
        }
        else {
            has_ascended = level > prev_level;
            has_descended = level < prev_level;
        }

        let dist = (level - prev_level).abs();
        if dist < 1 || dist > 3 {
            return false
        }

        prev_level = level
    }

    return true
}

fn part1(datafile: &str) -> i32 {
    match aoc_data::read_variable_tuples_from_file(datafile) {
        Ok(tuples) => {
            let mut safe_count = 0;
            for tuple in tuples {
                if is_safe(tuple) {
                    safe_count += 1;
                }
            }

            safe_count
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return -1;
        }
    }
}

fn part2(datafile: &str) -> i32 {
    0
}

fn datafile_path(is_sample_data: bool) -> String {
    aoc_data::datafile_path(2, is_sample_data)
}

fn main() {
    println!("Day 2:");
    println!("  Part 1:");
    println!("    Sample answer: {}", part1(&datafile_path(true)));
    println!("    Real answer:   {}", part1(&datafile_path(false)));
    println!("  Part 2:");
    println!("    Sample answer: {}", part2(&datafile_path(true)));
    println!("    Real answer:   {}", part2(&datafile_path(false)));
}

#[cfg(test)]
mod tests {
    #[test]
    fn day1_part1_sample_data() {
        let p1dist_sample = super::part1(&super::datafile_path(true));
        assert!(p1dist_sample == 2);
    }

    #[test]
    fn day1_part1_user_data() {
        let p1dist_sample = super::part1(&super::datafile_path(false));
        assert!(p1dist_sample == 279);
    }

    #[test]
    fn day1_part2_sample_data() {
        let p2result = super::part2(&super::datafile_path(true));
        assert!(p2result == 0);
    }

    #[test]
    fn day1_part2_real_data() {
        let p2result = super::part2(&super::datafile_path(false));
        assert!(p2result == 0);
    }
}
