fn main() {
    match std::env::var("part").unwrap().as_str() {
        "part1" => println!("{}", std::fs::read_to_string("input.txt").unwrap().lines().map(|line| line.as_bytes().split_at(line.len() / 2)).map(|(comp1, comp2)| u32::from((comp1.to_vec().into_iter().collect::<std::collections::HashSet<u8>>().intersection(&comp2.to_vec().into_iter().collect::<std::collections::HashSet<u8>>()).collect::<Vec<&u8>>()[0] - b'A' + 27)%(b'a' - b'A' + 26))).sum::<u32>()),
        _ => println!("{}", std::fs::read_to_string("input.txt").unwrap().lines().map(|line| line.as_bytes()).collect::<Vec<&[u8]>>().chunks(3).map(|group| u32::from((group[0].to_vec().into_iter().collect::<std::collections::HashSet<u8>>().intersection(&(group[1].to_vec().into_iter().collect::<std::collections::HashSet<u8>>().intersection(&group[2].to_vec().into_iter().collect::<std::collections::HashSet<u8>>()).copied().collect::<std::collections::HashSet<u8>>())).collect::<Vec<&u8>>()[0] - b'A' + 27)%(b'a' - b'A' + 26))).sum::<u32>()),
    }
}