fn main() {
    match std::env::var("part").unwrap().as_str() {
        "part1" => println!("{}", std::fs::read_to_string("input.txt").unwrap().lines().fold(0, |acc, line| acc + std::iter::repeat(['A', 'B', 'C']).take(3).flatten().enumerate().map(|(i, m)| (format!("{} {}", m, ['X', 'Y', 'Z'][(i%3+i/3+2)%3]), (i%3+i/3+2)%3+1+i/3*3)).collect::<std::collections::HashMap<String, usize>>()[line])),
        _ => println!("{}", std::fs::read_to_string("input.txt").unwrap().lines().fold(0, |acc, line| acc + std::iter::repeat(['A', 'B', 'C']).take(3).flatten().enumerate().map(|(i, m)| (format!("{} {}", m, ['X', 'Y', 'Z'][i/3]), (i%3+i/3+2)%3+1+i/3*3)).collect::<std::collections::HashMap<String, usize>>()[line])),
    }
}