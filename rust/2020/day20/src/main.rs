use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read, Result};

type Location = (usize, usize);
type Boundary = usize;
type Boundaries = [Boundary; 4];
type BoundaryCache = Vec<[Boundaries; 8]>;
type ImageMap = HashMap<Location, (usize, usize)>;

#[derive(Debug, PartialEq)]
struct Image {
    tile_no: usize,
    picture: Vec<Vec<bool>>,
    boundaries: Boundaries,
    boundary_size: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Position {
    Top,
    Right,
    Bottom,
    Left,
}

#[cfg(test)]
mod day20_tests {
    use super::*;

    static TEST_INPUT: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
";

    fn get_test_input() -> Vec<Image> {
        parse_input(TEST_INPUT)
    }

    #[test]
    fn should_parse_an_image() {
        let image = TEST_INPUT.split("\n\n").nth(0).unwrap();
        assert_eq!(
            Image {
                tile_no: 2311,
                picture: vec![
                    vec![true, false, false, true, false, false, false, false],
                    vec![false, false, false, true, true, false, false, true],
                    vec![true, true, true, false, true, false, false, false],
                    vec![true, false, true, true, false, true, true, true],
                    vec![true, false, false, false, true, false, true, true],
                    vec![true, false, true, false, true, false, false, true],
                    vec![false, true, false, false, false, false, true, false],
                    vec![true, true, false, false, false, true, false, true],
                ],
                //  Max: 1023
                boundaries: [0b0011010010, 0b0001011001, 0b0011100111, 0b0111110010],
                boundary_size: 10,
            },
            parse_image(image)
        );
    }

    #[test]
    fn should_find_image_boundaries() {
        let picture = vec![
            vec![
                false, false, true, true, false, true, false, false, true, false,
            ],
            vec![
                true, true, false, false, true, false, false, false, false, false,
            ],
            vec![
                true, false, false, false, true, true, false, false, true, false,
            ],
            vec![
                true, true, true, true, false, true, false, false, false, true,
            ],
            vec![
                true, true, false, true, true, false, true, true, true, false,
            ],
            vec![
                true, true, false, false, false, true, false, true, true, true,
            ],
            vec![
                false, true, false, true, false, true, false, false, true, true,
            ],
            vec![
                false, false, true, false, false, false, false, true, false, false,
            ],
            vec![
                true, true, true, false, false, false, true, false, true, false,
            ],
            vec![
                false, false, true, true, true, false, false, true, true, true,
            ],
        ];
        assert_eq!(
            [
                0b0011010010, //vec![2, 3, 5, 8],
                0b0001011001, //vec![3, 5, 6, 9],
                0b0011100111, //vec![2, 3, 4, 7, 8, 9],
                0b0111110010, //vec![1, 2, 3, 4, 5, 8],
            ],
            find_boundaries(&picture)
        );
    }

    #[test]
    fn should_flip_a_boundary() {
        assert_eq!(0b0100101100, flip_bits(0b0011010010, 10)); //  vec![1, 4, 6, 7]
        assert_eq!(0b1001101000, flip_bits(0b0001011001, 10)); //  vec![0, 3, 4, 6]
        assert_eq!(0b1110011100, flip_bits(0b0011100111, 10)); //  vec![0, 1, 2, 5, 6, 7]
        assert_eq!(0b0100111110, flip_bits(0b0111110010, 10)); //  vec![1, 4, 5, 6, 7, 8]
    }

    #[test]
    fn should_flip_a_set_of_boundaries() {
        assert_eq!(
            [0b0011100111, 0b1001101000, 0b0011010010, 0b0100111110,],
            flip_boundaries(
                &[
                    0b0011010010, //vec![2, 3, 5, 8],
                    0b0001011001, //vec![3, 5, 6, 9],
                    0b0011100111, //vec![2, 3, 4, 7, 8, 9],
                    0b0111110010, //vec![1, 2, 3, 4, 5, 8],
                ],
                10
            )
        );
    }

    #[test]
    fn should_rotate_a_set_of_boundaries() {
        assert_eq!(
            [0b0100111110, 0b0011010010, 0b1001101000, 0b0011100111,],
            rotate_boundaries(
                &[
                    0b0011010010, //vec![2, 3, 5, 8],
                    0b0001011001, //vec![3, 5, 6, 9],
                    0b0011100111, //vec![2, 3, 4, 7, 8, 9],
                    0b0111110010, //vec![1, 2, 3, 4, 5, 8],
                ],
                10
            )
        );
    }

