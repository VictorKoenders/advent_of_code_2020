use aoc_2020::*;

fn main() {
    let input = input_isize("day1.txt");
    println!("Part 1:");
    for (i, n) in input.iter().enumerate() {
        for n2 in input.iter().skip(i) {
            if n + n2 == 2020 {
                println!("{} + {} = 2020", n, n2);
                println!("{} x {} = {}", n, n2, n * n2);
            }
        }
    }
    println!("Part 2:");
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
