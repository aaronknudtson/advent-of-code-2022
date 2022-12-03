struct Rucksack {
    comp1: String,
    comp2: String,
    common: Option<char>,
}

impl Rucksack {
    fn new(rucksack: &str) -> Self {
        Self {
            comp1: String::from(&rucksack[0..(rucksack.len() / 2)]),
            comp2: String::from(&rucksack[(rucksack.len() / 2)..]),
            common: None,
        }
    }

    fn map_priority(c: char) -> u32 {
        if c.is_ascii_lowercase() {
            c as u32 - 'a' as u32 + 1
        } else {
            c as u32 - 'A' as u32 + 27
        }
    }

    fn find_doubled(&mut self) {
        for c in self.comp1.chars() {
            if self.comp2.contains(c) {
                self.common = Some(c);
                return;
            }
        }
    }

    fn priorities(rucksack: &str) -> u32 {
        let mut rucksack = Self::new(rucksack);
        rucksack.find_doubled();
        if let Some(doubled) = rucksack.common {
            return Rucksack::map_priority(doubled);
        }
        0
    }
}

fn three_elves_auth(all_rucksacks: String) -> u32 {
    let mut sum = 0;
    let rucksack_vec: Vec<_> = all_rucksacks.lines().collect();

    for three_sacks in rucksack_vec.chunks(3) {
        let (s1, s2, s3) = (three_sacks[0], three_sacks[1], three_sacks[2]);
        for c in s1.chars() {
            if s2.contains(c) && s3.contains(c) {
                sum += Rucksack::map_priority(c);
                break;
            }
        }
    }
    sum
}

fn main() {
    let rucksacks = std::fs::read_to_string("src/day03.txt").unwrap();

    println!(
        "Part one: {}",
        rucksacks
            .lines()
            .fold(0, |sum, line| sum + Rucksack::priorities(line))
    );

    println!("Part two: {}", three_elves_auth(rucksacks));
}

#[cfg(test)]
mod test {
    use crate::Rucksack;

    #[test]
    fn map_priority_works() {
        assert_eq!(Rucksack::map_priority('a'), 1);
        assert_eq!(Rucksack::map_priority('b'), 2);
        assert_eq!(Rucksack::map_priority('A'), 27);
        assert_eq!(Rucksack::map_priority('Z'), 52);
    }
}
