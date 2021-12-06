use crate::year_2021::day_4::data;

pub fn main() {
    let split = data::INPUT.split("\n");
    let vec: Vec<&str> = split.collect();
    let mut majorities: Vec<&str> = vec.clone();
    let mut minorities: Vec<&str> = vec.clone();
    let mut index = 0;

    while majorities.len() > 1 {
        // println!("{:?}", majorities.len());
        majorities = meeting_criteria_at_index(index, majorities, Criteria::Majority);
        index = index + 1;
    }
    index = 0;
    // println!("{:?}", majorities.len());
    let oxygen = isize::from_str_radix(&*majorities[0], 2).unwrap();

    while minorities.len() > 1 {
        // println!("{:?}", minorities.len());
        minorities = meeting_criteria_at_index(index, minorities, Criteria::Minority);
        index = index + 1;
    }
    // println!("{:?}", minorities.len());
    let c_oh_two = isize::from_str_radix(&*minorities[0], 2).unwrap();
    // println!("{:?}", c_oh_two);

    println!("{:?}", c_oh_two * oxygen);
}

enum Criteria {
    Majority,
    Minority,
}

fn meeting_criteria_at_index(index: i32, list: Vec<&str>, criteria: Criteria) -> Vec<&str> {
    let list_size = list.len();
    let mut ones_count = 0;
    for line in list.iter() {
        if line.chars().nth(index as usize).unwrap() == '1' {
            ones_count = ones_count + 1
        }
    }
    let more_or_equal_ones = ones_count >= (list_size - ones_count);
    let keep_char = match criteria {
        Criteria::Majority => {
            if more_or_equal_ones {
                '1'
            } else {
                '0'
            }
        }
        Criteria::Minority => {
            if more_or_equal_ones {
                '0'
            } else {
                '1'
            }
        }
    };
    list.into_iter()
        .filter(|&string| string.chars().nth(index as usize).unwrap() == keep_char)
        .collect::<Vec<&str>>()
}
