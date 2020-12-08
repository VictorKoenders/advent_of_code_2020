use aoc_2020::*;

fn main() {
    let input = input("day8.txt");
    let input = input.iter().map(Instruction::parse).collect::<Vec<_>>();
    // let input = input_isize("dayxxx.txt");

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
    for i in 0..input.len() {
        if let Instruction::Nop(v) = input[i] {
            let mut new_input = input.to_vec();
            new_input[i] = Instruction::Jmp(v);
            let result = execute(&new_input);
            if !result.is_infinite_loop() {
                println!(
                    "Changing statement at {} from NOP to JMP fixed the infinite loop",
                    i
                );
                println!("Acc is {}", result.get_value());
            }
        }
        if let Instruction::Jmp(v) = input[i] {
            let mut new_input = input.to_vec();
            new_input[i] = Instruction::Nop(v);
            let result = execute(&new_input);
            if !result.is_infinite_loop() {
                println!(
                    "Changing statement at {} from JMP to NOP fixed the infinite loop",
                    i
                );
                println!("Acc is {}", result.get_value());
            }
        }
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
    pub fn parse(line: &String) -> Self {
        let mut split = line.split(' ');
        let instruction = split.next().unwrap();

        let value = split.next().unwrap();
        let value = value.strip_prefix('+').unwrap_or(value);
        let value: i32 = value.parse().unwrap();

        match instruction {
            "nop" => Self::Nop(value),
            "acc" => Self::Acc(value),
            "jmp" => Self::Jmp(value),
            x => panic!("Unknown instruction {:?}", x),
        }
    }
}
