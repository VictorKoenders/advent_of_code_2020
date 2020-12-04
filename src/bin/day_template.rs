use aoc_2020::*;

fn main() {
    let input = input("dayxxx.txt");
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

fn part1(input: &[String]) {}
fn part2(input: &[String]) {}
