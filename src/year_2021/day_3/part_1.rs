use crate::year_2021::day_3::data;

pub fn main() {
    let split = data::INPUT.split("\n");
    let vec: Vec<&str> = split.collect();

    let mut digit_ones_counts = vec![0; vec[0].chars().count()];
    for line in vec {
        for (index, char) in line.chars().enumerate() {
            if char == '1' {
                digit_ones_counts[index] = digit_ones_counts[index] + 1
            }
        }
    }

    let gamma_rate = rounded_digit_value(digit_ones_counts.clone(), RoundDirection::Up);
    let epsilon_rate = rounded_digit_value(digit_ones_counts, RoundDirection::Down);

    println!("{:?}", &epsilon_rate * &gamma_rate);
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
