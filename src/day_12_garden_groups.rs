use itertools::iproduct;

pub fn run(input: &str) -> aoc::Answer {
    let (garden, w, h) = aoc::parse_char_grid(input)?;

    let mut fence_price = 0;
    let mut visited = vec![vec![false; w]; h];
    for (x, y) in iproduct!(0..w, 0..h) {
        if visited[y][x] {
            continue;
        }
        fence_price += get_region_price((x, y), &garden, &mut visited);
    }

    let mut fence_bulk_price = 0;
    let mut visited = vec![vec![false; w]; h];
    for (x, y) in iproduct!(0..w, 0..h) {
        if visited[y][x] {
            continue;
        }
        fence_bulk_price += get_region_bulk_price((x, y), &garden, &mut visited);
    }

    aoc::answer(fence_price, fence_bulk_price)
}

fn get_region_price(
    start: (usize, usize),
    garden: &[Vec<char>],
    visited: &mut [Vec<bool>],
) -> usize {
    let mut to_visit = vec![start];
    let mut area = 0;
    let mut perimeter = 0;
    while let Some((x, y)) = to_visit.pop() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;
        let plant_type = garden[y][x];
        area += 1;
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            let neighbor_plant = garden.get(ny).and_then(|row| row.get(nx));
            if neighbor_plant == Some(&plant_type) {
                to_visit.push((nx, ny))
            } else {
                perimeter += 1;
            }
        }
    }
    area * perimeter
}

fn get_region_bulk_price(
    start: (usize, usize),
    garden: &[Vec<char>],
    visited: &mut [Vec<bool>],
) -> usize {
    let mut to_visit = vec![start];
    let mut area = 0;
    let mut side_count = 0;
    while let Some((x, y)) = to_visit.pop() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;
        let plant_type = garden[y][x];
        area += 1;
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            let neighbor_plant = garden.get(ny).and_then(|row| row.get(nx));
            if neighbor_plant == Some(&plant_type) {
                to_visit.push((nx, ny))
            } else {
                // Rotate right.
                let (ox, oy) = (x.wrapping_add_signed(-dy), y.wrapping_add_signed(dx));
                let ortho_plant = garden.get(oy).and_then(|row| row.get(ox));
                let ortho_neighbor_plant = garden
                    .get(oy.wrapping_add_signed(dy))
                    .and_then(|row| row.get(ox.wrapping_add_signed(dx)));
                let first_of_side =
                    ortho_plant != Some(&plant_type) || ortho_neighbor_plant == Some(&plant_type);
                if first_of_side {
                    side_count += 1;
                }
            }
        }
    }
    area * side_count
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
