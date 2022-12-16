use std::collections::HashSet;

fn get_n_unique(input: &Vec<char>, n: usize) -> usize {
    let mut windows = input.windows(n);
    let mut count = n;
    while let Some(elems) = windows.next() {
        let mut hs = HashSet::<&char>::new();
        if elems.iter().map(|v| hs.insert(v)).all(|v| v == true) {
            break;
        }
        count += 1;
    }
    return count;
}
fn main() {
    let input: Vec<_> = std::fs::read_to_string("src/day06.txt")
        .unwrap()
        .chars()
        .collect();
    println!("Part one: {}", get_n_unique(&input, 4));
    println!("Part two: {}", get_n_unique(&input, 14));
}