    #[test]
    fn should_solve_the_input() {
        let images = get_test_input();
        let boundaries = generate_all_orientations(&images);
        let mut map = HashMap::<Location, (usize, usize)>::new();

        let result = solve(&images, &boundaries, &mut map, (0, 0), 2);

        assert!(result.is_some());

        let result_map = result.unwrap();

        assert_eq!(1951, images[result_map[&(0, 0)].0].tile_no);
        assert_eq!(3079, images[result_map[&(2, 0)].0].tile_no);
        assert_eq!(2971, images[result_map[&(0, 2)].0].tile_no);
        assert_eq!(1171, images[result_map[&(2, 2)].0].tile_no);
    }

    #[test]
    fn should_flip_an_image() {
        assert_eq!(
            vec![
                vec![true, true, false, false, false, true, false, true],
                vec![false, true, false, false, false, false, true, false],
                vec![true, false, true, false, true, false, false, true],
                vec![true, false, false, false, true, false, true, true],
                vec![true, false, true, true, false, true, true, true],
                vec![true, true, true, false, true, false, false, false],
                vec![false, false, false, true, true, false, false, true],
                vec![true, false, false, true, false, false, false, false],
            ],
            flip_image(&vec![
                vec![true, false, false, true, false, false, false, false],
                vec![false, false, false, true, true, false, false, true],
                vec![true, true, true, false, true, false, false, false],
                vec![true, false, true, true, false, true, true, true],
                vec![true, false, false, false, true, false, true, true],
                vec![true, false, true, false, true, false, false, true],
                vec![false, true, false, false, false, false, true, false],
                vec![true, true, false, false, false, true, false, true],
            ])
        );
    }

    #[test]
    fn should_rotate_an_image() {
        assert_eq!(
            vec![
                vec![true, false, true, true, true, true, false, true],
                vec![true, true, false, false, false, true, false, false],
                vec![false, false, true, false, true, true, false, false],
                vec![false, false, false, false, true, false, true, true],
                vec![false, false, true, true, false, true, true, false],
                vec![true, false, false, false, true, false, false, false],
                vec![false, true, false, true, true, false, false, false],
                vec![true, false, true, true, true, false, true, false],
            ],
            rotate_image(&vec![
                vec![true, false, false, true, false, false, false, false],
                vec![false, false, false, true, true, false, false, true],
                vec![true, true, true, false, true, false, false, false],
                vec![true, false, true, true, false, true, true, true],
                vec![true, false, false, false, true, false, true, true],
                vec![true, false, true, false, true, false, false, true],
                vec![false, true, false, false, false, false, true, false],
                vec![true, true, false, false, false, true, false, true],
            ])
        );
    }

    #[test]
    fn should_find_a_monster() {
        assert!(check_for_monster_at(
            &vec![
                vec![
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, true, false,
                ],
                vec![
                    true, false, false, false, false, true, true, false, false, false, false, true,
                    true, false, false, false, false, true, true, true,
                ],
                vec![
                    false, true, false, false, true, false, false, true, false, false, true, false,
                    false, true, false, false, true, false, false, false,
                ],
            ],
            (0, 0)
        ));
    }

    #[test]
    fn day20a_test() {
        let images = get_test_input();
        let (result, _) = day20a(&images);
        assert_eq!(20899048083289, result);
    }

