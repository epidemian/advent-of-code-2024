use anyhow::{Context, bail, ensure};
use std::{env, fs, io::IsTerminal, time};

mod day_01_historian_hysteria;
mod day_02_red_nosed_reports;
mod day_03_mull_it_over;
mod day_04_ceres_search;
mod day_05_print_queue;
mod day_06_guard_gallivant;
mod day_07_bridge_repair;
mod day_08_resonant_collinearity;
mod day_09_disk_fragmenter;
mod day_10_hoof_it;
mod day_11_plutonian_pebbles;
mod day_12_garden_groups;
mod day_13_claw_contraption;
mod day_14_restroom_redoubt;
mod day_15_warehouse_woes;
mod day_16_reindeer_maze;
mod day_16_reindeer_maze_custom_dijkstra;
mod day_17_chronospatial_computer;
mod day_18_ram_run;
mod day_19_linen_layout;
mod day_20_race_condition;
mod day_21_keypad_conundrum;
mod day_22_monkey_market;
mod day_23_lan_party;
mod day_24_crossed_wires;
mod day_25_code_chronicle;

fn main() -> aoc::Result<()> {
    let days = [
        day_01_historian_hysteria::run,
        day_02_red_nosed_reports::run,
        day_03_mull_it_over::run,
        day_04_ceres_search::run,
        day_05_print_queue::run,
        day_06_guard_gallivant::run,
        day_07_bridge_repair::run,
        day_08_resonant_collinearity::run,
        day_09_disk_fragmenter::run,
        day_10_hoof_it::run,
        day_11_plutonian_pebbles::run,
        day_12_garden_groups::run,
        day_13_claw_contraption::run,
        day_14_restroom_redoubt::run,
        day_15_warehouse_woes::run,
        day_16_reindeer_maze::run,
        day_17_chronospatial_computer::run,
        day_18_ram_run::run,
        day_19_linen_layout::run,
        day_20_race_condition::run,
        day_21_keypad_conundrum::run,
        day_22_monkey_market::run,
        day_23_lan_party::run,
        day_24_crossed_wires::run,
        day_25_code_chronicle::run,
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

    let args: Vec<_> = env::args().collect();
    match args.len() {
        1 => {
            for n in 1..=days.len() {
                run_single_day(n)?;
            }
        }
        2 => {
            let n = args[1].parse().context("Invalid day number")?;
            ensure!(1 <= n && n <= days.len(), "Day number out of range");
            run_single_day(n)?;
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
