use std::collections::{HashMap, HashSet};

use aoc_2020::*;

fn main() {
    let input = input("day7.txt");
    let input = Bag::parse(&input);

    let start = std::time::Instant::now();
    println!("Part 1:");
    part1(&input);
    println!("Done in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2:");
    part2(&input);
    println!("Done in {:?}", start.elapsed());
}

fn part1(input: &HashMap<&str, Bag>) {
    let query = "shiny gold";

    let mut parents = HashSet::new();
    let mut query_list = vec![query];

    while let Some(query) = query_list.pop() {
        for bag in input.values().filter(|b| b.contains_child_color(query)) {
            if !parents.contains(bag.color) {
                parents.insert(bag.color);
                query_list.push(bag.color);
            }
        }
    }

    println!("Found {} parents", parents.len());
}

fn part2(input: &HashMap<&str, Bag>) {
    let query = "shiny gold";

    fn count_child_bags(input: &HashMap<&str, Bag>, query: &str) -> usize {
        let bag = input.get(query).unwrap();
        let mut sum = 1;
        for (count, name) in &bag.children {
            sum += count * count_child_bags(input, name);
        }
        sum
    }

    let child_bags = count_child_bags(input, query);
    // we don't want to count the parent bag (our query), so -1
    let child_bags = child_bags - 1;
    println!("Found {} bags", child_bags);
}

#[derive(Debug)]
struct Bag<'a> {
    color: &'a str,
    children: Vec<(usize, &'a str)>,
}

impl<'a> Bag<'a> {
    pub fn parse(input: &'a [String]) -> HashMap<&'a str, Bag<'a>> {
        let mut result = HashMap::new();

        for line in input {
            // format is "bright white bag contains 1 bright white bag, 2 bright white bags"
            // first split to "bright white" and "1 bright ..."
            let mut split = line.split(" bags contain ");
            let color = split.next().unwrap();

            let right = split.next().unwrap();

            // Then split the children
            let children: Vec<(usize, &str)> = right
                .split(", ")
                .filter_map(|part| {
                    // Remove training dot
                    let part = part.strip_suffix(".").unwrap_or(part);
                    // turn "bags" into "bag"
                    let part = part.strip_suffix("s").unwrap_or(part);
                    // split trailing "bag"
                    let part = part.strip_suffix(" bag").unwrap();
                    // split "2 bright white" into "2" and "bright white"
                    let mut split = part.splitn(2, ' ');
                    let n = split.next().unwrap();

                    // We could have "no bright white" at this point, return nothing
                    if n == "no" {
                        None
                    } else {
                        let n = n.parse().unwrap();
                        let color = split.next().unwrap();

                        Some((n, color))
                    }
                })
                .collect();

            result.insert(color, Bag { color, children });
        }

        result
    }

    pub fn contains_child_color(&self, color: &str) -> bool {
        self.children
            .iter()
            .any(|(_, child_color)| child_color == &color)
    }
}
