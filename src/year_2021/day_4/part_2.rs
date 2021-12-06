use crate::year_2021::day_4::data;

#[derive(Debug)]
#[derive(Clone)]
struct Board {
    pub drawn: Vec<Vec<bool>>,
    pub numbers: Vec<Vec<i32>>,
    pub id: usize,
}

#[derive(Debug)]
struct Winner {
    pub winning_line: Vec<i32>,
    pub unmarked_sum: i32,
    pub winning_number: i32,
    pub score: i32,
    pub board: Board,
}

impl<'a> Board {
    pub fn draw(&mut self, value: i32) -> Option<Vec<i32>> {
        for (row_index, line) in (&self).numbers.clone().iter().enumerate() {
            for (col_index, &number) in line.iter().enumerate() {
                if number == value {
                    self.drawn[row_index][col_index] = true;
                    if !self.drawn[row_index].contains(&false) {
                        return Some(line.clone());
                    }
                    if self.drawn.iter().all(|row|
                        row[col_index]
                    ) {
                        return Some(line.clone());
                    }
                }
            }
        }
        return None;
    }

    pub fn unmarked_sum(&mut self) -> i32 {
        let mut sum = 0;
        for (row_index, line) in (&self).numbers.clone().iter().enumerate() {
            for (col_index, &number) in line.iter().enumerate() {
                if !self.drawn[row_index][col_index] {
                    sum = sum + number;
                }
            }
        }
        return sum;
    }
}

pub fn main() {
    let split = data::INPUT.split("\n");
    let mut lines: Vec<&str> = split.collect();
    let numbers: Vec<i32> = lines.remove(0).split(",").map(|num_str| num_str.parse::<i32>().unwrap()).collect();
    let board_lines = lines.chunks(6);
    let collected: Vec<&[&str]> = board_lines.collect();
    let mut boards: Vec<Board> = collected
        .iter()
        .enumerate()
        .map(|(index, &board_lines)| {
            let mut board_line_vec = board_lines.to_vec();
            board_line_vec.remove(0);
            let vec_board: Vec<Vec<i32>> = board_line_vec
                .iter()
                .map(|&board_line| {
                    let result: Vec<i32> = board_line.split(" ").filter(|&num| num != "").map(|num_str| num_str.parse::<i32>().unwrap()).collect();
                    result
                })
                .collect();
            Board {
                drawn: vec![
                    vec![false; 5],
                    vec![false; 5],
                    vec![false; 5],
                    vec![false; 5],
                    vec![false; 5],
                ],
                numbers: vec_board,
                id: index,
            }
        })
        .collect();
    let mut winners: Vec<Winner> = Vec::new();
    let mut won_state: Vec<bool> = boards.iter().map(|board| {false}).collect();
    for number in numbers {
        let mut round_winners: Vec<usize> = Vec::new();
        for (index, board) in &mut (boards.iter_mut().enumerate()) {
            let winning_line = board.draw(number);
            match winning_line {
                None => {}
                Some(winning_line) => {
                    let unmarked_sum = board.unmarked_sum();
                    winners.push(Winner {
                        winning_line,
                        unmarked_sum,
                        winning_number: number,
                        score: unmarked_sum * number,
                        board: board.clone(),
                    });
                    round_winners.push(index);
                    if !won_state[index] {
                        println!("{:?}", unmarked_sum * number);
                    }
                    won_state[index] = true;
                }
            }
        }
    }
}

enum RoundDirection {
    Up,
    Down,
}

fn rounded_digit_value(counts: Vec<i32>, round: RoundDirection) -> isize {
    let rounded_counts: Vec<&str> = counts
        .iter()
        .map(|&ones_count| match round {
            RoundDirection::Up => {
                if ones_count > 500 {
                    "1"
                } else {
                    "0"
                }
            }
            RoundDirection::Down => {
                if ones_count < 500 {
                    "1"
                } else {
                    "0"
                }
            }
        })
        .collect();
    let binary_string = rounded_counts.join("");
    isize::from_str_radix(&*binary_string, 2).unwrap()
}
