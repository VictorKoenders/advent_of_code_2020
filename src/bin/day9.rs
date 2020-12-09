use aoc_2020::*;

fn main() {
    let input = input_isize("day9.txt");

    let start = std::time::Instant::now();
    println!("Part 1:");
    part1(&input);
    println!("Done in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2:");
    part2(&input);
    println!("Done in {:?}", start.elapsed());
}

fn part1(input: &[isize]) -> isize {
    const PREAMBLE_SIZE: usize = 25;
    for (idx, num) in input.iter().enumerate().skip(PREAMBLE_SIZE) {
        let preamble = &input[idx - PREAMBLE_SIZE..idx];
        if !find_preamble_sum(preamble, *num) {
            println!("Could not find preamble sum for {}", num);
            return *num;
        }
    }

    panic!("Could not find unique num")
}

fn part2(input: &[isize]) {
    let number_to_find = part1(input);
    for (idx, first_num) in input.iter().enumerate() {
        let mut sum = *first_num;
        let mut last_index = idx + 1;
        while sum < number_to_find {
            sum += input[last_index];
            last_index += 1;
        }

        if sum == number_to_find {
            // println!("Found the start of the sum range at range {} .. {}", idx, last_index);
            let range = &input[idx..last_index];
            // println!("The numbers are {:?}", range);
            let max = range.iter().max().unwrap();
            let min = range.iter().min().unwrap();
            // println!("Sum is {} + {} = {}", min, max, min + max);
            println!("{}", min + max);
            break;
        }
    }
}

fn find_preamble_sum(input: &[isize], needle: isize) -> bool {
    for (idx, a) in input.iter().enumerate() {
        for b in input.iter().skip(idx) {
            if a + b == needle {
                return true;
            }
        }
    }
    false
}
