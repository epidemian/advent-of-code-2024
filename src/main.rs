use anyhow::{bail, Context};
use std::{env, fs, io::IsTerminal, time};

mod day_01_historian_hysteria;
mod day_02_red_nosed_reports;
mod day_03_mull_it_over;
mod day_04_ceres_search;
mod day_05_print_queue;
mod day_06_guard_gallivant;
mod day_07_bridge_repair;

fn main() -> aoc::Result<()> {
    let args: Vec<_> = env::args().collect();
    let days = [
        day_01_historian_hysteria::run,
        day_02_red_nosed_reports::run,
        day_03_mull_it_over::run,
        day_04_ceres_search::run,
        day_05_print_queue::run,
        day_06_guard_gallivant::run,
        day_07_bridge_repair::run,
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
    // Don't output duration if it's insignificant or we're not on a TTY (e.g. stdout is piped).
    if elapsed.as_millis() < 1 || !std::io::stdout().is_terminal() {
        "".to_string()
    } else {
        format!(" ({elapsed:.0?})")
    }
}
