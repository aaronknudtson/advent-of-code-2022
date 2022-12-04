use std::num::ParseIntError;
use std::str::FromStr;

struct Range {
    lo: u32,
    hi: u32,
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spl: Vec<_> = s.split('-').collect();
        Ok(Self {
            lo: spl.get(0).unwrap().parse()?,
            hi: spl.get(1).unwrap().parse()?,
        })
    }
}

impl Range {
    fn is_contained_by(&self, other: &Range) -> bool {
        self.lo >= other.lo && self.hi <= other.hi
    }

    fn is_overlapped_by(&self, other: &Range) -> bool {
        !(self.lo > other.hi || other.lo > self.hi)
    }
}

fn main() {
    let input = std::fs::read_to_string("src/day04.txt").unwrap();

    let range_pairs: Vec<_> = input
        .lines()
        .map(|l| l.split(',').map(|s| Range::from_str(s).unwrap()).collect::<Vec<_>>())
        .collect();
    let num_fully_contained = range_pairs.iter().filter(|pair| {
        let p1 = &pair[0];
        let p2 = &pair[1];
        p1.is_contained_by(p2) || p2.is_contained_by(p1)
    }).count();
    println!("Part one: {}", num_fully_contained);

    let num_partially_contained = range_pairs.iter().filter(|pair| {
        let p1 = &pair[0];
        let p2 = &pair[1];
        p1.is_overlapped_by(p2)
    }).count();
    println!("Part two: {}", num_partially_contained);
}
