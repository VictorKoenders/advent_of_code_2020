use lazy_static::lazy_static;
use regex::Regex;

use aoc_2020::*;

fn main() {
    let input = input("day4.txt");
    let passports = parse_passports(&input);

    let start = std::time::Instant::now();
    println!("Part 1:");
    part1(&passports);
    println!("Done in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2:");
    part2(&passports);
    println!("Done in {:?}", start.elapsed());
}

fn part1(input: &[Passport]) {
    println!(
        "{} passports are valid (excluding cid)",
        input.iter().filter(|p| p.all_but_cid_filled()).count()
    );
}
fn part2(input: &[Passport]) {
    println!(
        "{} passports are valid (excluding cid)",
        input.iter().filter(|p| p.all_fields_valid()).count()
    );
}

#[derive(Default, Debug)]
struct Passport<'a> {
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    eyr: Option<&'a str>,
    hcl: Option<&'a str>,
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    hgt: Option<&'a str>,

    cid: Option<&'a str>,
}

lazy_static! {
    static ref HEX_REGEX: Regex = Regex::new(r"^\#[a-f0-9]{6}$").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

impl Passport<'_> {
    pub fn all_but_cid_filled(&self) -> bool {
        self.ecl.is_some()
            && self.pid.is_some()
            && self.eyr.is_some()
            && self.hcl.is_some()
            && self.byr.is_some()
            && self.iyr.is_some()
            && self.hgt.is_some()
    }

    pub fn all_fields_valid(&self) -> bool {
        validate_between(self.byr, 1920, 2002)
            && validate_between(self.iyr, 2010, 2020)
            && validate_between(self.eyr, 2020, 2030)
            && height_value(self.hgt)
            && validate_regex(self.hcl, &HEX_REGEX)
            && any_of(self.ecl, &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"])
            && validate_regex(self.pid, &PID_REGEX)
    }

    pub fn all_empty(&self) -> bool {
        self.ecl.is_none()
            && self.pid.is_none()
            && self.eyr.is_none()
            && self.hcl.is_none()
            && self.byr.is_none()
            && self.iyr.is_none()
            && self.hgt.is_none()
            && self.cid.is_none()
    }
}

#[test]
fn validate_validators() {}

fn validate_between(v: Option<&str>, min: usize, max: usize) -> bool {
    match v.and_then(|v| v.parse::<usize>().ok()) {
        Some(v) => v >= min && v <= max,
        None => false,
    }
}

fn height_value(v: Option<&str>) -> bool {
    match v {
        Some(v) => {
            if let Some(cm) = v.strip_suffix("cm") {
                validate_between(Some(cm), 150, 193)
            } else if let Some(inch) = v.strip_suffix("in") {
                validate_between(Some(inch), 59, 76)
            } else {
                false
            }
        }
        None => false,
    }
}

fn any_of(v: Option<&str>, possible_values: &[&str]) -> bool {
    matches!(v, Some(v) if possible_values.contains(&v))
}

fn validate_regex(v: Option<&str>, re: &'static Regex) -> bool {
    match v {
        Some(v) => re.is_match(v),
        None => false,
    }
}

fn parse_passports(input: &[String]) -> Vec<Passport> {
    let mut result = Vec::new();
    let mut current_passport = Passport::default();

    for line in input {
        if line.is_empty() {
            result.push(current_passport);
            current_passport = Passport::default();
        }
        for chunk in line.split(' ') {
            let mut parts = chunk.split(':');
            if let (Some(name), Some(value)) = (parts.next(), parts.next()) {
                match name {
                    "ecl" => current_passport.ecl = Some(value),
                    "pid" => current_passport.pid = Some(value),
                    "eyr" => current_passport.eyr = Some(value),
                    "hcl" => current_passport.hcl = Some(value),
                    "byr" => current_passport.byr = Some(value),
                    "iyr" => current_passport.iyr = Some(value),
                    "hgt" => current_passport.hgt = Some(value),
                    "cid" => current_passport.cid = Some(value),
                    x => panic!("Unknown type: {:?}", x),
                }
            }
        }
    }

    if !current_passport.all_empty() {
        result.push(current_passport);
    }

    result
}
