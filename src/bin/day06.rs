use std::collections::HashSet;

fn main() {
    let input: Vec<_> = std::fs::read_to_string("src/day06.txt")
        .unwrap()
        .chars()
        .collect();
    let mut windows = input.windows(4);
    let mut count = 4;
    while let Some(elems) = windows.next() {
        let mut hs = HashSet::<&char>::new();
        if elems.iter().map(|v| hs.insert(v)).all(|v| v == true) {
            println!("{:?}", elems);
            println!("Part one: {}", count);
            break;
        }
        count += 1;
    }
}
