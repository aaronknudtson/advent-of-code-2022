fn scenic_score(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> usize {
    let len = grid.len();
    if i == 0 || j == 0 || i == len - 1 || j == len - 1 {
        return 0;
    }

    let height = grid[i][j];
    // left
    let mut run = 0;
    for t in (0..j).rev() {
        run += 1;
        if grid[i][t] >= height {
            break;
        }
    }
    let l = run;
    if l != run {
        println!("l: {}, run: {}", l, run);
    }
    // right
    run = 0;
    for t in j + 1..len {
        run += 1;
        if grid[i][t] >= height {
            break;
        }
    }
    let r = run;
    // up
    run = 0;
    for t in (0..i).rev() {
        run += 1;
        if grid[t][j] >= height {
            break;
        }
    }
    let u = run;
    // down
    run = 0;
    for t in i + 1..len {
        run += 1;
        if grid[t][j] >= height {
            break;
        }
    }
    let d = run;
    l * r * u * d
}

fn is_visible(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    if i == grid.len() - 1 || j == grid[i].len() - 1 || i == 0 || j == 0 {
        return true;
    }
    let height = grid[i][j];
    let l = *grid[i][..j].iter().max().unwrap() < height;
    let r = *grid[i][j + 1..].iter().max().unwrap() < height;
    let u = grid[..i].iter().map(|v| v[j]).max().unwrap() < height;
    let d = grid[i + 1..].iter().map(|v| v[j]).max().unwrap() < height;
    return u || d || l || r;
}

fn main() {
    let input = std::fs::read_to_string("src/day08.txt").unwrap();
    let grid: Vec<_> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    let len = grid.len();
    let mut scores = Vec::new();
    let mut count = 0;
    for i in 0..len {
        for j in 0..len {
            let visible = is_visible(&grid, i, j);
            scores.push(scenic_score(&grid, i, j));
            if visible {
                count += 1;
            }
        }
    }
    println!("Part one: {count}");
    println!("Part two: {:?}", scores.iter().max().unwrap());
}
