use anyhow::Context;
use itertools::Itertools;
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
    let mut free_pos = 0;
    loop {
        while free_pos < blocks.len() && blocks[free_pos] != FREE {
            free_pos += 1;
        }
        if free_pos == blocks.len() {
            break;
        }
        let mut last_block_pos = blocks.len() - 1;
        while blocks[last_block_pos] == FREE {
            last_block_pos -= 1;
        }
        blocks[free_pos] = blocks[last_block_pos];
        blocks.truncate(last_block_pos);
    }
    checksum(&blocks)
}

fn compact_whole_files(mut blocks: Vec<i32>) -> u64 {
    let chunks = blocks.iter().enumerate().chunk_by(|(_, b)| **b);
    let (mut free_spaces, files): (Vec<_>, Vec<_>) = chunks
        .into_iter()
        .map(|(_, chunk)| {
            let chunk = chunk.collect_vec();
            (chunk[0].0, chunk.len())
        })
        .partition(|&(pos, _)| blocks[pos] == FREE);

    for &(file_pos, file_size) in files.iter().rev() {
        let free_space = free_spaces
            .iter_mut()
            .take_while(|(pos, _)| *pos < file_pos)
            .find(|(_, size)| *size >= file_size);
        if let Some((free_pos, free_size)) = free_space {
            for i in 0..file_size {
                blocks.swap(*free_pos + i, file_pos + i);
            }
            *free_pos += file_size;
            *free_size -= file_size;
        };
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
