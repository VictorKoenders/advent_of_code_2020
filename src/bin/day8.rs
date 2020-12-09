use aoc_2020::*;

fn main() {
    let input = input("day8.txt");
    let input = input
        .iter()
        .map(|s| Instruction::parse(s))
        .collect::<Vec<_>>();

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
    let result = execute(input);
    println!("Acc is {}", result.get_value());
}
fn part2(input: &[Instruction]) {
    let mut instruction = input.to_vec();
    for i in 0..instruction.len() {
        let new_value = match instruction[i] {
            Instruction::Nop(v) => Instruction::Jmp(v),
            Instruction::Jmp(v) => Instruction::Nop(v),
            _ => continue,
        };
        let old_value = std::mem::replace(&mut instruction[i], new_value);

        let result = execute(&instruction);
        if !result.is_infinite_loop() {
            println!(
                "Acc is {}, flipped instruction is at {}",
                result.get_value(),
                i
            );
            break;
        }
        instruction[i] = old_value;
    }
}

fn execute(instructions: &[Instruction]) -> ExecuteResult {
    let mut pc = 0;
    let mut acc = 0;

    // TODO: Replace with bitvec?
    let mut visited = vec![false; instructions.len()];

    loop {
        if pc == instructions.len() {
            return ExecuteResult::Finished(acc);
        }
        if visited[pc] {
            return ExecuteResult::Loop(acc);
        }
        visited[pc] = true;
        let instruction = instructions[pc];
        match instruction {
            Instruction::Acc(val) => {
                acc += val;
                pc += 1;
            }
            Instruction::Jmp(rel_addr) => {
                if rel_addr < 0 {
                    pc -= rel_addr.abs() as usize;
                } else {
                    pc += rel_addr as usize;
                }
            }
            Instruction::Nop(_) => {
                pc += 1;
            }
        }
    }
}

pub enum ExecuteResult {
    Loop(i32),
    Finished(i32),
}

impl ExecuteResult {
    fn get_value(self) -> i32 {
        match self {
            ExecuteResult::Loop(v) => v,
            ExecuteResult::Finished(v) => v,
        }
    }

    fn is_infinite_loop(&self) -> bool {
        matches!(self, ExecuteResult::Loop(_))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Instruction {
    pub fn parse(line: &str) -> Self {
        let mut split = line.split(' ');
        let instruction = split.next().unwrap();

        let value = split.next().unwrap();
        let value: i32 = value.parse().unwrap();

        match instruction {
            "nop" => Self::Nop(value),
            "acc" => Self::Acc(value),
            "jmp" => Self::Jmp(value),
            x => panic!("Unknown instruction {:?}", x),
        }
    }
}
