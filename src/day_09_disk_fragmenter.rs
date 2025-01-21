use std::iter;

use anyhow::ensure;
use itertools::Itertools;

pub fn run(input: &str) -> aoc::Result<String> {
    let input = input.trim();
    ensure!(input.len() % 2 == 1, "Disk map length should be odd");

    let chars = input.bytes().chain([b'0']); // Add 0 free space at the end to simplify parsing.
    let mut blocks = vec![];
    for (file_id, (file_size, free_size)) in chars.tuples().enumerate() {
        let file_size = (file_size - b'0') as usize;
        let free_size = (free_size - b'0') as usize;
        blocks.extend(iter::repeat_n(Some(file_id), file_size));
        blocks.extend(iter::repeat_n(None, free_size));
    }

    let p1_checksum = compact_blocks(blocks.clone());
    let p2_checksum = compact_whole_files(blocks.clone());

    Ok(format!("{p1_checksum} {p2_checksum}"))
}

fn compact_blocks(mut blocks: Vec<Option<usize>>) -> usize {
    let mut free_idx = 0;
    loop {
        while free_idx < blocks.len() && blocks[free_idx].is_some() {
            free_idx += 1;
        }
        if free_idx == blocks.len() {
            break;
        }
        let mut last_block = blocks.pop().unwrap();
        while last_block.is_none() {
            last_block = blocks.pop().unwrap()
        }
        blocks[free_idx] = last_block;
    }
    checksum(&blocks)
}

fn compact_whole_files(mut blocks: Vec<Option<usize>>) -> usize {
    let mut file_id = blocks.last().unwrap().unwrap();
    while file_id > 0 {
        let file_idx = blocks.iter().position(|&b| b == Some(file_id)).unwrap();
        let file_size = blocks[file_idx..]
            .iter()
            .take_while(|&&b| b == Some(file_id))
            .count();
        let free_chunk = blocks[0..file_idx]
            .chunk_by_mut(|a, b| a == b)
            .find(|chunk| chunk[0].is_none() && chunk.len() >= file_size);
        if let Some(free_chunk) = free_chunk {
            #[allow(clippy::needless_range_loop)]
            for i in 0..file_size {
                free_chunk[i] = Some(file_id);
            }
            for i in 0..file_size {
                blocks[file_idx + i] = None;
            }
        }
        file_id -= 1;
    }
    checksum(&blocks)
}

fn checksum(blocks: &[Option<usize>]) -> usize {
    blocks
        .iter()
        .enumerate()
        .map(|(i, b)| i * b.unwrap_or(0))
        .sum()
}

#[test]
fn sample_test() {
    let sample = "2333133121414131402";
    assert_eq!(run(sample).unwrap(), "1928 2858")
}
