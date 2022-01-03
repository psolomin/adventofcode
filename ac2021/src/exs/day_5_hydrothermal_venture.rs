use std::collections::HashMap;
use crate::exs::utils::{open_range_vec, read_lines_as_str_vector};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: u32,
    y: u32
}

impl Point {
    fn from_string(input: String) -> Point {
        let pair: Vec<u32> = input.split(",")
            .map(|d| d.parse::<u32>().unwrap()).collect();
        return Point { x: *pair.first().unwrap(), y: *pair.last().unwrap() }
    }
}

#[derive(Clone, Debug)]
struct LineOfVents {
    start: Point,
    end: Point,
    line: Vec<Point>
}

impl LineOfVents {
    fn from_string(input: &String) -> LineOfVents {
        let start_and_end: Vec<Point> = input.split(" -> ")
            .map(|p| Point::from_string(p.to_string())).collect();

        let start = start_and_end.first().unwrap().clone();
        let end = start_and_end.last().unwrap().clone();
        return LineOfVents { start, end, line: LineOfVents::build_line(start, end) }
    }

    fn build_line(start: Point, end: Point) -> Vec<Point> {
        if start.x == end.x { LineOfVents::_build_vertical_line(start, end) }
        else if start.y == end.y { LineOfVents::_build_horizontal_line(start, end) }
        else { LineOfVents::_build_diagonal_line(start, end) }
    }

    fn _build_horizontal_line(start: Point, end: Point) -> Vec<Point> {
        open_range_vec(start.x, end.x).iter()
            .map(|x| Point {x: *x, y: end.y}).collect()
    }

    fn _build_vertical_line(start: Point, end: Point) -> Vec<Point> {
        open_range_vec(start.y, end.y).iter()
            .map(|y| Point {x: end.x, y: *y}).collect()
    }

    fn _build_diagonal_line(start: Point, end: Point) -> Vec<Point> {
        // TODO: simplify
        let mut line: Vec<Point> = Vec::new();
        let xes = open_range_vec(start.x, end.x);
        let mut ycs = open_range_vec(start.y, end.y).into_iter();

        for x in xes {
            // unwrap() will panic if the line is not truly diagonal
            line.push (Point { x, y: ycs.next().unwrap().clone() })
        }
        return line;
    }
}

fn parse_input(nearby_lines_encoded: &Vec<String>) -> Vec<LineOfVents> {
    return nearby_lines_encoded.iter()
        .map(|s| LineOfVents::from_string(s))
        .collect()
}

fn count_points_where_lines_overlap(nearby_lines: &Vec<LineOfVents>) -> u32 {
    let mut points_with_lines_count: HashMap<Point, u32> = HashMap::new();
    for line in nearby_lines {
        for point in &line.line {
            let existing = points_with_lines_count.entry(point.clone()).or_insert(0);
            *existing += 1
        }
    }
    return points_with_lines_count.values().filter(|v| **v >= 2u32).count() as u32;
}

fn keep_horizontal_and_vertical_only(nearby_lines: &Vec<LineOfVents>) -> Vec<LineOfVents> {
    return nearby_lines.iter().filter(
        |lov| (lov.start.x == lov.end.x) || (lov.start.y == lov.end.y)
    ).map(|lov| lov.clone()).collect()
}

fn parse_and_count_points_where_lines_overlap(
    nearby_lines_encoded: &Vec<String>, drop_diagonals: bool
) -> u32 {
    let parsed = parse_input(nearby_lines_encoded);
    return if drop_diagonals {
        count_points_where_lines_overlap(
            &keep_horizontal_and_vertical_only(&parsed))
    } else {
        count_points_where_lines_overlap(&parsed)
    }
}

pub fn day_5() {
    let filename = "data/day-5-hydrothermal-venture/data-part-1.txt";
    let lines = read_lines_as_str_vector(filename);
    let result = parse_and_count_points_where_lines_overlap(&lines, true);
    println!("Day 5 Part 1 result: {res}", res=result);
    let result = parse_and_count_points_where_lines_overlap(&lines, false);
    println!("Day 5 Part 2 result: {res}", res=result);
}

#[cfg(test)]
mod tests {
    use crate::exs::utils::strs_to_strings;
    use super::*;

    #[test]
    fn test_data_from_example() {
        let inputs = strs_to_strings(&vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2"
        ]);

        assert_eq!(parse_and_count_points_where_lines_overlap(&inputs, true), 5);
    }

    #[test]
    fn test_data_from_example_with_diagonals() {
        let inputs = strs_to_strings(&vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2"
        ]);

        assert_eq!(parse_and_count_points_where_lines_overlap(&inputs, false), 12);
    }

    #[test]
    fn test_horizontal_only() {
        let inputs = strs_to_strings(&vec![
            "0,9 -> 2,9",
            "0,9 -> 2,9",
            "0,9 -> 2,9"
        ]);

        assert_eq!(parse_and_count_points_where_lines_overlap(&inputs, true), 3);
    }

    #[test]
    fn test_horizontal_only_with_descent() {
        let inputs = strs_to_strings(&vec![
            "8,0 -> 0,8",
            "0,0 -> 8,8"
        ]);

        assert_eq!(parse_and_count_points_where_lines_overlap(&inputs, false), 1);
    }

    #[test]
    fn test_vertical_only() {
        let inputs = strs_to_strings(&vec![
            "0,9 -> 0,10",
            "0,8 -> 0,11",
        ]);

        assert_eq!(parse_and_count_points_where_lines_overlap(&inputs, true), 2);
    }

    #[test]
    fn test_diagonal_only() {
        let inputs = strs_to_strings(&vec![
            "0,0 -> 5,5",
            "5,5 -> 0,0",
        ]);

        assert_eq!(parse_and_count_points_where_lines_overlap(&inputs, false), 6);
    }

    #[test]
    fn test_mix() {
        let inputs = strs_to_strings(&vec![
            "0,0 -> 1,0",
            "0,0 -> 0,1",
            "0,0 -> 1,1",
            "0,0 -> 5,5",
            "1,1 -> 0,0"
        ]);

        assert_eq!(parse_and_count_points_where_lines_overlap(&inputs, false), 2);
    }

    #[test]
    fn points_equality() {
        let p = Point {x: 0 , y: 0};
        assert_eq!(p, p);
        assert_eq!(p, Point {x: 0 , y: 0});
        assert_ne!(p, Point {x: 0 , y: 1});
    }
}
