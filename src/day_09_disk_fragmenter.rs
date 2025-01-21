use anyhow::Context;
use std::iter::repeat_n;

pub fn run(input: &str) -> aoc::Result<String> {
    let mut blocks = vec![];
    for (ch, i) in input.trim().chars().zip(0..) {
        let size = ch.to_digit(10).context("Expected a digit")?;
        let block = if i % 2 == 0 { i / 2 } else { FREE };
        blocks.extend(repeat_n(block, size as usize));
    }

    let p1_checksum = compact_blocks(blocks.clone());
    let p2_checksum = compact_whole_files(blocks.clone());

    Ok(format!("{p1_checksum} {p2_checksum}"))
}

const FREE: i32 = -1;

fn compact_blocks(mut blocks: Vec<i32>) -> u64 {
    let mut free_idx = 0;
    loop {
        while free_idx < blocks.len() && blocks[free_idx] != FREE {
            free_idx += 1;
        }
        if free_idx == blocks.len() {
            break;
        }
        let mut last_block = blocks.pop().unwrap();
        while last_block == FREE {
            last_block = blocks.pop().unwrap()
        }
        blocks[free_idx] = last_block;
    }
    checksum(&blocks)
}

fn compact_whole_files(mut blocks: Vec<i32>) -> u64 {
    let mut file_id = *blocks.last().unwrap();
    while file_id > 0 {
        let file_idx = blocks.iter().position(|&b| b == file_id).unwrap();
        let file_size = blocks[file_idx..]
            .iter()
            .take_while(|&&b| b == file_id)
            .count();
        let free_chunk = blocks[0..file_idx]
            .chunk_by_mut(|a, b| a == b)
            .find(|chunk| chunk[0] == FREE && chunk.len() >= file_size);
        if let Some(free_chunk) = free_chunk {
            #[allow(clippy::needless_range_loop)]
            for i in 0..file_size {
                free_chunk[i] = file_id;
            }
            for i in 0..file_size {
                blocks[file_idx + i] = FREE;
            }
        }
        file_id -= 1;
    }
    checksum(&blocks)
}

fn checksum(blocks: &[i32]) -> u64 {
    (0..)
        .zip(blocks)
        .filter(|&(_, &b)| b != FREE)
        .map(|(i, &b)| i * b as u64)
        .sum()
}

#[test]
fn sample_test() {
    let sample = "2333133121414131402";
    assert_eq!(run(sample).unwrap(), "1928 2858")
}
