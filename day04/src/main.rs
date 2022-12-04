fn solve_part1() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let sections = input.split(['\n', ',', '-']).map(|split| split.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let pairs = sections.chunks(4).map(|chunk| ((chunk[0], chunk[1]), (chunk[2], chunk[3]))).collect::<Vec<((i32, i32),(i32, i32))>>(); //.next_chunk(4).chunks(4).map(|chunk| ((chunk[0]..=chunk[1]).collect::<HashSet<i32>>(), (chunk[2]..=chunk[3]).collect::<HashSet<i32>>())).collect::<Vec<(HashSet<i32>, HashSet<i32>)>>();
    let mut count = 0;
    for (elf1, elf2) in pairs {
        if ((elf1.0 - elf2.0).signum() + (elf1.1 - elf2.1).signum()).abs() < 2 {
            count += 1;
        }
    }
    return count;
}

fn solve_part2() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let sections = input.split(['\n', ',', '-']).map(|split| split.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let pairs = sections.chunks(4).map(|chunk| ((chunk[0], chunk[1]), (chunk[2], chunk[3]))).collect::<Vec<((i32, i32),(i32, i32))>>(); //.next_chunk(4).chunks(4).map(|chunk| ((chunk[0]..=chunk[1]).collect::<HashSet<i32>>(), (chunk[2]..=chunk[3]).collect::<HashSet<i32>>())).collect::<Vec<(HashSet<i32>, HashSet<i32>)>>();
    let mut count = 0;
    for (elf1, elf2) in pairs {
        if ((elf1.0 - elf2.1).signum() + (elf1.1 - elf2.0).signum()).abs() < 2 {
            count += 1;
        }
    }
    return count;
}

fn main() {
    match std::env::var("part").unwrap().as_str() {
        "part1" => println!("{}", solve_part1()),
        _ => println!("{}", solve_part2()),
    }
}