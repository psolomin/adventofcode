use std::collections::HashSet;
use super::matrix::Matrix;

use super::utils::read_lines_as_str_vector;

#[derive(Debug)]
struct HeightMap {
    points: Vec<Vec<Point>>,
    n_rows: usize,
    n_columns: usize
}

impl HeightMap {
    fn new(rows: &Vec<String>) -> HeightMap {
        let m = Matrix::<u32>::from_lines(rows);
        let n_rows = m.n_rows();
        let n_columns = m.n_columns();
        let points: Vec<Vec<Point>> = (0..n_rows).map(
            |row_id| (0..n_columns).map(
                |col_id| Point {
                    x: col_id as u32,
                    y: row_id as u32,
                    height: m.get_point(col_id as i32, row_id as i32).unwrap(),
                    is_low: None
                }
            ).collect()
        ).collect();

        return HeightMap { points, n_rows, n_columns }
    }

    fn get_point(&self, x: i32, y: i32) -> Option<Point> {
        if x < 0 || y < 0 { return None };
        let (x_, y_) = (x as usize, y as usize);
        if x_ >= self.n_columns || y_ >= self.n_rows { return None };
        return self.points.get(y_).unwrap().get(x_).cloned()
    }

    fn neighbour_points(&self, target: &Point) -> HashSet<Point> {
        let x_ = target.x as i32;
        let y_ = target.y as i32;
        let indexes_to_find = [
            (x_ + 1, y_),
            (x_ - 1, y_),
            (x_, y_ + 1),
            (x_, y_ - 1)
        ];

        let mut neighbour_points: HashSet<Point> = HashSet::new();

        for pair in indexes_to_find {
            let maybe_point = HeightMap::get_point(self, pair.0, pair.1);
            match maybe_point {
                Some(p) => { neighbour_points.insert(p); },
                None => {}
            }
        }

        return neighbour_points
    }

    fn points_iter(&self) -> impl Iterator<Item=Point> + '_ {
        let row_ids = 0..(self.n_rows);
        let col_ids = 0..(self.n_columns);
        return row_ids.flat_map(move |y| col_ids.clone().map(move |x| (x, y)))
            .map(move |(x, y)| self.points.get(y).unwrap().get(x).unwrap())
            .map(|p| p.clone())
    }
}


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Point {
    x: u32,
    y: u32,
    height: u32,
    is_low: Option<bool>
}


fn find_low_points(height_map: &HeightMap) -> Vec<Point> {
    let mut points = Vec::new();
    for point in height_map.points_iter() {
        let height = point.height;
        let neighbour_points = height_map.neighbour_points(&point);
        let neighbour_points_heights_min = neighbour_points.iter()
            .map(|p| p.height).min().unwrap();

        if height < neighbour_points_heights_min { points.push(point.clone()) }
    }
    return points
}

fn compute_risk(low_points: &Vec<Point>) -> u32 {
    return low_points.iter().map(|p| p.height + 1).sum()
}

fn compute_risk_level(input: &Vec<String>) -> u32 {
    let map = HeightMap::new(input);
    let low_points = find_low_points(&map);
    return compute_risk(&low_points)
}

fn find_basin_via_low_point<'a>(
    map: &HeightMap,
    low_point: &Point,
    basin_points_found_so_far: &'a mut HashSet<Point>,
    recursion_depth: usize
) -> &'a mut HashSet<Point> {

    let recursion_depth_max = 10_000usize;
    if recursion_depth > 10_000 { panic!("Recursion depth max {} is reached", recursion_depth_max) }

    basin_points_found_so_far.insert(low_point.clone());

    let neighbours = map.neighbour_points(low_point);
    if neighbours.is_empty() { return basin_points_found_so_far }

    for neighbour in neighbours {
        if neighbour.height == 9 { continue }
        if neighbour.height > low_point.height {
            find_basin_via_low_point(
                map,
                &neighbour,
                basin_points_found_so_far,
                recursion_depth + 1
            );
        }
    }

    return basin_points_found_so_far
}

fn find_basins_via_low_points(map: &HeightMap, low_points: &Vec<Point>) -> Vec<HashSet<Point>> {
    let mut basins = Vec::new();
    for low_point in low_points {
        let basin = &mut HashSet::<Point>::new();
        find_basin_via_low_point(map, low_point, basin, 0);
        basins.push(basin.clone())
    }
    return basins
}

fn find_basins(input: &Vec<String>) -> u32 {
    let map = HeightMap::new(input);
    let low_points = find_low_points(&map);
    let basins = find_basins_via_low_points(&map, &low_points);
    return compute_basins_result(&basins)
}

fn compute_basins_result(basins: &Vec<HashSet<Point>>) -> u32 {
    let mut basins_to_sort = basins.clone();
    basins_to_sort.sort_by_key(|b| - (b.len() as i32));
    let mut v = 1u32;
    for (i, b) in basins_to_sort.iter().enumerate() {
        if i >= 3 { break }
        v = v * b.len() as u32;
    }
    return v
}

pub fn day_9() {
    let filename= "data/day-9-smoke-basin/data-part-1.txt";
    let lines = read_lines_as_str_vector(filename);
    let result = compute_risk_level(&lines);
    println!("Day 9 Part 1 result: {res}", res=result);
    let result_basins = find_basins(&lines);
    println!("Day 9 Part 2 result: {res}", res=result_basins);
}

#[cfg(test)]
mod tests {
    use crate::exs::utils::strs_to_strings;
    use super::*;

    #[test]
    fn test_map() {
        let height_map = HeightMap::new(
            &strs_to_strings(&&vec![
                "012",
                "345",
                "678",
                "901"
            ])
        );

        assert_eq!(height_map.get_point(0, 0), Some(Point { x: 0, y: 0, height: 0, is_low: None }));
        assert_eq!(height_map.get_point(0, 1), Some(Point { x: 0, y: 1, height: 3, is_low: None }));
        assert_eq!(height_map.get_point(2, 3), Some(Point { x: 2, y: 3, height: 1, is_low: None }));
        assert_eq!(height_map.get_point(3, 3), None);
        assert_eq!(height_map.get_point(-1, 3), None);

        assert_eq!(height_map.points_iter().map(|p| p.height).sum::<u32>(), 46);
    }

    #[test]
    fn test_get_neighbour_points() {
        let height_map = HeightMap::new(
            &strs_to_strings(&vec![
                "012",
                "345",
                "678",
                "901"
            ])
        );

        assert_eq!(
            height_map.neighbour_points(&Point { x: 0, y: 0, height: 0, is_low: None }),
            [
                Point { x: 0, y: 1, height: 3, is_low: None },
                Point { x: 1, y: 0, height: 1, is_low: None }
            ].iter().cloned().collect()
        );

        assert_eq!(
            height_map.neighbour_points(&Point { x: 1, y: 1, height: 4, is_low: None }),
            [
                Point { x: 1, y: 0, height: 1, is_low: None },
                Point { x: 0, y: 1, height: 3, is_low: None },
                Point { x: 2, y: 1, height: 5, is_low: None },
                Point { x: 1, y: 2, height: 7, is_low: None }
            ].iter().cloned().collect()
        );
    }

    #[test]
    fn test_naive_simulate_example() {
        let input = strs_to_strings(&vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ]);

        assert_eq!(compute_risk_level(&input), 15);
    }

    #[test]
    fn test_finding_basins() {
        let input = strs_to_strings(&vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ]);

        assert_eq!(find_basins(&input), 1134);
    }
}
