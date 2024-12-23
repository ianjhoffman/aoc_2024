use util::res::Result;
use util::file::GenericParseError;
use std::iter;
use std::collections::{BTreeSet, HashMap};

// None = free space, Some(u32) = file with ID
type DiskBlock = Option<u32>;

struct Disk {
    blocks: Vec<DiskBlock>,
    file_pointers: Vec<(usize, usize)>, // (start, length)
    free_pointers: Vec<(usize, usize)>, // (start, length)
}

impl Disk {
    fn from_str(input: &String) -> Result<Disk> {
        let mut disk = Disk{blocks: vec![], file_pointers: vec![], free_pointers: vec![]};

        let mut is_file: bool = true;
        let mut file_id: u32 = 0;
        for c in input.chars() {
            let length = c.to_digit(10).ok_or_else(
                || GenericParseError::ValueError(format!("Invalid character: {}", c).to_owned())
            )? as usize;

            if is_file {
                disk.file_pointers.push((disk.blocks.len(), length));
                disk.blocks.extend(iter::repeat(DiskBlock::Some(file_id)).take(length));
            } else {
                if length > 0 { disk.free_pointers.push((disk.blocks.len(), length)); }
                disk.blocks.extend(iter::repeat(DiskBlock::None).take(length));

                // Increment file ID for next file blocks
                file_id += 1;
            }

            // Switch from file to not file, or vice versa
            is_file ^= true;
        }

        Ok(disk)
    }

    fn defragment_compact(&self) -> Vec<DiskBlock> {
        let mut out = self.blocks.clone();

        let mut write_idx = 0usize;
        let mut read_idx = self.blocks.len() - 1;
        while read_idx > write_idx {
            match (out[read_idx], out[write_idx]) {
                (None, None) => {
                    read_idx -= 1;
                },
                (None, Some(_)) => {
                    write_idx += 1;
                    read_idx -= 1;
                },
                (Some(file_id), None) => {
                    out[write_idx] = DiskBlock::Some(file_id);
                    out[read_idx] = DiskBlock::None;

                    write_idx += 1;
                    read_idx -= 1;
                },
                (Some(_), Some(_)) => {
                    write_idx += 1;
                },
            }
        }

        out
    }

    fn defragment_best_effort(&self) -> Vec<DiskBlock> {
        let mut free_blocks_by_size: HashMap<usize, BTreeSet<usize>> = self.free_pointers.iter().fold(HashMap::new(), |mut acc, &(start, length)| {
            acc.entry(length).or_insert(BTreeSet::new()).insert(start);
            acc
        });
        
        let mut out = self.blocks.clone();
        for &(file_start, file_length) in self.file_pointers.iter().rev() {
            let mut earliest_adequate_free_chunk: Option<(usize, usize)> = None; // (start, length)
            for free_length in file_length..=9 {
                match free_blocks_by_size.get(&free_length).map(|start_indices| start_indices.first()).flatten() {
                    Some(start_index) => {
                        if earliest_adequate_free_chunk.is_none() || *start_index < earliest_adequate_free_chunk.unwrap().0 {
                            earliest_adequate_free_chunk = Some((*start_index, free_length));
                        }
                    },
                    None => {},
                }
            }

            // If we found a free chunk that's further back than this file's start, move it
            match earliest_adequate_free_chunk {
                Some((free_start, free_length)) if free_start < file_start => {
                    // Remove this free block from the free blocks by size, any leftover size becomes a new free block
                    free_blocks_by_size.entry(free_length).and_modify(|e| { e.pop_first(); });
                    if file_length < free_length {
                        free_blocks_by_size.entry(free_length - file_length)
                            .or_insert(BTreeSet::new()).insert(free_start + file_length);
                    }

                    // Swap entries from file to free block
                    for i in 0..file_length {
                        out.swap(free_start + i, file_start + i);
                    }
                },
                _ => {}
            }
        }

        out
    }

    fn checksum(blocks: &Vec<DiskBlock>) -> u64 {
        blocks.iter().enumerate().map(|(idx, block)| {
            (block.unwrap_or(0) as u64) * (idx as u64)
        }).sum()
    }
}

fn part1(disk: &Disk) {
    let checksum = Disk::checksum(&disk.defragment_compact());

    println!("Checksum of defragmented disk (compaction): {}", checksum);
}

fn part2(disk: &Disk) {
    let checksum = Disk::checksum(&disk.defragment_best_effort());

    println!("Checksum of defragmented disk (best effort): {}", checksum);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let contents = util::file::read_to_string(file_path)?;
    let disk = Disk::from_str(&contents)?;

    part1(&disk);
    part2(&disk);

    Ok(())
}