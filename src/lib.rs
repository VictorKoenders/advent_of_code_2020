fn contents(file: &str) -> String {
    use std::io::Read;
    let mut file = std::fs::File::open(format!("input/{}", file)).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
}

pub fn input(file: &str) -> Vec<String> {
    contents(file).lines().map(|s| s.to_owned()).collect()
}
pub fn input_isize(file: &str) -> Vec<isize> {
    contents(file)
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
}
