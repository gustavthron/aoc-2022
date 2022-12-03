fn main() {
    match std::env::var("part").unwrap().as_str() {
        "part1" => println!("{}", std::fs::read_to_string("input.txt").unwrap().split("\n\n").fold(0, |accum, elf| std::cmp::max(accum, elf.lines().map(|l| l.parse::<u32>().unwrap()).sum()))),
        _ => println!("{}", std::fs::read_to_string("input.txt").unwrap().split("\n\n").map(|elf| elf.lines().map(|l| l.parse::<u32>().unwrap()).sum()).collect::<std::collections::BinaryHeap<u32>>().iter().take(3).sum::<u32>()),
    }
}