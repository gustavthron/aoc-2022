fn solve_part1() -> u32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let rucksacks: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect();
    let mut res: u32 = 0;
    for (compartment1, compartment2) in rucksacks {
        for item in compartment1.as_bytes() {
            if compartment2.as_bytes().contains(item) {
                res += if *item >= b'a' {
                    u32::from(*item - b'a' + 1)
                } else {
                    u32::from(*item - b'A' + 27)
                };
                break;
            }
        }
    }
    return res;
}

fn solve_part2() -> u32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let rucksacks = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();
    let groups: Vec<&[&[u8]]> = rucksacks.chunks(3).collect();
    let mut res: u32 = 0;
    for group in groups {
        for item in group[0] {
            if group[1].contains(item) && group[2].contains(item) {
                res += if *item >= b'a' {
                    u32::from(*item - b'a' + 1)
                } else {
                    u32::from(*item - b'A' + 27)
                };
                break;
            }
        }
    }
    return res;
}

fn main() {
    match std::env::var("part").unwrap().as_str() {
        "part1" => println!("{}", solve_part1()),
        _ => println!("{}", solve_part2()),
    }
}
