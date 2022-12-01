fn main() {
    match std::env::var("part").unwrap().as_str() {
        "part1" => println!("{}", std::io::read_to_string(std::fs::File::open("input.txt").unwrap()).unwrap().split("\n\n").map(|elf| elf.lines().map(|l| l.parse::<i32>().unwrap()).sum()).collect::<std::collections::BinaryHeap<i32>>().peek().unwrap()),
        _ => println!("{}", std::io::read_to_string(std::fs::File::open("input.txt").unwrap()).unwrap().split("\n\n").map(|elf| elf.lines().map(|l| l.parse::<i32>().unwrap()).sum()).collect::<std::collections::BinaryHeap<i32>>().iter().take(3).sum::<i32>())
    }
}