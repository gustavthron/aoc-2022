fn main() {
    match std::env::var("part").unwrap().as_str() {
        "part1" => println!("{}", std::fs::read_to_string("input.txt").unwrap().split(['\n', ',', '-']).map(|split| split.parse::<i32>().unwrap()).collect::<Vec<i32>>().chunks(4).filter(|chunk| ((chunk[0] - chunk[2]).signum() + (chunk[1] - chunk[3]).signum()).abs() < 2).count()),
        _ => println!("{}", std::fs::read_to_string("input.txt").unwrap().split(['\n', ',', '-']).map(|split| split.parse::<i32>().unwrap()).collect::<Vec<i32>>().chunks(4).filter(|chunk| ((chunk[0] - chunk[3]).signum() + (chunk[1] - chunk[2]).signum()).abs() < 2).count()),
    }
}