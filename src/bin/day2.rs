use aoc_2020::*;

#[derive(Debug)]
struct PasswordLine<'a> {
    min: usize,
    max: usize,
    needle: char,
    haystack: &'a str
}

impl<'a> PasswordLine<'a>  {
    pub fn parse(line: &'a str) -> Self {
        let mut parts = line.split(' ');
        let minmax = parts.next().unwrap();
        let needle = parts.next().unwrap();
        let haystack = parts.next().unwrap();

        let mut minmax = minmax.split('-');
        let min = minmax.next().unwrap().parse().unwrap();
        let max = minmax.next().unwrap().parse().unwrap();

        let needle = needle.chars().next().unwrap();

        Self {
            min,
            max,
            needle,
            haystack,
        }
    }

    pub fn count_is_valid(&self) -> bool {
        let count = self.haystack.chars().filter(|c| c == &self.needle).count();
        count >= self.min && count <= self.max
    }

    pub fn position_is_valid(&self) -> bool {
        // positions get -1'd because the question is 1-indexed but rust is 0-indexed
        let first = self.haystack.chars().nth(self.min - 1).unwrap();
        let second = self.haystack.chars().nth(self.max - 1).unwrap();

        (first == self.needle && second != self.needle) ||
            (second == self.needle && first != self.needle)
    }
}


fn main() {
    let input = input("day2.txt");
    let input: Vec<_> = input.iter().map(|s| PasswordLine::parse(s)).collect();

    let start = std::time::Instant::now();
    println!("Part 1:");
    part1(&input);
    println!("Done in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2:");
    part2(&input);
    println!("Done in {:?}", start.elapsed());
}

fn part1(input: &[PasswordLine]) {
    let mut count = 0;
    for line in input {
        if line.count_is_valid() {
            count += 1;
        }
    }
    println!("Found {:?} valid passwords", count);
}
fn part2(input: &[PasswordLine]) {
    let mut count = 0;
    for line in input {
        if line.position_is_valid() {
            count += 1;
        }
    }
    println!("Found {:?} valid passwords", count);

}
