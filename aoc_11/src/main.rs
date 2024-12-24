use util::res::Result;
use std::collections::HashMap;

fn blink_n(stones: &Vec<u64>, iterations: usize) -> usize {
    let mut stone_counts: HashMap<u64, usize> = stones.iter().fold(HashMap::new(), |mut acc, stone| {
        *acc.entry(*stone).or_insert(0) += 1;
        acc
    });

    for _ in 0..iterations {
        stone_counts = stone_counts.iter().fold(HashMap::new(), |mut acc, (stone, count)| {
            let blinked = match (stone, stone.to_string().as_str()) {
                (0, _) => vec![1],
                (_, s) if (s.len() & 1) == 0 => {
                    let (first, second) = s.split_at(s.len() >> 1);
                    vec![first.parse::<u64>().unwrap(), second.parse::<u64>().unwrap()]
                },
                _ => vec![stone * 2024]
            };

            for new_stone in blinked {
                *acc.entry(new_stone).or_insert(0) += count;
            }
            acc
        });
    }

    stone_counts.iter().map(|(_, count)| count).sum()
}

fn part1(stones: &Vec<u64>) {
    println!("After 25 blinks, there are {} stones", blink_n(stones, 25));
}

fn part2(stones: &Vec<u64>) {
    println!("After 75 blinks, there are {} stones", blink_n(stones, 75));
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let contents = util::file::read_to_string(file_path)?;
    let stones = contents.split_whitespace().map(|raw_num| {
        raw_num.parse::<u64>().map_err(|e| e.into())
    }).collect::<Result<Vec<u64>>>()?;

    part1(&stones);
    part2(&stones);
    
    Ok(())
}