use aoc_2020::*;

fn main() {
    let input = input("day13.txt");
    let mut iter = input.into_iter();
    let timestamp: usize = iter.next().unwrap().parse().unwrap();
    let busses: Vec<Option<BusId>> = iter
        .next()
        .unwrap()
        .split(',')
        .map(|s| if s == "x" { None } else { Some(BusId::new(s)) })
        .collect();
    // let input = input_isize("dayxxx.txt");

    let start = std::time::Instant::now();
    println!("Part 1:");
    part1(timestamp, &busses);
    println!("Done in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2:");
    part2(timestamp, &busses);
    println!("Done in {:?}", start.elapsed());
}

fn part1(timestamp: usize, busses: &[Option<BusId>]) {
    let mut lowest_distance = usize::max_value();
    let mut lowest_bus = None;

    for bus in busses.iter().filter_map(|b| b.as_ref()) {
        let time_since_last_bus = timestamp % bus.0;
        let next_timestamp = timestamp - time_since_last_bus + bus.0;
        let time_until_next_bus = next_timestamp - timestamp;
        if time_until_next_bus < lowest_distance {
            lowest_distance = time_until_next_bus;
            lowest_bus = Some(bus);
        }
    }
    let lowest_bus = lowest_bus.unwrap();
    println!(
        "Next bus is {:?}, in {} minutes",
        lowest_bus, lowest_distance
    );
    println!("{}", lowest_bus.0 * lowest_distance);
}

fn part2(_timestamp: usize, busses: &[Option<BusId>]) {
    let first_bus = busses[0].unwrap();
    let mut timestamp = 100000000000000 + first_bus.0;

    let busses_with_indexes = busses
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|(idx, bus)| {
            if let Some(bus) = bus {
                Some((idx, bus))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    loop {
        // check if we're sequential yet
        let mut valid = true;
        for (idx, bus) in &busses_with_indexes {
            if (timestamp + idx) % bus.0 != 0 {
                valid = false;
                break;
            }
        }
        if !valid {
            timestamp = timestamp
                .checked_add(first_bus.0)
                .expect("timestamp overflow");
            continue;
        }
        println!("Timestamp is {}", timestamp);

        for (idx, bus) in busses.iter().enumerate() {
            if let Some(bus) = bus {
                println!(
                    "{:?} at index {} and timestamp {}",
                    bus,
                    idx,
                    timestamp + idx
                );
            }
        }
        break;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct BusId(usize);

impl BusId {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}
