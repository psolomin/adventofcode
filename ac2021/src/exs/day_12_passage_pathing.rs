use std::collections::{HashMap, HashSet};

use super::utils::read_lines_as_str_vector;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Cave {
    id: String,
    is_start: bool,
    is_end: bool,
    is_large: bool,
}

impl Cave {
    fn new(id: &String) -> Cave {
        return Cave {
            id: id.clone(),
            is_start: id == "start",
            is_end: id == "end",
            is_large: &id.to_uppercase() == id
        }
    }

    fn repr(&self) -> &str {
        return &self.id
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Connection {
    start: Cave,
    end: Cave
}

impl Connection {
    fn new(start_id: &String, end_id: &String) -> Connection {
        return Connection {
            start: Cave::new(start_id),
            end: Cave::new(end_id)
        }
    }

    fn repr(&self) -> String {
        return format!("Connection({} -> {})", self.start.id, self.end.id)
    }
}

#[derive(Clone, Copy)]
enum PathRestrictions {
    V1,
    V2,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Path {
    caves: Vec<Cave>,
    starts_with: Cave,
    ends_with: Cave,
    caves_visits_count: HashMap<String, u32>,
    small_caves_with_more_than_two_visits: u32
}

impl Path {
    fn new(seed_cave: Cave) -> Path {
        return Path {
            caves: vec![seed_cave.clone()],
            starts_with: seed_cave.clone(),
            ends_with: seed_cave.clone(),
            caves_visits_count: Default::default(),
            small_caves_with_more_than_two_visits: 0
        }
    }

    fn ids(&self) -> String {
        return self.caves.iter()
            .map(|p|p.repr())
            .collect::<Vec<&str>>().join(",")
    }

    fn append(&mut self, cave: Cave, path_restrictions: PathRestrictions) -> bool {
        let cave_id = cave.id.clone();
        let cnt = self.caves_visits_count
            .entry(cave_id.clone()).or_insert(0);
        *cnt += 1;

        if *cnt > 1 && !cave.is_large { self.small_caves_with_more_than_two_visits += 1 }

        match path_restrictions {
            PathRestrictions::V1 => {
                if *cnt > 1 && !cave.is_large {
                    return false
                }
            },
            PathRestrictions::V2 => {
                if self.small_caves_with_more_than_two_visits > 1 && !cave.is_large {
                    return false
                }
            }
        }

        if *cnt > 1000 {
            return false
        }

        self.caves.push(cave.clone());
        self.ends_with = cave;
        return true
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Paths {
    paths: Vec<Path>
}

impl Paths {
    fn seed() -> Paths {
        let seed_cave = Cave::new(&"start".to_string());
        let seed_path = Path::new(seed_cave);
        return Paths { paths: vec![seed_path] }
    }

    fn new() -> Paths {
        return Paths { paths: Default::default() }
    }

    fn add(&mut self, path: Path) {
        self.paths.push(path);
    }

    fn repr(&self) -> Vec<String> {
        return self.paths.iter().map(|p| p.ids()).collect()
    }

    fn len(&self) -> usize {
        return self.paths.len()
    }
}

#[derive(Debug)]
struct CaveSystem {
    connections: HashMap<String, HashSet<Connection>>
}

impl CaveSystem {
    const MAX_ITER: u32 = 30;

    fn new() -> CaveSystem {
        return CaveSystem { connections: Default::default() }
    }

    fn add_connection(&mut self, conn: Connection) {
        let current = self.connections.entry(conn.start.id.clone())
            .or_insert(HashSet::new());
        current.insert(conn);
    }

    fn from_strings(pairs: &Vec<String>) -> CaveSystem {
        let mut cave_system = CaveSystem::new();
        let processed_pairs: Vec<Connection> = pairs.iter()
            .map(|encoded_pair| encoded_pair.split("-").collect::<Vec<&str>>())
            .map(|v|(v.clone().first().unwrap().clone(), v.last().unwrap().clone()))
            .map(|(from, to)|(Cave::new(&from.to_string()), Cave::new(&to.to_string())))
            .map(|(from, to)|
                if to.is_start { (to, from) }
                else { (from, to) })
            .map(|(from, to)|
                if from.is_end { (to, from) }
                else { (from, to) })
            .map(|(from, to)| {
                if !from.is_start && !to.is_end { vec![
                    Connection { start: from.clone(), end: to.clone() },
                    Connection { start: to, end: from }
                ]}
                else {
                    vec![
                        Connection { start: from, end: to },
                    ]
                }
            })
            .flatten()
            .collect();

        for p in processed_pairs {
            cave_system.add_connection(p);
        }
        
        return cave_system
    }

    fn compute_all_paths(&self, path_restrictions: PathRestrictions) -> Paths {
        let mut paths = Paths::seed();
        let mut iter_id = 0;
        let mut some_path_does_not_have_end = true;
        while iter_id < CaveSystem::MAX_ITER && some_path_does_not_have_end {
            let mut new_paths = Paths::new();
            some_path_does_not_have_end = false;

            for path in &paths.paths {
                let c = self.connections.get(&path.ends_with.id);
                match c {
                    Some(conns) => {
                        for conn in conns {
                            let mut new_path = path.clone();
                            let is_accepted = new_path.append(
                                conn.end.clone(),
                                path_restrictions
                            );
                            if is_accepted {
                                new_paths.add(new_path);
                            }
                        }
                    }
                    _ => {
                        new_paths.add(path.clone());
                    }
                }

                if !path.ends_with.is_end {
                    some_path_does_not_have_end = true
                }
            }

            paths = new_paths;
            iter_id += 1;

            if !some_path_does_not_have_end {
                println!("Early stopped at iter={}", iter_id)
            }
        }
        return paths
    }
}

pub fn day_12() {
    let filename= "data/day-12-passage-pathing/data-part-1.txt";
    let lines = read_lines_as_str_vector(filename);
    let cave_system = CaveSystem::from_strings(&lines);
    let result = cave_system.compute_all_paths(PathRestrictions::V1);
    println!("Day 12 Part 1 result: {res}", res=result.len());

    let result = cave_system.compute_all_paths(PathRestrictions::V2);
    println!("Day 12 Part 2 result: {res}", res=result.len());
}

#[cfg(test)]
mod tests {
    use crate::exs::utils::strs_to_strings;
    use super::*;

    #[test]
    fn test_compute_all_paths() {
        let path_restrictions = PathRestrictions::V1;
        let mut cave_system = CaveSystem::new();

        cave_system.add_connection(
            Connection::new(&"start".to_string(), &"a".to_string())
        );
        cave_system.add_connection(
            Connection::new(&"a".to_string(), &"end".to_string())
        );
        assert_eq!(cave_system.compute_all_paths(path_restrictions).repr(), vec!["start,a,end"]);

        cave_system.add_connection(
            Connection::new(&"start".to_string(), &"A".to_string())
        );
        cave_system.add_connection(
            Connection::new(&"A".to_string(), &"a".to_string())
        );
        cave_system.add_connection(
            Connection::new(&"a".to_string(), &"A".to_string())
        );
        cave_system.add_connection(
            Connection::new(&"A".to_string(), &"end".to_string())
        );
        assert_eq!(
            cave_system.compute_all_paths(path_restrictions).repr().sort(),
            vec![
                "start,a,end",
                "start,A,end",
                "start,A,a,end",
                "start,a,A,end",
                "start,A,a,A,end"
            ].sort()
        );
    }

    #[test]
    fn load_pairs_and_compute_paths() {
        let path_restrictions = PathRestrictions::V1;
        let inputs = strs_to_strings(&vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end"
        ]);
        let cave_system = CaveSystem::from_strings(&inputs);
        assert_eq!(cave_system.compute_all_paths(path_restrictions).len(), 10)
    }

    #[test]
    fn load_pairs_and_compute_paths_bigger() {
        let path_restrictions = PathRestrictions::V1;
        let inputs = strs_to_strings(&vec![
            "dc-end",
            "HN-start",
            "start-kj",
            "dc-start",
            "dc-HN",
            "LN-dc",
            "HN-end",
            "kj-sa",
            "kj-HN",
            "kj-dc"
        ]);
        let cave_system = CaveSystem::from_strings(&inputs);
        assert_eq!(
            cave_system.compute_all_paths(path_restrictions).repr().sort(),
            vec![
                "start,HN,dc,HN,end",
                "start,HN,dc,HN,kj,HN,end",
                "start,HN,dc,end",
                "start,HN,dc,kj,HN,end",
                "start,HN,end",
                "start,HN,kj,HN,dc,HN,end",
                "start,HN,kj,HN,dc,end",
                "start,HN,kj,HN,end",
                "start,HN,kj,dc,HN,end",
                "start,HN,kj,dc,end",
                "start,dc,HN,end",
                "start,dc,HN,kj,HN,end",
                "start,dc,end",
                "start,dc,kj,HN,end",
                "start,kj,HN,dc,HN,end",
                "start,kj,HN,dc,end",
                "start,kj,HN,end",
                "start,kj,dc,HN,end",
                "start,kj,dc,end"
            ].sort()
        );
    }

    #[test]
    fn load_pairs_and_compute_paths_realistic_example() {
        let path_restrictions = PathRestrictions::V1;
        let inputs = strs_to_strings(&vec![
            "fs-end",
            "he-DX",
            "fs-he",
            "start-DX",
            "pj-DX",
            "end-zg",
            "zg-sl",
            "zg-pj",
            "pj-he",
            "RW-he",
            "fs-DX",
            "pj-RW",
            "zg-RW",
            "start-pj",
            "he-WI",
            "zg-he",
            "pj-fs",
            "start-RW"
        ]);
        let cave_system = CaveSystem::from_strings(&inputs);
        assert_eq!(cave_system.compute_all_paths(path_restrictions).len(), 226)
    }

    #[test]
    fn load_pairs_and_compute_paths_v2() {
        let path_restrictions = PathRestrictions::V2;
        let inputs = strs_to_strings(&vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end"
        ]);
        let cave_system = CaveSystem::from_strings(&inputs);
        assert_eq!(cave_system.compute_all_paths(path_restrictions).len(), 36)
    }

    #[test]
    fn load_pairs_and_compute_paths_bigger_v2() {
        let path_restrictions = PathRestrictions::V2;
        let inputs = strs_to_strings(&vec![
            "dc-end",
            "HN-start",
            "start-kj",
            "dc-start",
            "dc-HN",
            "LN-dc",
            "HN-end",
            "kj-sa",
            "kj-HN",
            "kj-dc"
        ]);
        let cave_system = CaveSystem::from_strings(&inputs);
        assert_eq!(cave_system.compute_all_paths(path_restrictions).len(), 103);
    }

    #[test]
    fn load_pairs_and_compute_paths_realistic_example_v2() {
        let path_restrictions = PathRestrictions::V2;
        let inputs = strs_to_strings(&vec![
            "fs-end",
            "he-DX",
            "fs-he",
            "start-DX",
            "pj-DX",
            "end-zg",
            "zg-sl",
            "zg-pj",
            "pj-he",
            "RW-he",
            "fs-DX",
            "pj-RW",
            "zg-RW",
            "start-pj",
            "he-WI",
            "zg-he",
            "pj-fs",
            "start-RW"
        ]);
        let cave_system = CaveSystem::from_strings(&inputs);
        assert_eq!(cave_system.compute_all_paths(path_restrictions).len(), 3509)
    }
}
