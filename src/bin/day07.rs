use std::{cell::RefCell, rc::Rc, str::Lines};

struct File {
    name: String,
    size: u32,
}
type Fil = Rc<RefCell<File>>;

struct Directory {
    path: String,
    size: u32,
    parent: Option<Dir>,
    directories: Vec<Dir>,
    files: Vec<Fil>,
}
type Dir = Rc<RefCell<Directory>>;
enum Command {
    Cd(String),
    Ls,
    Output(String),
}
fn parse(lines: &mut Lines, root: Dir, curr: Dir) {
    if let Some(line) = lines.next() {
        let cmd = parse_line(&line);
        if let Some(new_curr) = parse_command(&cmd, Rc::clone(&root), Rc::clone(&curr)) {
            parse(lines, Rc::clone(&root), Rc::clone(&new_curr));
        } else {
            parse(lines, Rc::clone(&root), Rc::clone(&curr));
        }
    }
}

fn parse_line(line: &str) -> Command {
    let elems: Vec<&str> = line.split_whitespace().collect();
    match elems[..] {
        ["$", "ls"] => Command::Ls,
        ["$", "cd", v] => Command::Cd(v.to_string()),
        _ => Command::Output(line.to_string()),
    }
}

fn parse_command(cmd: &Command, root: Dir, curr: Dir) -> Option<Dir> {
    match cmd {
        Command::Output(v) => {
            parse_output(v.as_str(), curr);
            None
        }
        Command::Cd(dir) => change_dir(dir.as_str(), root, curr),
        _ => None,
    }
}

fn parse_output(line: &str, curr: Dir) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let mut mut_curr = curr.as_ref().borrow_mut();
    match parts[..] {
        ["dir", path] => mut_curr.directories.push(Rc::new(RefCell::new(Directory {
            size: 0,
            path: path.to_string(),
            files: Vec::new(),
            directories: Vec::new(),
            parent: Some(Rc::clone(&curr)),
        }))),
        [size_str, name] => {
            let size = size_str.parse::<u32>().unwrap();
            mut_curr.files.push(Rc::new(RefCell::new(File {
                size,
                name: name.to_string(),
            })));
            mut_curr.size += size;
        }
        _ => (),
    }
}

fn change_dir(path: &str, root: Dir, curr: Dir) -> Option<Dir> {
    let new_dir = match path {
        ".." => Rc::clone(&curr.borrow().parent.as_ref().unwrap()),
        "/" => root,
        name => find_dir(curr, &name),
    };
    Some(Rc::clone(&new_dir))
}

fn find_dir(curr: Dir, path: &str) -> Dir {
    let my_curr = &*curr.borrow();
    for elem in my_curr.directories.iter() {
        let dir = &*elem.borrow();
        if dir.path == path {
            return Rc::clone(elem);
        }
    }

    println!("Don't think I should be here");
    // let mut mut_curr = curr.as_ref().borrow_mut();
    // mut_curr.directories.push()
    return Rc::new(RefCell::new(Directory {
        size: 0,
        path: path.to_string(),
        directories: Vec::new(),
        files: Vec::new(),
        parent: Some(Rc::clone(&curr)),
    }));
}

impl Directory {
    fn parse_term_output(input: String) -> Dir {
        let root = Rc::new(RefCell::new(Directory {
            size: 0,
            path: "/".to_owned(),
            parent: None,
            directories: Vec::new(),
            files: Vec::new(),
        }));
        let mut lines = input.lines();

        // parse
        parse(&mut lines, Rc::clone(&root), Rc::clone(&root));

        return root;
    }
}

fn get_sizes(root: Dir, sum: u32, sums: Rc<RefCell<Vec<u32>>>) -> u32 {
    let curr = &*root.borrow();

    // base case
    if curr.directories.len() == 0 {
        let dirsum = curr.files.iter().map(|v| v.borrow().size).sum::<u32>();
        if dirsum <= 100000 {
            let mut this = sums.as_ref().borrow_mut();
            this.push(dirsum);
        }
        return dirsum;
    }

    // recurse
    let mut sub_dirs_sum = 0;
    for dirs in curr.directories.iter() {
        sub_dirs_sum += get_sizes(Rc::clone(&dirs), sum, Rc::clone(&sums));
    }

    // post get sum
    let curr_dirsum = curr.files.iter().map(|v| v.borrow().size).sum::<u32>();
    let dirsum = { sub_dirs_sum + curr_dirsum };
    if dirsum <= 100000 {
        let mut this = sums.as_ref().borrow_mut();
        this.push(dirsum);
    }

    dirsum
}

fn main() {
    let input = std::fs::read_to_string("src/day07.txt").unwrap();
    let root = Directory::parse_term_output(input);
    let sum = 0;
    let sums = Rc::new(RefCell::new(Vec::new()));

    get_sizes(root, sum, Rc::clone(&sums));
    println!("{:?}", sums);
    println!("Part one: {}", sums.borrow().iter().sum::<u32>());
}
