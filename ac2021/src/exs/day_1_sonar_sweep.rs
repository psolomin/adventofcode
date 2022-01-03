use super::utils::read_lines_as_vector;

fn count_increases(measurements: &Vec<i32>) -> i32 {
    let mut previous: i32 = 0;
    let mut cnt: i32 = 0;
    let mut first_is_processed: bool = false;

    for measurement in measurements {
        if first_is_processed {
            if *measurement > previous {
                cnt += 1
            }
        } else {
            first_is_processed = true
        }
        previous = *measurement;
    }
    return cnt;
}

fn count_increases_sliding_window(measurements: &Vec<i32>) -> i32 {
    let mut cnt: i32 = 0;

    let mut pos: i32 = 0;
    let window_size: i32 = 3;
    for _measurement in measurements {
        let previous_window_lower_bound = pos - window_size;
        let previous_window_upper_bound = pos;
        let current_window_lower_bound = previous_window_lower_bound + 1;
        let current_window_upper_bound = previous_window_upper_bound + 1;

        if previous_window_lower_bound >= 0 {

            let previous_window = &measurements[
                previous_window_lower_bound as usize..previous_window_upper_bound as usize];

            let current_window = &measurements[
                current_window_lower_bound as usize..current_window_upper_bound as usize];

            let previous_window_sum: i32 = previous_window.iter().sum();
            let current_window_sum: i32 = current_window.iter().sum();

            if current_window_sum > previous_window_sum { cnt += 1 }
        }
        pos += 1;
        if pos > current_window_upper_bound { break; }
    }
    return cnt;
}

pub fn day_1() {
    let filename = "data/day-1-sonar-sweep/data-part-1.txt";
    let measurements = read_lines_as_vector(filename);
    let result = count_increases(&measurements);
    println!("Day 1 Part 1 result: {res}", res=result);

    let filename = "data/day-1-sonar-sweep/data-part-2.txt";
    let measurements = read_lines_as_vector(filename);
    let result = count_increases_sliding_window(&measurements);
    println!("Day 1 Part 2 result: {res}", res=result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases() {
        assert_eq!(count_increases(&vec![1, 2, 5, 3, 8]), 3);
        assert_eq!(count_increases(&vec![1, 2, 5, 3, 2]), 2);
    }

    #[test]
    fn test_count_increases_sliding_window() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases_sliding_window(&input), 5);
    }
}
