const DIGIT_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("../input.txt").to_string();
    let total = input
        .lines()
        .map(|line| (find_first(line, false).unwrap() * 10 + find_first(line, true).unwrap()))
        .sum::<u32>();
    println!("{}", total);
}

fn find_first(line: &str, reverse: bool) -> Option<u32> {
    let char_indices = line.char_indices();
    let iter: Box<dyn Iterator<Item = (usize, char)>> = if reverse {
        Box::new(char_indices.rev())
    } else {
        Box::new(char_indices)
    };
    for (i, c) in iter {
        if c.is_ascii_digit() {
            return Some(c.to_digit(10).unwrap() as u32);
        } else {
            for (d, w) in DIGIT_WORDS.iter().enumerate() {
                if line[i..].starts_with(w) {
                    return Some(d as u32);
                }
            }
        }
    }
    None
}
