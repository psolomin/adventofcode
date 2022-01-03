use crate::exs::utils::read_lines_as_str_vector;

struct CrabPosition {
    x: u32
}

fn find_minimal_fuel_joint_position(
    initial_positions: &Vec<CrabPosition>,
    cost_function: &dyn Fn(&Vec<u32>, u32) -> u32
) -> u32 {
    let x_positions = initial_positions.iter().map(|xp| xp.x).collect();
    let estimate = median(&x_positions);
    let lower_estimate = estimate - 1;
    let upper_estimate = estimate + 1;
    let lower_estimate_cost = cost_function(&x_positions, lower_estimate);
    let center_estimate_cost = cost_function(&x_positions, estimate);
    let upper_estimate_cost = cost_function(&x_positions, upper_estimate);
    return std::cmp::min(
        std::cmp::min(lower_estimate_cost, center_estimate_cost),
        upper_estimate_cost
    )
}

fn find_minimal_fuel_joint_position_brute_force(
    initial_positions: &Vec<CrabPosition>,
    cost_function: &dyn Fn(&Vec<u32>, u32) -> u32
) -> u32 {
    let mut costs: Vec<u32> = Vec::with_capacity(initial_positions.len());
    let xes: Vec<u32> = initial_positions.iter().map(|c| c.x).collect();
    let lower_bound = min(&xes);
    let upper_bound = max(&xes);
    for p in lower_bound..=upper_bound {
        let cost = cost_function(&xes, p);
        costs.push(cost)
    }
    return min(&costs)
}

pub fn day_7() {
    let filename = "data/day-7-the-threachery-of-whales/data-part-1.txt";
    let crabs_data: Vec<u32> = read_lines_as_str_vector(filename)
        .first().unwrap().split(",").map(|d| d.parse::<u32>().unwrap()).collect();

    let crabs: Vec<CrabPosition> = crabs_data.iter()
        .map(|d| CrabPosition { x: *d }).collect();

    let result = find_minimal_fuel_joint_position(
        &crabs, &cost_function);

    let result_brute_force = find_minimal_fuel_joint_position_brute_force(
        &crabs, &cost_function);

    // FIXME: find out why estimates don't work
    println!(
        "Day 7 Part 1 result: {res}; result_brute_force = {rbf}",
        res=result, rbf=result_brute_force
    );

    let result = find_minimal_fuel_joint_position(
        &crabs, &cost_function_arithmetic_progression);

    let result_brute_force = find_minimal_fuel_joint_position_brute_force(
        &crabs, &cost_function_arithmetic_progression);

    println!(
        "Day 7 Part 2 result: {res}; result_brute_force = {rbf}",
        res=result, rbf=result_brute_force
    );
}

fn min(numbers: &Vec<u32>) -> u32 { numbers.iter().min().unwrap().clone() }

fn max(numbers: &Vec<u32>) -> u32 { numbers.iter().max().unwrap().clone() }

fn average(numbers: &Vec<u32>) -> f32 {
    numbers.iter().sum::<u32>() as f32 / numbers.len() as f32
}

fn median(numbers: &Vec<u32>) -> u32 {
    let mut numbers_to_sort = numbers.clone();
    numbers_to_sort.sort();
    let mid = numbers_to_sort.len() / 2;
    return numbers[mid]
}

fn cost_function(numbers: &Vec<u32>, point: u32) -> u32 {
    return numbers.iter().map(|n| (*n as i32 - point as i32).abs()).sum::<i32>() as u32
}

fn cost_function_arithmetic_progression(numbers: &Vec<u32>, point: u32) -> u32 {
    return numbers.iter()
        .map(|n| sum_of_arithmetic_progression_elements(*n, point))
        .sum()
}

fn sum_of_arithmetic_progression_elements(start: u32, end: u32) -> u32 {
    let steps_to_do = (end as i32 - start as i32).abs() as u32;
    return (1 + steps_to_do) * steps_to_do / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_example_data_example() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14].iter()
            .map(|d| CrabPosition { x: *d }).collect();
        assert_eq!(find_minimal_fuel_joint_position(&input, &cost_function), 37);
    }

    #[test]
    fn test_with_example_data_example_brute_force() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14].iter()
            .map(|d| CrabPosition { x: *d }).collect();
        assert_eq!(find_minimal_fuel_joint_position_brute_force(&input, &cost_function), 37);
    }

    #[test]
    fn test_no_moves_needed() {
        let input = vec![16, 16].iter()
            .map(|d| CrabPosition { x: *d }).collect();
        assert_eq!(find_minimal_fuel_joint_position(&input, &cost_function), 0);
    }

    #[test]
    fn test_average() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(average(&input).ceil() as u32, 5);
        assert_eq!(average(&input).floor() as u32, 4);
    }

    #[test]
    fn test_median() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(median(&input), 2);
    }

    #[test]
    fn test_cost_function_arithmetic_progression() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(cost_function_arithmetic_progression(&input, 5), 168);
    }
}
