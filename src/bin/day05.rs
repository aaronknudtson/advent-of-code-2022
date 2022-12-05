use onig::Regex;

fn apply_direction_part_one(d: &Vec<u8>, stacks: &mut Vec<Vec<char>>) {
    let num_move = d[0];
    let from = d[1] - 1;
    let to = d[2] - 1;
    for _ in 0..num_move {
        if let Some(v) = stacks.get_mut(from as usize).unwrap().pop() {
            stacks.get_mut(to as usize).unwrap().push(v);
        }
    }
}

fn apply_direction_part_two(d: &Vec<u8>, stacks: &mut Vec<Vec<char>>) {
    let num_move = d[0];
    let from = d[1] - 1;
    let to = d[2] - 1;
    let mut tmp = Vec::new();
    for _ in 0..num_move {
        if let Some(v) = stacks.get_mut(from as usize).unwrap().pop() {
            tmp.push(v);
        }
    }
    for _ in 0..num_move {
        if let Some(v) = tmp.pop() {
            stacks.get_mut(to as usize).unwrap().push(v);
        }
    }
}

fn parse_boxes(boxes: Vec<&str>) -> Vec<Vec<char>> {
    let mut stacks = Vec::with_capacity(9);
    for _ in 0..9 {
        stacks.push(Vec::new());
    }
    let blen = boxes.len();
    for i in 0..blen {
        let &working_str = boxes.get(blen - i - 1).unwrap();
        for j in 0..9 {
            let c = working_str.chars().nth(j * 4 + 1).unwrap();
            if c.is_uppercase() {
                stacks.get_mut(j).unwrap().push(c);
            }
        }
    }
    stacks
}

fn main() {
    let input = std::fs::read_to_string("src/day05.txt").unwrap();
    let dir_pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut stacks_p1: Vec<_> = parse_boxes(input.lines().take(9).collect());
    input
        .lines()
        .skip(stacks_p1.len() + 1)
        .map(|d| {
            dir_pattern
                .captures(d)
                .unwrap()
                .iter()
                .skip(1)
                .map(|dir| dir.unwrap().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .for_each(|d| apply_direction_part_one(&d, &mut stacks_p1));
    println!(
        "Part one: {:?}",
        stacks_p1
            .iter()
            .map(|s| s.last().unwrap())
            .collect::<String>()
    );

    let mut stacks_p2: Vec<_> = parse_boxes(input.lines().take(9).collect());
    input
        .lines()
        .skip(stacks_p2.len() + 1)
        .map(|d| {
            dir_pattern
                .captures(d)
                .unwrap()
                .iter()
                .skip(1)
                .map(|dir| dir.unwrap().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .for_each(|d| apply_direction_part_two(&d, &mut stacks_p2));
    println!(
        "Part two: {:?}",
        stacks_p2
            .iter()
            .map(|s| s.last().unwrap())
            .collect::<String>()
    )
}
