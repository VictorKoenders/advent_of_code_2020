pub fn input(file: &str) -> Vec<String> {
    use std::io::Read;
    let mut file = std::fs::File::open(format!("input/{}", file)).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content.split_whitespace().map(|s| s.to_owned()).collect()
}
pub fn input_isize(file: &str) -> Vec<isize> {
    let input = input(file);
    input.into_iter().map(|i| i.parse().unwrap()).collect()
}
