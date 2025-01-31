use itertools::iproduct;
use pathfinding::prelude::bfs_reach;
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
    aoc::answers(
        regions.iter().map(get_fence_price).sum::<usize>(),
        regions.iter().map(get_fence_bulk_price).sum::<usize>(),
    )
}

type Point = (usize, usize);
type Region = HashSet<Point>;
const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn get_region_at((x, y): Point, garden: &[Vec<char>]) -> Region {
    let plant_type = garden[y][x];
    bfs_reach((x, y), |&point| {
        DIRS.into_iter()
            .map(move |d| add_signed(point, d))
            .filter(move |&(nx, ny)| garden.get(ny).and_then(|r| r.get(nx)) == Some(&plant_type))
    })
    .collect()
}

fn get_fence_price(region: &Region) -> usize {
    let count_plot_fences = |&point| {
        DIRS.into_iter()
            .filter(|&d| !region.contains(&add_signed(point, d)))
            .count()
    };
    let perimeter: usize = region.iter().map(count_plot_fences).sum();
    region.len() * perimeter
}

fn get_fence_bulk_price(region: &Region) -> usize {
    let count_plot_first_sides = |&point| {
        DIRS.into_iter()
            .filter(|&d| !region.contains(&add_signed(point, d)))
            .filter(|&(dx, dy)| {
                // We know there'll be a fence in the direction dx,dy. We check if we're the first
                // plot on this fence side.
                let ortho_dir = (-dy, dx); // Rotate right.
                let ortho_point = add_signed(point, ortho_dir);
                !region.contains(&ortho_point)
                    || region.contains(&add_signed(ortho_point, (dx, dy)))
            })
            .count()
    };
    let side_count: usize = region.iter().map(count_plot_first_sides).sum();
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
