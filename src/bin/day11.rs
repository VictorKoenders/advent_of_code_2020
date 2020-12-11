use aoc_2020::*;

fn main() {
    let input = input("day11.txt");
    let map = Map::parse(&input);

    let start = std::time::Instant::now();
    println!("Part 1:");
    part1(&map);
    println!("Done in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2:");
    part2(&map);
    println!("Done in {:?}", start.elapsed());
}

fn part1(input: &Map) {
    let mut current = input.clone();
    loop {
        let mut has_change = false;
        let mut next = current.clone();

        for y in 0..current.height as isize {
            for x in 0..current.width as isize {
                let pos = current.get_or_floor(x, y);

                let adjacent = &[
                    current.get_or_floor(x - 1, y - 1),
                    current.get_or_floor(x, y - 1),
                    current.get_or_floor(x + 1, y - 1),
                    current.get_or_floor(x - 1, y),
                    current.get_or_floor(x + 1, y),
                    current.get_or_floor(x - 1, y + 1),
                    current.get_or_floor(x, y + 1),
                    current.get_or_floor(x + 1, y + 1),
                ];

                if pos.is_empty() && adjacent.iter().all(Position::is_not_occupied) {
                    // If a seat is empty and there are no occupied seats adjacent to it, the seat becomes occupied.
                    next.set(x, y, Position::Occupied);
                    has_change = true;
                } else if pos.is_occupied()
                    && adjacent.iter().filter(|p| p.is_occupied()).count() >= 4
                {
                    // If a seat is occupied and four or more seats adjacent to it are also occupied, the seat becomes empty.
                    next.set(x, y, Position::Empty);
                    has_change = true;
                }
            }
        }

        current = next;

        if !has_change {
            break;
        }
    }

    println!(
        "There are {} occupied seats",
        current.positions.iter().filter(|p| p.is_occupied()).count()
    );
}

fn part2(input: &Map) {
    let mut current = input.clone();
    loop {
        let mut has_change = false;
        let mut next = current.clone();

        for y in 0..current.height as isize {
            for x in 0..current.width as isize {
                let pos = current.get_or_floor(x, y);
                let view_directions = &[
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];
                let mut seats = Vec::with_capacity(view_directions.len());
                for direction in view_directions {
                    seats.push(current.get_next_chair_in_ray((x, y), *direction));
                }

                if pos.is_empty() && seats.iter().all(Position::is_not_occupied) {
                    // If a seat is empty and there are no occupied seats adjacent to it, the seat becomes occupied.
                    next.set(x, y, Position::Occupied);
                    has_change = true;
                } else if pos.is_occupied() && seats.iter().filter(|p| p.is_occupied()).count() >= 5
                {
                    // If a seat is occupied and five or more seats adjacent to it are also occupied, the seat becomes empty.
                    next.set(x, y, Position::Empty);
                    has_change = true;
                }
            }
        }

        current = next;

        if !has_change {
            break;
        }
    }

    println!(
        "There are {} occupied seats",
        current.positions.iter().filter(|p| p.is_occupied()).count()
    );
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    positions: Vec<Position>,
}

impl Map {
    pub fn parse(input: &[String]) -> Self {
        let width = input[0].as_bytes().len();
        let height = input.len();

        let mut positions = Vec::with_capacity(width * height);

        for line in input {
            for c in line.bytes() {
                positions.push(Position::new(c));
            }
        }

        Self {
            width,
            height,
            positions,
        }
    }

    pub fn set(&mut self, x: isize, y: isize, position: Position) {
        let idx = self.width * y as usize + x as usize;
        self.positions[idx] = position;
    }

    pub fn get_or_floor(&self, x: isize, y: isize) -> Position {
        if !self.is_in_bounds(x, y) {
            Position::Floor
        } else {
            let idx = self.width * y as usize + x as usize;
            self.positions[idx]
        }
    }

    fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
    }

    pub fn get_next_chair_in_ray(&self, mut pos: (isize, isize), dir: (isize, isize)) -> Position {
        loop {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            if !self.is_in_bounds(pos.0, pos.1) {
                break Position::Floor;
            }
            let tile = self.get_or_floor(pos.0, pos.1);
            if tile.is_chair() {
                break tile;
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

impl Position {
    fn new(b: u8) -> Self {
        match b {
            b'.' => Self::Floor,
            b'L' => Self::Empty,
            b'#' => Self::Occupied,
            x => panic!("Unexpected position: {:?}", x as char),
        }
    }

    fn is_occupied(&self) -> bool {
        matches!(self, Position::Occupied)
    }
    fn is_empty(&self) -> bool {
        matches!(self, Position::Empty)
    }
    fn is_not_occupied(&self) -> bool {
        !self.is_occupied()
    }
    fn is_chair(&self) -> bool {
        matches!(self, Position::Empty | Position::Occupied)
    }
}
