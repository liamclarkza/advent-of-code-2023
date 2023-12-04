fn main() {
    let input = include_str!("../input.txt");

    let sum: i32 = input.lines().map(|line| {
        let digits: Vec<_> = line.chars().filter(|c| c.is_digit(10)).collect();
        format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse::<i32>().unwrap()
    }).sum();

    println!("{}", sum);
}