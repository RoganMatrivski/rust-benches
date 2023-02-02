use iai::black_box;

fn split(str: &str) -> (&str, &str) {
    let mut split_str = str.split('|');
    let first = split_str.next().unwrap();
    let second = split_str.next().unwrap();

    (first, second)
}

fn split_at_find(str: &str) -> (&str, &str) {
    let split_pos = str.find('|').unwrap();
    let (first, second) = str.split_at(split_pos);

    (first, second)
}

fn split_at_position(str: &str) -> (&str, &str) {
    let split_pos = str.chars().position(|c| c == '|').unwrap();
    let (first, second) = str.split_at(split_pos);

    (first, second)
}

const INPUT_STR: &str = "asdasd|asddsa";

fn iai_split() -> (&'static str, &'static str) {
    split(black_box(INPUT_STR))
}

fn iai_split_at_find() -> (&'static str, &'static str) {
    split_at_find(black_box(INPUT_STR))
}

fn iai_split_at_position() -> (&'static str, &'static str) {
    split_at_position(black_box(INPUT_STR))
}

iai::main!(iai_split, iai_split_at_find, iai_split_at_position);
