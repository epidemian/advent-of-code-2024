use anyhow::{bail, Context};
use std::{env, fs, time};

mod day_01_historian_hysteria;
mod day_02_red_nosed_reports;

fn main() -> aoc::Result<()> {
    let args: Vec<_> = env::args().collect();
    let days = [
        day_01_historian_hysteria::run,
        day_02_red_nosed_reports::run,
    ];

    let run_single_day = |day_num: usize| -> aoc::Result<()> {
        let instant = time::Instant::now();
        let filename = format!("inputs/{day_num:02}.txt");
        let input =
            fs::read_to_string(&filename).with_context(|| format!("Error reading {filename}"))?;
        let output = days[day_num - 1](&input)?;
        let time_annotation = format_time_annotation(instant.elapsed());
        println!("Day {day_num}{time_annotation}: {output}");
        Ok(())
    };

    match args.len() {
        1 => {
            for day in 1..=days.len() {
                run_single_day(day)?;
            }
        }
        2 => {
            let day_num = args[1].parse::<usize>().context("Invalid day number")?;
            if day_num < 1 || day_num > days.len() {
                bail!("Day number out of range");
            }
            run_single_day(day_num)?;
        }
        _ => {
            bail!("Usage: {} [day_number]", args[0]);
        }
    }

    Ok(())
}

fn format_time_annotation(elapsed: time::Duration) -> String {
    if elapsed.as_millis() < 1 {
        "".to_string()
    } else {
        format!(" ({elapsed:.0?})")
    }
}
