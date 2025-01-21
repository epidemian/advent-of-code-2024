use anyhow::Context;
use std::iter::repeat_n;

pub fn run(input: &str) -> aoc::Result<String> {
    let mut blocks = vec![];
    for (i, ch) in input.trim().chars().enumerate() {
        let size = ch.to_digit(10).context("Expected a digit")?;
        let block = if i % 2 == 0 { Some(i / 2) } else { None };
        blocks.extend(repeat_n(block, size as usize));
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
