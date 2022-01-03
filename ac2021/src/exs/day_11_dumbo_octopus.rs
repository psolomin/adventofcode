use crate::exs::matrix::Matrix;
use super::utils::read_lines_as_str_vector;

#[derive(Debug, Eq, PartialEq)]
struct Cavern {
    energies: Matrix<u32>,
    n_rows: usize,
    n_columns: usize,
    flashes: u64
}

impl Cavern {
    const MAX_ENERGY: u32 = 9;
    const MAX_ITER: u32 = 100;

    fn new(rows: &Vec<String>) -> Cavern {
        let m = Matrix::<u32>::from_lines(rows);
        return Cavern {
            energies: m.clone(),
            n_rows: m.n_rows(),
            n_columns: m.n_columns(),
            flashes: 0
        }
    }

    fn neighbour_energies_indexes(&self, target_x: usize, target_y: usize) -> Vec<(usize, usize)> {
        let x_ = target_x as i32;
        let y_ = target_y as i32;
        let indexes_to_find = [
            (x_ + 1, y_),
            (x_ - 1, y_),
            (x_, y_ + 1),
            (x_, y_ - 1),

            (x_ + 1, y_ + 1),
            (x_ + 1, y_ - 1),
            (x_ - 1, y_ + 1),
            (x_ - 1, y_ - 1)
        ];

        let mut neighbours = Vec::new();
        for pair in indexes_to_find {
            if self.energies.index_exists(pair.0, pair.1) {
                neighbours.push((pair.0 as usize, pair.1 as usize))
            }
        }

        return neighbours
    }

    fn trigger(&mut self) -> u64 {
        for x in 0..(self.n_columns as i32) {
            for y in 0..(self.n_rows as i32) {
                let energy = self.energies.get_mut_point(x, y).unwrap();
                *energy += 1
            }
        }

        let mut flashes = 0;
        let mut iter_id: u32 = 0;
        while Cavern::has_max_energies(self) {
            iter_id += 1;

            if iter_id >= Cavern::MAX_ITER {
                panic!("Iteration count={}, max={}", iter_id, Cavern::MAX_ITER)
            }

            for y in 0..self.n_rows {
                for x in 0..self.n_columns {
                    let energy = self.energies.get_mut_point(x as i32, y as i32).unwrap();

                    if *energy > Cavern::MAX_ENERGY {
                        flashes += 1;
                        *energy = 0;
                        let neighbour_coordinates =
                            Cavern::neighbour_energies_indexes(self, x, y);
                        for (x_n, y_n) in neighbour_coordinates {
                            let neighbour_energy =
                                self.energies.get_mut_point(x_n as i32, y_n as i32).unwrap();
                            if *neighbour_energy > 0 {
                                *neighbour_energy += 1
                            }
                        }
                    }
                }
            }
        }

        return flashes
    }

    fn has_max_energies(&self) -> bool {
        for energy in self.energies.points_iter() {
            if energy > Cavern::MAX_ENERGY { return true }
        }
        return false
    }
}

fn create_cavern_and_run_steps(rows: &Vec<String>, n_steps: u32) -> u64 {
    let mut cavern = Cavern::new(rows);
    let mut total_flashes = 0;
    for _ in 0..n_steps { total_flashes += cavern.trigger() }
    return total_flashes
}

fn create_cavern_and_find_first_sync(rows: &Vec<String>, max_steps: u32) -> u32 {
    let mut cavern = Cavern::new(rows);
    for step_id in 0..=max_steps {
        if step_id == max_steps {
            panic!("Iteration count={}, max={}, sync not found", step_id, max_steps)
        }
        let flashes_cnt = cavern.trigger();
        if flashes_cnt as usize == (cavern.n_rows * cavern.n_columns)  {
            return step_id + 1
        }
    }
    return 0
}

pub fn day_11() {
    let filename= "data/day-11-dumbo-octopus/data-part-1.txt";
    let lines = read_lines_as_str_vector(filename);
    let result = create_cavern_and_run_steps(&lines, 100);
    println!("Day 11 Part 1 result: {res}", res=result);
    let result = create_cavern_and_find_first_sync(&lines, 1000);
    println!("Day 11 Part 2 result: {res}", res=result);
}

#[cfg(test)]
mod tests {
    use crate::exs::utils::strs_to_strings;
    use super::*;

    #[test]
    fn test_flashes_count() {
        let input_data = strs_to_strings(&vec![
            "123",
            "456"
        ]);
        let mut cavern = Cavern::new(&input_data);
        assert_eq!(cavern.has_max_energies(), false);
        assert_eq!(cavern.trigger(), 0);
        assert_eq!(cavern.trigger(), 0);
        assert_eq!(cavern.trigger(), 0);
        assert_eq!(cavern.trigger(), 2)
    }

    #[test]
    fn test_flashes_count_task_example() {
        let input_data = strs_to_strings(&vec![
            "11111",
            "19991",
            "19191",
            "19991",
            "11111"
        ]);
        let mut cavern = Cavern::new(&input_data);
        assert_eq!(cavern.trigger(), 9);
    }

    #[test]
    fn test_flashes_multiple_triggers() {
        let input_data = strs_to_strings(&vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526"
        ]);
        assert_eq!(create_cavern_and_run_steps(&input_data, 1), 0);
        assert_eq!(create_cavern_and_run_steps(&input_data, 2), 35);
        assert_eq!(create_cavern_and_run_steps(&input_data, 10), 204);
        assert_eq!(create_cavern_and_run_steps(&input_data, 100), 1656);

        assert_eq!(create_cavern_and_find_first_sync(&input_data, 200), 195)
    }
}
