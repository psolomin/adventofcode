use std::collections::HashSet;
use super::matrix::Matrix;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
    risk_level: u32
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        return Point { x, y, risk_level: 0 }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Path {
    steps: Vec<Point>,
    total_risk: u32
}

impl Path {
    fn new(start: Point) -> Path {
        return Path { steps: vec![start], total_risk: 0 }
    }

    fn from_vec(steps: &Vec<Point>) -> Path {
        return Path { steps: steps.clone(), total_risk: 0 }
    }

    fn first(&self) -> &Point {
        return self.steps.first().unwrap()
    }

    fn last(&self) -> &Point {
        return self.steps.last().unwrap()
    }

    fn repr(&self) -> String {
        return self.steps.iter().map(|p| format!("({},{})", p.x, p.y))
            .collect::<Vec<_>>().join("->")
    }
}

struct Cavern {
    risk_levels: Matrix<u32>,
}

impl Cavern {
    const MAX_ITER: usize = 5;

    fn new(rows: &Vec<String>) -> Cavern {
        let risk_levels = Matrix::from_lines(rows);
        return Cavern { risk_levels }
    }

    fn find_lowest_risk_path_brute_force(&self) -> Path {
        let path = Path::new(Point::new(0, 0));
        return path
    }

    fn simulate_paths_brute_force(&self) -> Vec<Path> {
        let start_x = 0;
        let start_y = 0;
        let start_path = Path::new(Point::new(start_x, start_y));
        let mut paths = vec![start_path];
        Cavern::_simulate_paths_brute_force(
            self, &mut paths, start_x, start_y, 0);
        return paths
    }

    fn _simulate_paths_brute_force(
        &self,
        paths: &mut Vec<Path>,
        target_x: usize,
        target_y: usize,
        iter_id: usize
    ) {
        if iter_id > Cavern::MAX_ITER {
            panic!("Limit reached. Iter ID={}", iter_id)
        }

        let ns = Cavern::_find_neighbours(
            self, target_x, target_y);

        if ns.len() == 0 { return }

        let mut paths_new = Vec::new();
        for p in &paths.clone() {
            let last = p.last();
            let neighbours = Cavern::_find_neighbours(
                self, last.x, last.y);

            for neighbour in &neighbours {
                let mut new_p_points = p.clone().steps;
                new_p_points.push(neighbour.clone());
                paths_new.push(Path::from_vec(&new_p_points));
                Cavern::_simulate_paths_brute_force(
                    self,
                    &mut paths_new,
                    neighbour.x.clone(),
                    neighbour.y.clone(),
                    iter_id + 1
                );
            }
        }
        *paths = paths_new;
    }

    fn _find_neighbours(&self, target_x: usize, target_y: usize) -> HashSet<Point> {
        let x_ = target_x as i32;
        let y_ = target_y as i32;
        let indexes_to_find = [
            (x_ + 1, y_),
            (x_, y_ + 1)
        ];

        let mut neighbours = HashSet::new();
        for pair in indexes_to_find {
            if self.risk_levels.index_exists(pair.0, pair.1) {
                neighbours.insert(Point::new(pair.0 as usize, pair.1 as usize));
            }
        }

        return neighbours
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exs::utils::strs_to_strings;

    #[test]
    fn test_cavern() {
        let input = strs_to_strings(&vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581"
        ]);
        let cavern = Cavern::new(&input);
        assert_eq!(cavern.risk_levels.n_rows(), 10);
        assert_eq!(cavern.risk_levels.n_columns(), 10);
    }

    #[test]
    fn test_simulate_paths_brute_force() {
        let input = strs_to_strings(&vec![
            "116",
            "138"
        ]);
        let cavern = Cavern::new(&input);
        let paths = cavern.simulate_paths_brute_force();
        for p in paths {
            println!("{}", p.repr())
        }
    }
}
