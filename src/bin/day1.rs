use aoc_2020::*;

fn main() {
    let input = input_isize("day1.txt");
    
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
    for (i, n) in input.iter().enumerate() {
        for n2 in input.iter().skip(i) {
            if n + n2 == 2020 {
                println!("{} + {} = 2020", n, n2);
                println!("{} x {} = {}", n, n2, n * n2);
            }
        }
    }
}

fn part2(input: &[isize]) {
    for (i, n) in input.iter().enumerate() {
        for (j, n2) in input.iter().enumerate().skip(i) {
            for n3 in input.iter().skip(j) {
                if n + n2 + n3 == 2020 {
                    println!("{} + {} + {} = 2020", n, n2, n3);
                    println!("{} x {} x {} = {}", n, n2, n3, n * n2 * n3);
                }
            }
        }
    }
}

