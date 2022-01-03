use std::convert::TryInto;
use super::utils::read_lines_as_str_vector;

struct Board<const BOARD_SIZE: usize> {
    digits: [(u32, u8); BOARD_SIZE],
    is_completed: bool,
    score: u32
}

impl <const BOARD_SIZE: usize> Board<BOARD_SIZE> {
    fn new(digits: &Vec<u32>) -> Board<BOARD_SIZE> {
        let digits_data: [(u32, u8); BOARD_SIZE] = digits.iter()
            .map(|e| (e.clone(), 0u8))
            .collect::<Vec<(u32, u8)>>().as_slice().try_into().unwrap();

        return Board::<BOARD_SIZE> {
            digits: digits_data,
            is_completed: false,
            score: 0
        }
    }

    fn apply_step(&mut self, step_digit: u32) -> (bool, u32) {
        if self.is_completed { return (true, self.score) }

        // TODO: make this compute compile-time
        let side_size = Board::<BOARD_SIZE>::_compute_side_length();

        for idx in 0..self.digits.len() {
            let elem = self.digits[idx];

            if elem.0 == step_digit {
                self.digits[idx].1 = 1;

                if Board::<BOARD_SIZE>::_check_horizontal(self, side_size, idx)
                    || Board::<BOARD_SIZE>::_check_vertical(self, side_size, idx) {
                    let score = Board::<BOARD_SIZE>::_calculate_score(self, step_digit);
                    self.score = score;
                    self.is_completed = true;
                    return (true, score)
                }
            }
        }
        return (false, 0)
    }

    fn _check_horizontal(&self, side_size: u32, idx: usize) -> bool {
        let h = Board::<BOARD_SIZE>::_get_horizontal_from_its_element_index(side_size, idx);
        let mut horizontal_sum: u32 = 0;

        for h_idx in h {
            let checked = self.digits.get(h_idx).unwrap().1 as u32;
            horizontal_sum += checked;
        }

        return horizontal_sum == side_size
    }

    fn _check_vertical(&self, side_size: u32, idx: usize) -> bool {
        let vertical_indexes = Board::<BOARD_SIZE>::_get_vertical_from_its_element_index(side_size, idx);
        let mut vertical_sum: u32 = 0;

        for v_idx in vertical_indexes {
            let checked = self.digits.get(v_idx).unwrap().1 as u32;
            vertical_sum += checked;
        }

        return vertical_sum == side_size
    }

    fn _calculate_score(&self, step_digit: u32) -> u32 {
        let sum_of_unchecked: u32 = self.digits.iter()
            .filter(|d| d.1 == 0u8)
            .map(|d| d.0)
            .sum();
        return sum_of_unchecked * step_digit
    }

    fn _get_horizontal_from_its_element_index(side_size: u32, elem_idx: usize) -> Vec<usize> {
        let bs: u32 = BOARD_SIZE as u32;

        for i in (1u32..).take_while(|x| x * x <= bs) {
            let lower = ((i - 1) * side_size) as usize;
            let upper = (i * side_size) as usize;
            if elem_idx >= lower && elem_idx <= upper {
                return (lower..upper).collect()
            }
        }

        return Vec::new()
    }

    fn _compute_side_length() -> u32 {
        let bs: u32 = BOARD_SIZE as u32;
        let mut side_size = 0u32;
        for _ in (1u32..).take_while(|x| x * x <= bs) {
            side_size += 1;
        }
        return side_size
    }

    fn _get_vertical_from_its_element_index(side_size: u32, elem_idx: usize) -> Vec<usize> {
        let column_id = modulus!(elem_idx as u32, side_size) as usize;
        let start = column_id;
        let end = BOARD_SIZE - side_size as usize + column_id;
        let indexes = (start..end + 1)
            .step_by(side_size as usize)
            .collect();

        return indexes
    }
}

fn build_board<const BOARD_SIZE: usize>(board_data: Vec<u32>) -> Board<BOARD_SIZE> {
    return Board::<BOARD_SIZE>::new(&board_data)
}

fn split_into_chunks(lines: &Vec<String>) -> (String, Vec<Vec<String>>) {
    let mut segments: Vec<Vec<String>> = vec![Vec::new()];
    let mut segment_id: usize = 0;

    for line in lines {
        if line != "" {
            segments[segment_id].push(line.to_string());
        } else {
            segment_id += 1;
            segments.push(Vec::new());
        }
    }

    let moves = segments.get(0).unwrap().get(0).unwrap().clone();
    let boards_data = segments[1..].to_vec();
    return (moves, boards_data)
}

fn extract_steps_digits(input: &String) -> Vec<u32> {
    return input.split(",").map(|line| line.parse::<u32>().unwrap()).collect()
}

