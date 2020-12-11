use aoc_2020::*;

fn main() {
    let mut input = input_isize("day10.txt");
    input.sort_unstable();

    let start = std::time::Instant::now();
    println!("Part 1:");
    part1(&input);
    println!("Done in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2:");
    part2(&input);
    println!("Done in {:?}", start.elapsed());
}

fn part1(input: &[isize]) {
    let mut calc_jolts = vec![0isize; 4];
    // from 0 to the first entry
    calc_jolts[input[0] as usize] += 1;

    for (current, next) in input.iter().copied().zip(input.iter().copied().skip(1)) {
        let diff = next - current;
        if diff > 3 || diff <= 0 {
            panic!("Unexpected diff: {} to {} is {}", current, next, diff);
        }
        calc_jolts[diff as usize] += 1;
    }
    // last entry + 3
    calc_jolts[3] += 1;

    println!("Result: {}", calc_jolts[1] * calc_jolts[3]);
}

fn part2(_input: &[isize]) {}
