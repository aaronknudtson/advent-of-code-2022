struct Rucksack {
    comp1: String,
    comp2: String,
}

impl Rucksack {
    fn new(rucksack: &str) -> Self {
        Self {
            comp1: String::from(&rucksack[0..(rucksack.len() / 2)]),
            comp2: String::from(&rucksack[(rucksack.len() / 2)..]),
        }
    }

    fn map_priority(c: char) -> u32 {
        if c.is_ascii_lowercase() {
            c as u32 - 'a' as u32 + 1
        } else {
            c as u32 - 'A' as u32 + 27
        }
    }

    fn find_doubled(&self) -> Option<char> {
        for c in self.comp1.chars() {
            if self.comp2.contains(c) {
                return Some(c)
            }
        }
        return None
    }

    fn priorities(rucksack: &str) -> u32 {
        let rucksack = Self::new(rucksack);
        if let Some(doubled) = rucksack.find_doubled() {
            return Rucksack::map_priority(doubled)
        }
        return 0
    }
}

fn main() {
    let rucksacks = std::fs::read_to_string("src/day03.txt").unwrap();

    println!(
        "Part one: {}",
        rucksacks
            .lines()
            .fold(0, |sum, line| sum + Rucksack::priorities(line))
    )
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