fn extract_board<const BOARD_SIZE: usize>(input: Vec<String>) -> Board<BOARD_SIZE> {
    let data = input.iter()
        .map(|row| row.clone())
        .map(|row| row.replace("  ", " "))
        .map(|row| row.trim().split(" ")
            .map(|row| row.to_string()).collect::<Vec<String>>())
            .map(|digits| digits.iter().map(|d| d.parse::<u32>().unwrap())
                .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>()
        .concat();

    return build_board(data)
}

fn build_inputs_from_lines<const BOARD_SIZE: usize>(lines: &Vec<String>)
    -> (Vec<Board<BOARD_SIZE>>, Vec<u32>) {
    let (steps, boards_chunks) = split_into_chunks(lines);
    let steps_digits = extract_steps_digits(&steps);
    let boards = boards_chunks.iter()
        .map(|chunk| extract_board(chunk.clone()))
        .collect();

    return (boards, steps_digits)
}

fn get_winning_board_score<const BOARD_SIZE: usize>(
    mut boards: Vec<Board<BOARD_SIZE>>,
    steps: Vec<u32>
) -> u32 {
    for step in steps {
        for board in &mut boards {
            let (board_won, board_score) =
                board.apply_step(step);

            if board_won { return board_score }
        }
    }
    return 0
}

fn get_score_of_last_winning_board<const BOARD_SIZE: usize>(
    mut boards: Vec<Board<BOARD_SIZE>>,
    steps: Vec<u32>
) -> u32 {
    let mut discarded_boards: Vec<usize> = Vec::new();

    for step in steps {
        if boards.len() == 1 { return boards.get(0).unwrap().score }
        for (id, board) in boards.iter_mut().enumerate() {
            if discarded_boards.contains(&id) { continue }
            let (board_won, _) =
                board.apply_step(step);
            if board_won { discarded_boards.push(id) }
        }
    }
    let last_won = discarded_boards.last().unwrap().clone();
    return boards.get(last_won).unwrap().score
}

pub fn day_4() {
    const BOARD_SIZE: usize = 25usize;
    let filename = "data/day-4-giant-squid/data-part-1.txt";
    let lines = read_lines_as_str_vector(filename);
    let (boards, steps) = build_inputs_from_lines::<BOARD_SIZE>(&lines);
    let winning_board_score = get_winning_board_score::<BOARD_SIZE>(boards, steps);
    println!("Day 4 Part 1 result: {res}", res=winning_board_score);

    let (boards, steps) = build_inputs_from_lines::<BOARD_SIZE>(&lines);
    let score_of_last_winning_board = get_score_of_last_winning_board::<BOARD_SIZE>(boards, steps);
    println!("Day 4 Part 2 result: {res}", res=score_of_last_winning_board);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_wins() {
        const BOARD_SIZE: usize = 9usize;
        let mut board = Board::<BOARD_SIZE>::new(&vec![
            0, 1, 2,
            3, 4, 5,
            6, 7, 8
        ]);
        assert_eq!((false, 0), board.apply_step(0));
        assert_eq!((false, 0), board.apply_step(1));
        assert_eq!((true, 66), board.apply_step(2));
    }

    #[test]
    fn test_board_wins_other_horizontal() {
        const BOARD_SIZE: usize = 9usize;
        let mut board = Board::<BOARD_SIZE>::new(&vec![
            0, 1, 2,
            3, 4, 5,
            6, 7, 8
        ]);

        assert_eq!((false, 0), board.apply_step(6));
        assert_eq!((false, 0), board.apply_step(7));
        assert_eq!((true, 120), board.apply_step(8));
    }

    #[test]
    fn test_board_wins_vertical() {
        const BOARD_SIZE: usize = 9usize;
        let mut board = Board::<BOARD_SIZE>::new(&vec![
            22, 11, 40,
            35, 56, 90,
            111, 12, 1
        ]);

        assert_eq!((false, 0), board.apply_step(22));
        assert_eq!((false, 0), board.apply_step(11));
        assert_eq!((false, 0), board.apply_step(56));
        assert_eq!((false, 0), board.apply_step(1));
        assert_eq!((true, 3312), board.apply_step(12));

        // applying more steps should not change board's state
        assert_eq!((true, 3312), board.apply_step(12));
        assert_eq!((true, 3312), board.apply_step(8));
    }

    #[test]
    fn test_example() {
        const BOARD_SIZE: usize = 25usize;
        let filename = "data/day-4-giant-squid/test-data.txt";
        let lines = read_lines_as_str_vector(filename);
        let (boards, steps) = build_inputs_from_lines::<BOARD_SIZE>(&lines);
        let winning_board_score = get_winning_board_score::<BOARD_SIZE>(boards, steps);
        assert_eq!(winning_board_score, 4512)
    }

    #[test]
    fn test_example_last_winning_board() {
        const BOARD_SIZE: usize = 25usize;
        let filename = "data/day-4-giant-squid/test-data.txt";
        let lines = read_lines_as_str_vector(filename);
        let (boards, steps) = build_inputs_from_lines::<BOARD_SIZE>(&lines);
        let score_of_last_winning_board = get_score_of_last_winning_board::<BOARD_SIZE>(boards, steps);
        assert_eq!(score_of_last_winning_board, 1924)
    }
}
