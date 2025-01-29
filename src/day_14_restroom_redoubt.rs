use itertools::Itertools;
use std::collections::HashSet;

pub fn run(input: &str) -> aoc::Answer {
    let robots = parse_robots(input)?;
    aoc::answer(
        get_safety_factor(&robots, 101, 103),
        // The easter egg position was found by running `print_steps(&robots)` and then grepping the
        // output for "xxxxxxxx".
        8270,
    )
}

type Point = (i64, i64);

fn parse_robots(input: &str) -> aoc::Result<Vec<(Point, Point)>> {
    let parse_robot = |s| {
        let numbers = aoc::parse_numbers(s)?;
        let [x, y, vx, vy] = numbers[..].try_into()?;
        Ok(((x, y), (vx, vy)))
    };
    input.lines().map(parse_robot).try_collect()
}

fn get_safety_factor(robots: &[(Point, Point)], width: i64, height: i64) -> usize {
    let final_positions = robots
        .iter()
        .map(|r| robot_position_after(r, 100, width, height));
    let quadrant_counts = final_positions
        .map(|(x, y)| ((x - width / 2).signum(), (y - height / 2).signum()))
        .filter(|&(x_cmp, y_cmp)| x_cmp != 0 && y_cmp != 0)
        .counts();
    quadrant_counts.values().product()
}

fn robot_position_after(robot: &(Point, Point), seconds: i64, width: i64, height: i64) -> Point {
    let &((x, y), (vx, vy)) = robot;
    let final_x = (x + seconds * vx).rem_euclid(width);
    let final_y = (y + seconds * vy).rem_euclid(height);
    (final_x, final_y)
}

#[allow(dead_code)]
fn print_steps(robots: &[(Point, Point)]) {
    let w = 101;
    let h = 103;
    for seconds in 1..10_000 {
        let positions: &HashSet<Point> = &robots
            .iter()
            .map(|r| robot_position_after(r, seconds, w, h))
            .collect();

        let grid: String = (0..h)
            .flat_map(|y| {
                (0..w)
                    .map(move |x| {
                        if positions.contains(&(x, y)) {
                            'x'
                        } else {
                            '.'
                        }
                    })
                    .chain(['\n'])
            })
            .collect();
        println!("after {seconds}s\n{grid}\n");
    }
}

#[test]
fn sample_test() {
    let sample = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
    let robots = parse_robots(sample).unwrap();
    assert_eq!(get_safety_factor(&robots, 11, 7), 12)
}
