use aoc_2020::*;

fn main() {
    let input = input("day5.txt");
    let input: Vec<Seat> = input.into_iter().map(Seat::parse).collect();

    let start = std::time::Instant::now();
    println!("Part 1:");
    part1(&input);
    println!("Done in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2:");
    part2(&input);
    println!("Done in {:?}", start.elapsed());
}

fn part1(input: &[Seat]) {
    let mut highest_seat = None;
    let mut highest = 0;

    for seat in input {
        let num = seat.get_number();
        let value = seat_id(num);
        if value > highest {
            highest = value;
            highest_seat = Some(seat);
        }
    }

    let seat = highest_seat.unwrap();
    println!("Highest is seat {:?}", seat);
    let num = seat.get_number();
    let id = seat_id(num);
    println!("Row {}, column {}, id {}", num.0, num.1, id);
}
fn part2(input: &[Seat]) {
    let mut ids: Vec<_> = input.iter().map(|s| seat_id(s.get_number())).collect();
    ids.sort_unstable();
    for (id, next_id) in ids.iter().zip(ids.iter().skip(1)) {
        if id + 1 != *next_id {
            println!("There is a gap between {} and {}", id, next_id)
        }
    }
}

#[derive(Debug)]
pub struct Seat {
    row_steps: [RowStep; 7],
    side_steps: [SideStep; 3],
}

impl Seat {
    pub fn parse(input: impl AsRef<str>) -> Self {
        let mut bytes = input.as_ref().bytes();
        let row_steps = array_init::array_init(|_| RowStep::new(bytes.next().unwrap()));
        let side_steps = array_init::array_init(|_| SideStep::new(bytes.next().unwrap()));

        Self {
            row_steps,
            side_steps,
        }
    }

    pub fn get_number(&self) -> (usize, usize) {
        let mut row = 0..128;
        for step in self.row_steps.iter() {
            let middle = ((row.end - row.start) / 2) + row.start;
            match step {
                // front = lower half
                RowStep::Front => row.end = middle,
                // back = higher half
                RowStep::Back => row.start = middle,
            }
        }
        debug_assert!(row.start == row.end - 1);

        let mut col = 0..8;

        for step in self.side_steps.iter() {
            let middle = ((col.end - col.start) / 2) + col.start;

            match step {
                SideStep::Left => col.end = middle,
                SideStep::Right => col.start = middle,
            }
        }
        debug_assert!(col.start == col.end - 1);

        (row.start, col.start)
    }
}

fn seat_id((row, col): (usize, usize)) -> usize {
    row * 8 + col
}

#[test]
fn validate_seat_number() {
    assert_eq!((44, 5), Seat::parse("FBFBBFFRLR").get_number());
    assert_eq!((70, 7), Seat::parse("BFFFBBFRRR").get_number());
    assert_eq!((14, 7), Seat::parse("FFFBBBFRRR").get_number());
    assert_eq!((102, 4), Seat::parse("BBFFBBFRLL").get_number());

    assert_eq!(357, seat_id(Seat::parse("FBFBBFFRLR").get_number()));
    assert_eq!(567, seat_id(Seat::parse("BFFFBBFRRR").get_number()));
    assert_eq!(119, seat_id(Seat::parse("FFFBBBFRRR").get_number()));
    assert_eq!(820, seat_id(Seat::parse("BBFFBBFRLL").get_number()));
}

#[derive(Debug)]
enum RowStep {
    Front,
    Back,
}

impl RowStep {
    pub fn new(b: u8) -> Self {
        match b {
            b'F' => Self::Front,
            b'B' => Self::Back,
            x => panic!("Unexpected RowStep {}", x as char),
        }
    }
}

#[derive(Debug)]
enum SideStep {
    Left,
    Right,
}

impl SideStep {
    pub fn new(b: u8) -> Self {
        match b {
            b'L' => Self::Left,
            b'R' => Self::Right,
            x => panic!("Unexpected SideStep {}", x as char),
        }
    }
}
