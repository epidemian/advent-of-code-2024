use itertools::iproduct;
use rustc_hash::FxHashSet as HashSet;

pub fn run(input: &str) -> aoc::Answer {
    let (garden, w, h) = aoc::parse_char_grid(input)?;

    let mut regions = Vec::new();
    let mut visited = Region::default();
    for point in iproduct!(0..w, 0..h) {
        if !visited.contains(&point) {
            let region = get_region_at(point, &garden);
            visited.extend(&region);
            regions.push(region);
        }
    }
    aoc::answer(
        regions.iter().map(get_region_price).sum::<usize>(),
        regions.iter().map(get_region_bulk_price).sum::<usize>(),
    )
}

type Point = (usize, usize);
type Region = HashSet<Point>;
const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn get_region_at(start: Point, garden: &[Vec<char>]) -> Region {
    let mut region = HashSet::default();
    let mut to_visit = vec![start];
    while let Some((x, y)) = to_visit.pop() {
        if !region.insert((x, y)) {
            continue;
        };
        let plant_type = garden[y][x];
        for d in DIRS {
            let (nx, ny) = add_signed((x, y), d);
            let neighbor_plant = garden.get(ny).and_then(|row| row.get(nx));
            if neighbor_plant == Some(&plant_type) {
                to_visit.push((nx, ny))
            }
        }
    }
    region
}

fn get_region_price(region: &Region) -> usize {
    let mut perimeter = 0;
    for &point in region {
        for d in DIRS {
            let neighbor = add_signed(point, d);
            if !region.contains(&neighbor) {
                perimeter += 1;
            }
        }
    }
    region.len() * perimeter
}

fn get_region_bulk_price(region: &Region) -> usize {
    let mut side_count = 0;
    for &point in region {
        for (dx, dy) in DIRS {
            let neighbor = add_signed(point, (dx, dy));
            if !region.contains(&neighbor) {
                let ortho_dir = (-dy, dx); // Rotate right.
                let ortho_point = add_signed(point, ortho_dir);
                let first_of_side = !region.contains(&ortho_point)
                    || region.contains(&add_signed(ortho_point, (dx, dy)));
                if first_of_side {
                    side_count += 1;
                }
            }
        }
    }
    region.len() * side_count
}

fn add_signed((x, y): Point, (dx, dy): (isize, isize)) -> Point {
    (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy))
}

#[test]
fn small_sample_test() {
    let sample = "AAAA
BBCD
BBCC
EEEC
";
    assert_eq!(run(sample).unwrap(), "140 80")
}

#[test]
fn xoxo_sample_test() {
    let sample = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
    assert_eq!(run(sample).unwrap(), "772 436")
}

#[test]
fn e_sample_test() {
    let sample = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
    assert_eq!(run(sample).unwrap(), "692 236")
}

#[test]
fn abba_sample_test() {
    let sample = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
    assert_eq!(run(sample).unwrap(), "1184 368")
}
