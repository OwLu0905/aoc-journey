use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct PathPosition(i32, i32);

impl PathPosition {
    fn subtract(&self, other: &PathPosition) -> PathPosition {
        PathPosition(self.0 - other.0, self.1 - other.1)
    }
}

#[derive(Debug)]
struct Terrain {
    puzzle: HashMap<PathPosition, Material>,
}

impl Terrain {
    pub fn new() -> Self {
        Terrain {
            puzzle: HashMap::new(),
        }
    }
    pub fn set_material(&mut self, pos: PathPosition, material: Material) {
        self.puzzle.insert(pos, material);
    }
}

pub fn problem14_1(path: &str) -> i32 {
    let file = File::open(path).expect("cant open files");
    let reader = BufReader::new(file);

    let mut rock_list = Vec::new();

    for line in reader.lines() {
        let line_str = line.unwrap();
        let rock_pos: Vec<(i32, i32)> = line_str
            .split(" -> ")
            .map(|pos| {
                let pos_split: Vec<i32> =
                    pos.split(",").map(|p| p.parse::<i32>().unwrap()).collect();
                (pos_split[0], pos_split[1])
            })
            .collect();
        rock_list.push(rock_pos);
    }

    let mut terrain = Terrain::new();
    let mut max_y = 0;

    for rock in rock_list {
        let length = rock.len();
        for i in 0..length - 1 {
            let current_rock = PathPosition(rock[i].0, rock[i].1);
            let next_rock = PathPosition(rock[i + 1].0, rock[i + 1].1);

            let check_pos = next_rock.subtract(&current_rock);

            match check_pos {
                PathPosition(gap_x, 0) if gap_x > 0 => {
                    if max_y < next_rock.1 {
                        max_y = next_rock.1;
                    }
                    for j in 0..=gap_x {
                        let rock = PathPosition(rock[i].0 + j, next_rock.1);
                        terrain.set_material(rock, Material::Rock)
                    }
                }
                PathPosition(gap_x, 0) if gap_x < 0 => {
                    if max_y < next_rock.1 {
                        max_y = next_rock.1;
                    }
                    let gap_x = gap_x * -1;
                    for j in 0..=gap_x {
                        let rock = PathPosition(rock[i].0 - j, next_rock.1);
                        terrain.set_material(rock, Material::Rock)
                    }
                }
                PathPosition(0, gap_y) if gap_y > 0 => {
                    for j in 0..=gap_y {
                        let rock = PathPosition(next_rock.0, rock[i].1 + j);
                        terrain.set_material(rock, Material::Rock)
                    }

                    if max_y < rock[i].1 + gap_y {
                        max_y = rock[i].1 + gap_y;
                    }
                }
                PathPosition(0, gap_y) if gap_y < 0 => {
                    let gap_y = gap_y * -1;
                    for j in 0..=gap_y {
                        let rock = PathPosition(next_rock.0, rock[i].1 - j);
                        terrain.set_material(rock, Material::Rock)
                    }
                    if max_y < rock[i].1 - gap_y {
                        max_y = rock[i].1 - gap_y;
                    }
                }
                _ => {
                    panic!("unkown material layout")
                }
            };
        }
    }

    let mut count = 0;
    let mut start_sand = PathPosition(500, 0);

    while start_sand.1 <= max_y {
        let x = start_sand.0;
        let y = start_sand.1;

        let down = PathPosition(x, y + 1);

        let down_check = terrain.puzzle.get_mut(&down);
        if down_check.is_none() || (down_check.is_some() && *down_check.unwrap() == Material::Air) {
            terrain.puzzle.insert(down, Material::Air);
            start_sand = PathPosition(x, y + 1);
            continue;
        }

        let left_down = PathPosition(x - 1, y + 1);

        let down_check = terrain.puzzle.get_mut(&left_down);
        if down_check.is_none() || (down_check.is_some() && *down_check.unwrap() == Material::Air) {
            terrain.puzzle.insert(left_down, Material::Air);
            start_sand = PathPosition(x - 1, y + 1);
            continue;
        }

        let right_down = PathPosition(x + 1, y + 1);

        let down_check = terrain.puzzle.get_mut(&right_down);
        if down_check.is_none() || (down_check.is_some() && *down_check.unwrap() == Material::Air) {
            terrain.puzzle.insert(right_down, Material::Air);
            start_sand = PathPosition(x + 1, y + 1);
            continue;
        }

        terrain.puzzle.insert(PathPosition(x, y), Material::Sand);

        count += 1;
        start_sand = PathPosition(500, 0);
    }

    count
}

