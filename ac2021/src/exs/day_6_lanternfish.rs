use std::collections::HashMap;
use super::utils::read_lines_as_str_vector;

#[derive(Clone, Copy, Debug)]
struct LanternFish {
    age: u32,
    alive: bool
}

impl LanternFish {
    fn new(age: u32) -> LanternFish { LanternFish { age, alive: true } }

    fn day_passed(&mut self) -> Option<LanternFish> {
        if self.age == 0 {
            self.age = 6;
            return Option::Some(LanternFish::new(8))
        } else {
            self.age -= 1;
            return Option::None
        }
    }
}

fn run_naive_simulation(fish_ages_initial: &Vec<u32>, till_day: u32) -> u64 {
    let fish_population: Vec<LanternFish> = fish_ages_initial.iter()
        .map(|age| LanternFish::new(*age)).collect();

    let mut fish_population_previous = fish_population.clone();

    for _ in 0..till_day {
        let expected_new_population_cnt = fish_population_previous.len() * 2;
        let mut fish_population_new: Vec<LanternFish> = Vec::with_capacity(expected_new_population_cnt);

        for fish in fish_population_previous {
            let mut fish_to_update = fish.clone();
            let maybe_newborn_fish = fish_to_update.day_passed();
            match maybe_newborn_fish {
                Some(newborn_fish) => fish_population_new.push(newborn_fish),
                _ => {}
            }

            fish_population_new.push(fish_to_update);
        }

        fish_population_previous = fish_population_new;
    }

    return fish_population_previous.len() as u64
}

fn run_aggregates_simulation(fish_ages_initial: &Vec<u32>, till_day: u32) -> u64 {
    let fish_population_count_by_age: HashMap<u32, u64> = fish_ages_initial.iter()
        .fold(HashMap::new(), |mut acc, x| {
            let existing = acc.entry(x.clone()).or_insert(0);
            *existing += 1;
            return acc
        });

    let mut previous_fish_population_count_by_age = fish_population_count_by_age.clone();
    for _ in 0..till_day {
        let mut new_fish_population_count_by_age: HashMap<u32, u64> = HashMap::new();

        let maybe_zeros = previous_fish_population_count_by_age.get(&0);

        match maybe_zeros {
            Some(x) => {
                new_fish_population_count_by_age.insert(8, x.clone());
                new_fish_population_count_by_age.insert(6, x.clone());
            }
            None => { }
        }

        for age in 0..8 {
            let previous_age_cnt = previous_fish_population_count_by_age.get(&(age + 1));
            let e = new_fish_population_count_by_age.entry(age).or_insert(0);
            match previous_age_cnt {
                Some(x) => *e += x,
                None => {}
            };
        }

        previous_fish_population_count_by_age = new_fish_population_count_by_age;
    }

    return previous_fish_population_count_by_age.values().sum()
}

pub fn day_6() {
    let filename = "data/day-6-lanternfish/data-part-1.txt";
    let fish_initial_ages: Vec<u32> = read_lines_as_str_vector(filename)
        .first().unwrap().split(",").map(|d| d.parse::<u32>().unwrap()).collect();
    let result = run_naive_simulation(&fish_initial_ages, 80);
    println!("Day 6 Part 1 result: {res}", res=result);

    let result = run_aggregates_simulation(&fish_initial_ages, 256);
    println!("Day 6 Part 2 result: {res}", res=result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive_simulate_example() {
        let input = vec![3, 4, 3, 1, 2];
        assert_eq!(run_naive_simulation(&input, 5), 10);
        assert_eq!(run_naive_simulation(&input, 18), 26);
        assert_eq!(run_naive_simulation(&input, 80), 5934);
    }

    #[test]
    fn test_simulate_example() {
        let input = vec![3, 4, 3, 1, 2];
        assert_eq!(run_aggregates_simulation(&input, 5), 10);
        assert_eq!(run_aggregates_simulation(&input, 18), 26);
        assert_eq!(run_aggregates_simulation(&input, 80), 5934);
        assert_eq!(run_aggregates_simulation(&input, 256), 26984457539);
    }
}
