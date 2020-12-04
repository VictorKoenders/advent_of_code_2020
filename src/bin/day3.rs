use aoc_2020::*;

fn main() {
    let input = input("day3.txt");
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

fn part1(map: &Map) {
    let tree_count = map.get_tree_count(3, 1);

    println!("{} trees", tree_count);
}
fn part2(map: &Map) {
    let tree_count_1_1 = map.get_tree_count(1, 1);
    let tree_count_3_1 = map.get_tree_count(3, 1);
    let tree_count_5_1 = map.get_tree_count(5, 1);
    let tree_count_7_1 = map.get_tree_count(7, 1);
    let tree_count_1_2 = map.get_tree_count(1, 2);

    println!("1x1: {}", tree_count_1_1);
    println!("3x1: {}", tree_count_3_1);
    println!("5x1: {}", tree_count_5_1);
    println!("7x1: {}", tree_count_7_1);
    println!("1x2: {}", tree_count_1_2);

    println!(
        "Multiplied: {}",
        tree_count_1_1 * tree_count_1_2 * tree_count_3_1 * tree_count_5_1 * tree_count_7_1
    );
}

struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn parse(input: &[String]) -> Self {
        let height = input.len();
        let width = input[0].len();
        let mut cells = Vec::with_capacity(width * height);
        for line in input {
            for byte in line.bytes() {
                cells.push(Cell::from_byte(byte))
            }
        }
        assert_eq!(cells.len(), cells.capacity());
        assert_eq!(cells.len(), width * height);

        Map {
            cells,
            width,
            height,
        }
    }

    pub fn get(&self, mut x: usize, y: usize) -> &Cell {
        x %= self.width;
        let index = y * self.width + x;
        self.cells.get(index).unwrap()
    }

    pub fn get_tree_count(&self, x_diff: usize, y_diff: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut tree_count = 0;
        while y < self.height {
            let cell = self.get(x, y);
            if let Cell::Tree = cell {
                tree_count += 1;
            }

            x += x_diff;
            y += y_diff;
        }

        tree_count
    }
}

enum Cell {
    Open,
    Tree,
}

impl Cell {
    pub fn from_byte(b: u8) -> Self {
        match b {
            b'.' => Self::Open,
            b'#' => Self::Tree,
            x => panic!("Unexpected cell: {:?}", x),
        }
    }
}
