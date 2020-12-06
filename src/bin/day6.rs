use aoc_2020::*;
use itertools::Itertools;

fn main() {
    let input = input("day6.txt");
    let input = Group::parse(&input);

    let start = std::time::Instant::now();
    println!("Part 1:");
    part1(&input);
    println!("Done in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2:");
    part2(&input);
    println!("Done in {:?}", start.elapsed());
}

fn part1(input: &[Group]) {
    let mut total_answers = 0;
    for group in input {
        let bytes = group.people.iter().flat_map(|p| p.answers).unique();
        let count = bytes.count();
        total_answers += count;
    }
    println!("Total answers: {}", total_answers);
}
fn part2(input: &[Group]) {
    let mut total_answers = 0;
    for group in input {
        let (first_person, other_people) = group.people.split_first().unwrap();

        if other_people.is_empty() {
            // if this is the only person in the group, all the answers are "yes"
            total_answers += first_person.answers.len();
        } else {
            // else we need to count each byte that's also available in everyone else's bytes
            let confirmed_answers = first_person
                .answers
                .iter()
                .filter(|answer| other_people.iter().all(|p| p.answers.contains(answer)))
                .count();

            total_answers += confirmed_answers;
        }
    }
    println!(
        "{} answers are 'yes' by everyone in the group",
        total_answers
    );
}

#[derive(Default)]
struct Group<'a> {
    people: Vec<Person<'a>>,
}

impl<'a> Group<'a> {
    pub fn parse(input: &'a [String]) -> Vec<Self> {
        let mut groups = Vec::new();
        let mut group = Group::default();
        for line in input {
            if line.is_empty() {
                groups.push(group);
                group = Default::default();
            } else {
                group.people.push(Person {
                    answers: line.as_bytes(),
                });
            }
        }

        if !group.people.is_empty() {
            groups.push(group);
        }

        groups
    }
}

struct Person<'a> {
    answers: &'a [u8],
}
