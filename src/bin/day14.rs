use aoc_2020::*;

fn main() {
    let input = input("day14.txt");
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
    let mut values = Vec::new();
    let mut clear_mask = 0;
    let mut set_mask = 0;
    for instruction in input {
        match instruction {
            Instruction::SetMask { clear, set } => {
                clear_mask = *clear;
                set_mask = *set;
            }
            Instruction::SetValue { address, value } => {
                let value = value | set_mask;
                let value = value & !clear_mask;

                values.resize_with(address + 1, || 0);
                values[*address] = value;
            }
        }
    }
    println!("{}", values.into_iter().sum::<u64>());
}

fn part2(_input: &[Instruction]) {}

#[derive(Debug)]
enum Instruction {
    SetMask { clear: u64, set: u64 },
    SetValue { address: usize, value: u64 },
}

impl Instruction {
    pub fn parse(input: &[String]) -> Vec<Self> {
        let mut result = Vec::new();

        for line in input {
            let mut split = line.split(" = ");
            let left = split.next().unwrap();
            let right = split.next().unwrap();

            if left == "mask" {
                let mut clear = 0;
                let mut set = 0;
                for (idx, c) in right.bytes().rev().enumerate() {
                    if c == b'0' {
                        clear |= 1 << idx;
                    } else if c == b'1' {
                        set |= 1 << idx;
                    }
                }
                result.push(Instruction::SetMask { clear, set });
            } else {
                let address = &left[4..left.len() - 1];
                result.push(Instruction::SetValue {
                    address: address.parse().unwrap(),
                    value: right.parse().unwrap(),
                });
            }
        }

        result
    }
}
