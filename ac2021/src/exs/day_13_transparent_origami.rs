use std::collections::HashSet;
use super::utils::lines_into_chunks;
use super::utils::read_lines_as_str_vector;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Paper {
    dots: HashSet<(u32, u32)>
}

#[derive(Debug, Eq, PartialEq)]
enum AlongAxis {
    Y,
    X
}

impl Paper {
    fn new(input: &Vec<String>) -> Paper {
        let mut data = Vec::with_capacity(input.len());
        for line in input {
            let parts: Vec<u32> = line.split(",")
                .map(|x|x.parse::<u32>().unwrap()).collect();
            data.push(parts);
        }
        let dots: HashSet<(u32, u32)> = data.iter()
            .map(|parts|(*parts.first().unwrap(), *parts.last().unwrap()))
            .collect();

        return Paper { dots }
    }

    fn len(&self) -> usize {
        return self.dots.len()
    }

    fn fold_along(&self, cmd: &FoldCommand) -> Paper {
        let mut new_dots: HashSet<(u32, u32)> = HashSet::new();
        for dot in &self.dots {
            let new_dot = match cmd.along {
                AlongAxis::Y => {
                    if dot.1 > cmd.along_coord {
                        let d = dot.1 - cmd.along_coord;
                        (dot.0, dot.1 - d * 2)
                    } else {
                        dot.clone()
                    }
                },
                AlongAxis::X => {
                    if dot.0 > cmd.along_coord {
                        let d = dot.0 - cmd.along_coord;
                        (dot.0 - d * 2, dot.1)
                    } else {
                        dot.clone()
                    }
                }
            };
            new_dots.insert(new_dot);
        }
        return Paper { dots: new_dots }
    }

    fn repr(&self) -> Vec<String> {
        let mut to_sort: Vec<(u32, u32)> = self.dots.iter()
            .map(|r| r.clone()).collect();
        to_sort.sort();
        let max_x = to_sort.last().unwrap().0;
        let max_y = to_sort.last().unwrap().1;
        return (0..=max_y).map(|y|
            (0..=max_x).map(|x| (x, y))
                .map(
                    |pair| {
                        if self.dots.contains(&pair) { "#" }
                        else { "." }
                    }
                ).collect::<Vec<&str>>().join("")
            ).collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct FoldCommand {
    along: AlongAxis,
    along_coord: u32
}

impl FoldCommand {
    fn new(encoded_cmd: &String) -> FoldCommand {
        let decoded_cmd = FoldCommand::_decode_fold_command(encoded_cmd);
        return FoldCommand { along: decoded_cmd.0, along_coord: decoded_cmd.1 }
    }

    fn _decode_fold_command(fold_command: &String) -> (AlongAxis, u32) {
        return (
            if fold_command.contains("y=") { AlongAxis::Y }
            else if fold_command.contains("x=") { AlongAxis::X }
            else { panic!("Unknown command {}", fold_command) },
            fold_command.clone()[("fold along y=".len())..].parse::<u32>().unwrap()
        )
    }

    fn decode_multiple(encoded_commands: &Vec<String>) -> Vec<FoldCommand> {
        return encoded_commands.iter()
            .map(|cmd| FoldCommand::new(cmd))
            .collect()
    }
}

fn decode_chunks(lines: &Vec<String>) -> (Paper, Vec<FoldCommand>) {
    let (dots_data, commands_data) = lines_into_chunks(lines);
    return (
        Paper::new(&dots_data),
        FoldCommand::decode_multiple(&commands_data)
    )
}

fn execute_commands(paper: &Paper, commands: &Vec<FoldCommand>) -> Paper {
    let mut new_paper = paper.clone();
    for cmd in commands {
        new_paper = new_paper.fold_along(cmd)
    }
    return new_paper
}

pub fn day_13() {
    let filename= "data/day-13-transparent-origami/data-part-1.txt";
    let lines = read_lines_as_str_vector(filename);
    let (paper, commands) = decode_chunks(&lines);
    let first_command = commands.first().unwrap();
    let paper_after_first_command = paper.fold_along(first_command);
    println!("Day 13 Part 1 result: {res}", res=paper_after_first_command.len());
    let final_paper = execute_commands(&paper, &commands);
    println!("Day 13 Part 2 result:");
    for line in final_paper.repr() {
        println!("{}", line)
    }
}

#[cfg(test)]
mod tests {
    use crate::exs::utils::strs_to_strings;
    use super::*;

    #[test]
    fn test_fold_along() {
        let inputs = strs_to_strings(&vec![
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0"
        ]);
        let paper = Paper::new(&inputs);
        assert_eq!(paper.len(), inputs.len());
        let paper_folded_by_y = paper.fold_along(
            &FoldCommand::new(&"fold along y=7".to_string()));
        assert_eq!(paper_folded_by_y.len(), 17);

        let paper_folded_by_x = paper_folded_by_y.fold_along(
            &FoldCommand::new(&"fold along x=5".to_string()));
        assert_eq!(paper_folded_by_x.len(), 16);
        assert_eq!(paper_folded_by_x.repr(), strs_to_strings(&vec![
            "#####",
            "#...#",
            "#...#",
            "#...#",
            "#####"
        ]))
    }

    #[test]
    fn test_decode_command() {
        assert_eq!(
            FoldCommand::new(&"fold along y=7".to_string()),
            FoldCommand { along: AlongAxis::Y, along_coord: 7}
        );
        assert_eq!(
            FoldCommand::new(&"fold along x=187129".to_string()),
            FoldCommand { along: AlongAxis::X, along_coord: 187129}
        )
    }

    #[test]
    fn test_decode_chunks() {
        assert_eq!(
            decode_chunks(
                &strs_to_strings(&vec![
                    "6,10",
                    "0,14",
                    "9,10",
                    "",
                    "fold along y=7",
                    "fold along x=5"
                ])
            ),
            (
                Paper {
                    dots: vec![(6, 10), (0, 14), (9, 10)].iter()
                        .map(|pair| pair.clone())
                        .collect::<HashSet<(u32, u32)>>()
                },
                vec![
                    FoldCommand { along: AlongAxis::Y, along_coord: 7 },
                    FoldCommand { along: AlongAxis::X, along_coord: 5 }
                ]
            )
        )
    }
}
