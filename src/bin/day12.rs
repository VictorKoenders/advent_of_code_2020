use aoc_2020::*;

fn main() {
    let input = input("day12.txt");
    let input = Instruction::parse(&input);

    let start = std::time::Instant::now();
    println!("Part 1:");
    part1(&input);
    println!("Done in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2:");
    part2(&input);
    println!("Done in {:?}", start.elapsed());
}

fn part1(input: &[Instruction]) {
    let mut ship = Ship::default();

    for instruction in input {
        instruction.apply_to_ship(&mut ship);
    }
    println!(
        "Ship at {}/{} (answer: {})",
        ship.x,
        ship.y,
        ship.x.abs() + ship.y.abs()
    );
}
fn part2(input: &[Instruction]) {
    let mut ship = Ship::default();

    for instruction in input {
        instruction.apply_to_ship_waypoint(&mut ship);
    }
    println!(
        "Ship at {}/{} (answer: {})",
        ship.x,
        ship.y,
        ship.x.abs() + ship.y.abs()
    );
}

struct Ship {
    x: isize,
    y: isize,
    facing: isize,
    waypoint_x: isize,
    waypoint_y: isize,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            facing: 0,
            waypoint_x: 10,
            waypoint_y: -1,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

impl Instruction {
    pub fn parse(input: &[String]) -> Vec<Self> {
        let mut result = Vec::new();
        for line in input {
            let num: isize = line.as_str()[1..].parse().unwrap();
            let instruction = match line.as_bytes()[0] {
                b'N' => Self::North(num),
                b'S' => Self::South(num),
                b'E' => Self::East(num),
                b'W' => Self::West(num),
                b'L' => Self::Left(num),
                b'R' => Self::Right(num),
                b'F' => Self::Forward(num),
                x => panic!("Unknown instruction {:?}", x as char),
            };
            result.push(instruction)
        }
        result
    }

    pub fn apply_to_ship(&self, ship: &mut Ship) {
        match self {
            Self::North(num) => ship.y -= num,
            Self::East(num) => ship.x += num,
            Self::South(num) => ship.y += num,
            Self::West(num) => ship.x -= num,
            Self::Left(num) => ship.facing -= num,
            Self::Right(num) => ship.facing += num,
            Self::Forward(num) => {
                let facing = (ship.facing as f32).to_radians();
                let distance = *num as f32;
                let x = (facing.cos() * distance) as isize;
                let y = (facing.sin() * distance) as isize;
                ship.x += x as isize;
                ship.y += y as isize;
            }
        }
    }
    pub fn apply_to_ship_waypoint(&self, ship: &mut Ship) {
        match self {
            Self::North(num) => ship.waypoint_y -= num,
            Self::East(num) => ship.waypoint_x += num,
            Self::South(num) => ship.waypoint_y += num,
            Self::West(num) => ship.waypoint_x -= num,
            Self::Left(num) => {
                let (x, y) = rotate_point((ship.waypoint_x, ship.waypoint_y), -num as f32);
                ship.waypoint_x = x;
                ship.waypoint_y = y;
            }
            Self::Right(num) => {
                let (x, y) = rotate_point((ship.waypoint_x, ship.waypoint_y), *num as f32);
                ship.waypoint_x = x;
                ship.waypoint_y = y;
            }
            Self::Forward(num) => {
                ship.x += ship.waypoint_x * num;
                ship.y += ship.waypoint_y * num;
            }
        }
    }
}

fn rotate_point((x, y): (isize, isize), angle: f32) -> (isize, isize) {
    let angle = angle.to_radians();

    let fx = x as f32;
    let fy = y as f32;

    let new_x = fx * angle.cos() - fy * angle.sin();
    let new_y = fx * angle.sin() - fy * angle.cos();

    let new_x = new_x.round() as isize;
    let new_y = new_y.round() as isize;

    if new_x.abs() != x.abs() && new_x.abs() != y.abs() {
        panic!("new_x {} should be either {} or {}", new_x, x, y);
    }
    if new_y.abs() != x.abs() && new_y.abs() != y.abs() {
        panic!("new_y {} should be either {} or {}", new_y, x, y);
    }

    (new_x, new_y)
}

#[test]
fn test_rotate() {
    assert_eq!(rotate_point((0, 10), 90.0), (-10, 0));
    assert_eq!(rotate_point((10, 10), 90.0), (-10, 10));
    assert_eq!(rotate_point((10, -4), 90.0), (4, 10));
}