    #[test]
    fn day20b_test() {
        let images = get_test_input();
        let (_, image_map) = day20a(&images);
        let result = day20b(&images, &image_map);
        assert_eq!(273, result);
    }
}

static MONSTER_LEN: usize = 20;
static MONSTER_ROW_ONE: usize = 2;
static MONSTER_ROW_TWO: usize = 549255;
static MONSTER_ROW_THREE: usize = 299592;
static MONSTER_SQUARES: usize = 15;

fn to_value(vector: &[bool]) -> usize {
    vector
        .iter()
        .fold(0, |acc, &b| acc * 2 + (if b { 1 } else { 0 }))
}

fn flip_bits(number: usize, boundary_size: usize) -> usize {
    (0..boundary_size)
        .map(|i| 1 << i)
        .map(|n| number & n == n)
        .fold(0, |acc, b| acc * 2 + (if b { 1 } else { 0 }))
}

fn check_for_monster_at(image: &Vec<Vec<bool>>, location: Location) -> bool {
    let (col, row) = location;

    (to_value(&image[row][col..col + MONSTER_LEN]) & MONSTER_ROW_ONE == MONSTER_ROW_ONE)
        && (to_value(&image[row + 1][col..col + MONSTER_LEN]) & MONSTER_ROW_TWO == MONSTER_ROW_TWO)
        && (to_value(&image[row + 2][col..col + MONSTER_LEN]) & MONSTER_ROW_THREE
            == MONSTER_ROW_THREE)
}

fn find_monsters(image: &Vec<Vec<bool>>) -> usize {
    let max_col_row = image.len();
    let mut monster_count = 0;
    let mut row = 0;
    let mut col = 0;

    while row + 2 < max_col_row {
        while col + MONSTER_LEN <= max_col_row {
            if check_for_monster_at(image, (col, row)) {
                // col += MONSTER_LEN;
                monster_count += 1;
            }

            col += 1;
        }

        col = 0;
        row += 1;
    }

    monster_count
}

fn flip_image(image: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    image.into_iter().rev().map(|v| v.clone()).collect()
}

fn rotate_image(image: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let max = image.len() - 1;
    (0..=max)
        .map(|y| (0..=max).map(|x| image[max - x][y]).collect())
        .collect()
}

fn orient_image(image: &Vec<Vec<bool>>, orientation: usize) -> Vec<Vec<bool>> {
    match orientation {
        0 => image.clone(),
        1 => rotate_image(image),
        2 => rotate_image(&rotate_image(image)),
        3 => rotate_image(&rotate_image(&rotate_image(image))),
        4 => flip_image(image),
        5 => rotate_image(&flip_image(image)),
        6 => rotate_image(&rotate_image(&flip_image(image))),
        7 => rotate_image(&rotate_image(&rotate_image(&flip_image(image)))),
        _ => unreachable!(),
    }
}

fn get_boundary(boundaries: &Boundaries, position: Position) -> Boundary {
    match position {
        Position::Top => boundaries[0],
        Position::Right => boundaries[1],
        Position::Bottom => boundaries[2],
        Position::Left => boundaries[3],
    }
}

fn solve(
    images: &Vec<Image>,
    boundaries: &BoundaryCache,
    current_map: &ImageMap,
    location: Location,
    max_col_row: usize,
) -> Option<ImageMap> {
    let (col, row) = location;

    let seen_images: HashSet<usize> = current_map.values().map(|&(index, _)| index).collect();
    let mut current_map = current_map.clone();

    for (index, _) in images.iter().enumerate() {
        if seen_images.contains(&index) {
            continue;
        }

        for (orientation, bounds) in boundaries[index].iter().enumerate() {
            if row > 0 {
                //  Doesn't match the row above, so can't use it
                let (above_index, above_orientation) = current_map[&(col, row - 1)];
                if get_boundary(&bounds, Position::Top)
                    != get_boundary(
                        &boundaries[above_index][above_orientation],
                        Position::Bottom,
                    )
                {
                    continue;
                }
            }

            if col > 0 {
                //  Doesn't match the col to the left, so can't use it
                let (left_index, left_orientation) = current_map[&(col - 1, row)];
                let left_bound = get_boundary(&bounds, Position::Left);
                let right_bound =
                    get_boundary(&boundaries[left_index][left_orientation], Position::Right);
                if left_bound != right_bound {
                    continue;
                }
            }

            if col < max_col_row {
                let right_matches = find_matching_edges(
                    get_boundary(bounds, Position::Right),
                    Position::Left,
                    images,
                    boundaries,
                    &seen_images,
                    index,
                );
                if right_matches.len() == 0 {
                    continue;
                }
            }

            if row < max_col_row {
                let down_matches = find_matching_edges(
                    get_boundary(bounds, Position::Bottom),
                    Position::Top,
                    images,
                    boundaries,
                    &seen_images,
                    index,
                );
                if down_matches.len() == 0 {
                    continue;
                }
            }

            //  Let's try this one here, see if it works
            current_map.insert((col, row), (index, orientation));

            let next_loc = if col == max_col_row && row == max_col_row {
                return Some(current_map.clone());
            } else if col == max_col_row {
                (0, row + 1)
            } else {
                (col + 1, row)
            };

            match solve(images, boundaries, &current_map, next_loc, max_col_row) {
                Some(m) => return Some(m),
                None => {}
            };
        }
    }

    None
}

fn find_matching_edges(
    boundary: Boundary,
    position: Position,
    images: &Vec<Image>,
    boundaries: &BoundaryCache,
    seen_images: &HashSet<usize>,
    current_index: usize,
) -> Vec<(usize, usize)> {
    images
        .iter()
        .enumerate()
        .filter(|(i, _)| !seen_images.contains(i) && current_index != *i)
        .flat_map(|(i, _)| {
            boundaries[i].iter().enumerate().map(move |(or, bound)| {
                if get_boundary(bound, position) == boundary {
                    Some((i, or))
                } else {
                    None
                }
            })
        })
        .filter_map(|x| x)
        .collect()
}

fn find_boundaries(picture: &Vec<Vec<bool>>) -> Boundaries {
    let top = to_value(&picture[0][..]);
    let bottom = to_value(&picture[picture.len() - 1][..]);
    let right = to_value(
        &picture
            .iter()
            .enumerate()
            .map(|(_, row)| row[row.len() - 1])
            .collect::<Vec<bool>>()[..],
    );
    let left = to_value(
        &picture
            .iter()
            .enumerate()
            .map(|(_, row)| row[0])
            .collect::<Vec<bool>>()[..],
    );

    [top, right, bottom, left]
}

fn generate_all_orientations(images: &Vec<Image>) -> BoundaryCache {
    images
        .iter()
        .map(|image| generate_orientations(&image.boundaries, image.boundary_size))
        .collect()
}

fn generate_orientations(boundaries: &Boundaries, boundary_size: usize) -> [Boundaries; 8] {
    let r90 = rotate_boundaries(&boundaries, boundary_size);
    let r180 = rotate_boundaries(&r90, boundary_size);
    let r270 = rotate_boundaries(&r90, boundary_size);
    let flip = flip_boundaries(&boundaries, boundary_size);
    let flip90 = rotate_boundaries(&flip, boundary_size);
    let flip180 = rotate_boundaries(&flip90, boundary_size);
    let flip270 = rotate_boundaries(&flip180, boundary_size);

    [
        boundaries.clone(),
        r90,
        r180,
        r270,
        flip,
        flip90,
        flip180,
        flip270,
    ]
}

fn flip_boundaries(boundaries: &Boundaries, boundary_size: usize) -> Boundaries {
    [
        boundaries[2],
        flip_bits(boundaries[1], boundary_size),
        boundaries[0],
        flip_bits(boundaries[3], boundary_size),
    ]
}

fn rotate_boundaries(boundaries: &Boundaries, boundary_size: usize) -> Boundaries {
    [
        flip_bits(boundaries[3], boundary_size),
        boundaries[0],
        flip_bits(boundaries[1], boundary_size),
        boundaries[2],
    ]
}

fn parse_image(image: &str) -> Image {
    let mut lines = image.lines();
    let title_line = lines.next().unwrap();
    let tile_no = title_line
        .replace("Tile ", "")
        .replace(":", "")
        .parse::<usize>()
        .unwrap();
    let full_picture = lines
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    let boundaries = find_boundaries(&full_picture);
    let height = full_picture.len();
    let picture = full_picture[1..(height - 1)]
        .into_iter()
        .map(|row| row[1..(height - 1)].into_iter().map(|&b| b).collect())
        .collect();

    Image {
        tile_no: tile_no,
        picture: picture,
        boundaries: boundaries,
        boundary_size: height,
    }
}

fn parse_input(input: &str) -> Vec<Image> {
    input
        .split("\n\n")
        .filter(|&line| line != "")
        .map(|image| parse_image(image))
        .collect()
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day20a(images: &Vec<Image>) -> (usize, ImageMap) {
    let boundaries = generate_all_orientations(&images);
    let max_col_row = (images.len() as f64).sqrt() as usize - 1;
    let result = solve(
        &images,
        &boundaries,
        &HashMap::<Location, (usize, usize)>::new(),
        (0, 0),
        max_col_row,
    )
    .unwrap();

    (
        images[result[&(0, 0)].0].tile_no
            * images[result[&(0, max_col_row)].0].tile_no
            * images[result[&(max_col_row, 0)].0].tile_no
            * images[result[&(max_col_row, max_col_row)].0].tile_no,
        result,
    )
}

fn day20b(images: &Vec<Image>, image_map: &ImageMap) -> usize {
    let max_col_row = (images.len() as f64).sqrt() as usize;
    let height = images[0].picture.len();
    let full_image: Vec<Vec<bool>> = (0..max_col_row)
        .flat_map(|y| {
            (0..max_col_row)
                .map(|x| {
                    let (index, orientation) = image_map[&(x, y)];
                    orient_image(&images[index].picture, orientation)
                })
                .fold(vec![vec![]; height], |acc, image| {
                    (0..height)
                        .map(|x| [&acc[x][..], &image[x][..]].concat())
                        .collect()
                })
        })
        .collect();

    let total_count = full_image.iter().flat_map(|v| v).filter(|&&b| b).count();

    for orientation in 0..8 {
        let test_image = orient_image(&full_image, orientation);

        match find_monsters(&test_image) {
            0 => {}
            monster_count => {
                return total_count - (monster_count * MONSTER_SQUARES);
            }
        }
    }

    0
}

fn main() -> Result<()> {
    let input = read_file("input")?;
    let images = parse_input(&input);

    use std::time::Instant;
    let total = Instant::now();

    let (result, image_map) = day20a(&images);
    println!(
        "Day 20A - {} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day20b(&images, &image_map);
    println!(
        "Day 20B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());

    Ok(())
}
