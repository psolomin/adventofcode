use super::utils::read_lines_as_str_vector;

fn calc_final_pos_and_depth(movements: &Vec<String>) -> (i32, i32) {
    let mut horizontal_position: i32 = 0;
    let mut depth: i32 = 0;
    for movement in movements {
        let (command, coordinate) = parse_into_movement(movement);
        if command.eq("forward") { horizontal_position += coordinate }
        if command.eq("down") { depth += coordinate }
        if command.eq("up") {
            depth -= coordinate;
            if depth < 0 { depth = 0 }
        }
    }
    return (horizontal_position, depth)
}

fn calc_final_pos_and_depth_with_aim(movements: &Vec<String>) -> (i32, i32) {
    let mut horizontal_position: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    for movement in movements {
        let (command, coordinate) = parse_into_movement(movement);
        if command.eq("forward") {
            horizontal_position += coordinate;
            depth += aim * coordinate
        }
        if command.eq("down") {
            aim += coordinate
        }
        if command.eq("up") {
            aim -= coordinate
        }
    }
    return (horizontal_position, depth)
}

pub fn day_2() {
    let filename = "data/day-2-dive/data-part-1.txt";
    let movements = read_lines_as_str_vector(filename);
    let result = calc_final_pos_and_depth(&movements);
    println!("Day 2 Part 1 result: {res}", res=result.0 * result.1);
    let result2 = calc_final_pos_and_depth_with_aim(&movements);
    println!("Day 2 Part 2 result: {res}", res=result2.0 * result2.1);
}

fn parse_into_movement(movement: &String) -> (&str, i32) {
    let mut split = movement.split_whitespace();
    let command = split.next().unwrap();
    let coordinate = split.next().unwrap().parse::<i32>().unwrap();
    return (command, coordinate);
}

#[cfg(test)]
mod tests {
    use crate::exs::utils::strs_to_strings;
    use super::*;

    #[test]
    fn test_calc_final_pos_and_depth() {
        let inputs = strs_to_strings(&vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2"
        ]);

        assert_eq!(calc_final_pos_and_depth(&inputs), (15, 10));
    }

    #[test]
    fn test_calc_final_pos_and_depth_with_aim() {
        let inputs = strs_to_strings(&vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2"
        ]);

        assert_eq!(calc_final_pos_and_depth_with_aim(&inputs), (15, 60));
    }
}
