use anyhow::bail;
use itertools::Itertools;
use regex::bytes::Regex;

pub fn run(input: &str) -> aoc::Answer {
    let robots = parse_robots(input)?;
    aoc::answers(
        get_safety_factor(&robots, 101, 103),
        find_easter_egg(&robots, 101, 103)?,
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

fn find_easter_egg(robots: &[(Point, Point)], width: i64, height: i64) -> aoc::Result<i64> {
    let easter_egg_lines = [
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "x                             x",
        "x                             x",
        "x                             x",
        "x                             x",
        "x              x              x",
        "x             xxx             x",
        "x            xxxxx            x",
        "x           xxxxxxx           x",
        "x          xxxxxxxxx          x",
        "x            xxxxx            x",
        "x           xxxxxxx           x",
        "x          xxxxxxxxx          x",
        "x         xxxxxxxxxxx         x",
        "x        xxxxxxxxxxxxx        x",
        "x          xxxxxxxxx          x",
        "x         xxxxxxxxxxx         x",
        "x        xxxxxxxxxxxxx        x",
        "x       xxxxxxxxxxxxxxx       x",
        "x      xxxxxxxxxxxxxxxxx      x",
        "x        xxxxxxxxxxxxx        x",
        "x       xxxxxxxxxxxxxxx       x",
        "x      xxxxxxxxxxxxxxxxx      x",
        "x     xxxxxxxxxxxxxxxxxxx     x",
        "x    xxxxxxxxxxxxxxxxxxxxx    x",
        "x             xxx             x",
        "x             xxx             x",
        "x             xxx             x",
        "x                             x",
        "x                             x",
        "x                             x",
        "x                             x",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    ];
    let easter_egg_re = Regex::new(&easter_egg_lines.join(".*")).unwrap();
    let mut room = vec![b' '; (width * height) as usize];
    for seconds in 1..1_000_000 {
        room.fill(b' ');
        for robot in robots {
            let (x, y) = robot_position_after(robot, seconds, width, height);
            room[(y * width + x) as usize] = b'x'
        }
        if easter_egg_re.is_match(&room) {
            return Ok(seconds);
        }
    }
    bail!("Easter egg not found")
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