pub fn problem14_2(path: &str) -> i32 {
    let file = File::open(path).expect("cant open files");
    let reader = BufReader::new(file);

    let mut rock_list = Vec::new();

    for line in reader.lines() {
        let line_str = line.unwrap();
        let rock_pos: Vec<(i32, i32)> = line_str
            .split(" -> ")
            .map(|pos| {
                let pos_split: Vec<i32> =
                    pos.split(",").map(|p| p.parse::<i32>().unwrap()).collect();
                (pos_split[0], pos_split[1])
            })
            .collect();
        rock_list.push(rock_pos);
    }

    let mut terrain = Terrain::new();
    let mut max_y = 0;

    for rock in rock_list {
        let length = rock.len();
        for i in 0..length - 1 {
            let current_rock = PathPosition(rock[i].0, rock[i].1);
            let next_rock = PathPosition(rock[i + 1].0, rock[i + 1].1);

            let check_pos = next_rock.subtract(&current_rock);

            match check_pos {
                PathPosition(gap_x, 0) if gap_x > 0 => {
                    if max_y < next_rock.1 {
                        max_y = next_rock.1;
                    }
                    for j in 0..=gap_x {
                        let rock = PathPosition(rock[i].0 + j, next_rock.1);
                        terrain.set_material(rock, Material::Rock)
                    }
                }
                PathPosition(gap_x, 0) if gap_x < 0 => {
                    if max_y < next_rock.1 {
                        max_y = next_rock.1;
                    }
                    let gap_x = gap_x * -1;
                    for j in 0..=gap_x {
                        let rock = PathPosition(rock[i].0 - j, next_rock.1);
                        terrain.set_material(rock, Material::Rock)
                    }
                }
                PathPosition(0, gap_y) if gap_y > 0 => {
                    for j in 0..=gap_y {
                        let rock = PathPosition(next_rock.0, rock[i].1 + j);
                        terrain.set_material(rock, Material::Rock)
                    }

                    if max_y < rock[i].1 + gap_y {
                        max_y = rock[i].1 + gap_y;
                    }
                }
                PathPosition(0, gap_y) if gap_y < 0 => {
                    let gap_y = gap_y * -1;
                    for j in 0..=gap_y {
                        let rock = PathPosition(next_rock.0, rock[i].1 - j);
                        terrain.set_material(rock, Material::Rock)
                    }
                    if max_y < rock[i].1 - gap_y {
                        max_y = rock[i].1 - gap_y;
                    }
                }
                _ => {
                    panic!("unkown material layout")
                }
            };
        }
    }

    let max_y = max_y + 2;

    let mut count = 0;
    let mut start_sand = PathPosition(500, 0);

    while start_sand.1 <= max_y {
        let x = start_sand.0;
        let y = start_sand.1;

        if y + 1 == max_y {
            let floor = PathPosition(x, y + 1);
            terrain.puzzle.insert(floor, Material::Rock);
            terrain.puzzle.insert(PathPosition(x, y), Material::Sand);

            count += 1;
            start_sand = PathPosition(500, 0);
            continue;
        }

        let down = PathPosition(x, y + 1);
        let down_check = terrain.puzzle.get_mut(&down);
        if down_check.is_none() || (down_check.is_some() && *down_check.unwrap() == Material::Air) {
            terrain.puzzle.insert(down, Material::Air);
            start_sand = PathPosition(x, y + 1);
            continue;
        }

        let left_down = PathPosition(x - 1, y + 1);
        let down_check = terrain.puzzle.get_mut(&left_down);
        if down_check.is_none() || (down_check.is_some() && *down_check.unwrap() == Material::Air) {
            terrain.puzzle.insert(left_down, Material::Air);
            start_sand = PathPosition(x - 1, y + 1);
            continue;
        }

        let right_down = PathPosition(x + 1, y + 1);
        let down_check = terrain.puzzle.get_mut(&right_down);
        if down_check.is_none() || (down_check.is_some() && *down_check.unwrap() == Material::Air) {
            terrain.puzzle.insert(right_down, Material::Air);
            start_sand = PathPosition(x + 1, y + 1);
            continue;
        }

        if x == 500 && y == 0 {
            count += 1;
            break;
        }

        terrain.puzzle.insert(PathPosition(x, y), Material::Sand);
        count += 1;
        start_sand = PathPosition(500, 0);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d14_1() {
        let file_path_simple = "testdata/y2022_p14_simple.txt";
        assert_eq!(problem14_1(file_path_simple), 24);

        let file_path = "testdata/y2022_p14.txt";
        assert_eq!(problem14_1(file_path), 674);
    }

    #[test]
    fn tests_y2022_d14_2() {
        let file_path_simple = "testdata/y2022_p14_simple.txt";
        assert_eq!(problem14_2(file_path_simple), 93);

        let file_path = "testdata/y2022_p14.txt";
        assert_eq!(problem14_2(file_path), 24958);
    }
}
