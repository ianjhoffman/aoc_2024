use util::res::Result;
use std::collections::HashMap;

fn parse_input() -> Result<(Vec<u32>, Vec<u32>)> {
    let file_path = util::file::get_input_file_path();
    let raw_lines = util::file::read_lines_raw(file_path)?;

    let mut list1: Vec<u32> = Vec::with_capacity(raw_lines.len());
    let mut list2: Vec<u32> = Vec::with_capacity(raw_lines.len());
    for raw_line in raw_lines.iter() {
        let parsed_nums: Vec<u32> = raw_line.split_whitespace().map(|raw_num| raw_num.parse::<u32>())
            .collect::<std::result::Result<Vec<u32>, std::num::ParseIntError>>()?;

        match parsed_nums.as_slice() {
            [num1, num2] => {
                list1.push(*num1);
                list2.push(*num2);

                println!("{:?} {:?}", *num1, *num2);
            }
            _ => return Err(From::from(
                format!("Failed to parse line: {:?}", raw_line).to_owned()
            ))
        }
    }

    list1.sort();
    list2.sort();

    Ok((list1, list2))
}

fn part1(list1: &Vec<u32>, list2: &Vec<u32>) {
    let total_distance: u32 = list1.iter().zip(list2.iter())
        .map(|(e1, e2)| e1.abs_diff(*e2))
        .sum();

    println!("Total distance between lists: {:?}", total_distance);
}

fn part2(list1: &Vec<u32>, list2: &Vec<u32>) {
    let list2_counts: HashMap<u32, u32> = list2.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(*num).or_insert(0) += 1;
        acc
    });

    let similarity_score: u32 = list1.iter().map(|num| num * list2_counts.get(num).or(Some(&0)).unwrap()).sum();

    println!("Similarity score between lists: {:?}", similarity_score);
}

fn main() -> Result<()> {
    let (list1, list2) = parse_input()?;

    part1(&list1, &list2);
    part2(&list1, &list2);

    Ok(())
}