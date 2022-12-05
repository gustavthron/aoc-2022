fn parse_input() -> (Vec<Vec<u8>>, Vec<(usize, usize, usize)>) {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (input1, input2) = input.split_once("\n\n").unwrap();
    let mut input1_iter = input1.lines().rev();
    let mut stacks: Vec<Vec<u8>> = input1_iter
        .next()
        .unwrap()
        .split("   ")
        .map(|_| Vec::new())
        .collect();
    for line in input1_iter {
        for (i, c) in line.as_bytes().chunks(4).map(|chunk| chunk[1]).enumerate() {
            if c != b' ' {
                stacks[i].push(c);
            }
        }
    }
    let input2_splits = input2.split([' ', '\n']).collect::<Vec<&str>>();
    let instructions: Vec<(usize, usize, usize)> = input2_splits
        .chunks(6)
        .map(|chunk| {
            (
                chunk[1].parse::<usize>().unwrap(),
                chunk[3].parse::<usize>().unwrap(),
                chunk[5].parse::<usize>().unwrap(),
            )
        })
        .collect();
    return (stacks, instructions);
}

fn solve_part1() -> String {
    let (mut stacks, instructions) = parse_input();
    for (m, f, t) in instructions {
        for _ in 0..m {
            let c = stacks[f - 1].pop().unwrap();
            stacks[t - 1].push(c);
        }
    }
    let top_crates: Vec<u8> = stacks.iter().map(|stack| *stack.last().unwrap()).collect();
    return std::str::from_utf8(&top_crates).unwrap().to_string();
}

fn solve_part2() -> String {
    let (mut stacks, instructions) = parse_input();
    for (m, f, t) in instructions {
        let mut buf: Vec<u8> = Vec::new();
        for _ in 0..m {
            buf.push(stacks[f - 1].pop().unwrap());
        }
        for _ in 0..m {
            stacks[t - 1].push(buf.pop().unwrap());
        }
    }
    let top_crates: Vec<u8> = stacks.iter().map(|stack| *stack.last().unwrap()).collect();
    return std::str::from_utf8(&top_crates).unwrap().to_string();
}

fn main() {
    match std::env::var("part").unwrap().as_str() {
        "part1" => println!("{}", solve_part1()),
        _ => println!("{}", solve_part2()),
    }
}
