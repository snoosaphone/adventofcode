#[derive(Debug)]
struct FileSystem<'a> {
    root_dir: Dir<'a>,
    current_dir: String,
}

impl FileSystem<'_> {
    fn new(name: &str) -> Self {
        let new_root_dir = Dir::new(name);
        Self {
            root_dir: new_root_dir,
            current_dir: name.to_string(),
        }
    }

    // Take in node name
    // fn find_dir<'a>(curr_node: &<'a> Dir, name: &<'a> str) -> Option<&Dir> {
    // if curr_node.name == name {
    // return Some(&curr_node);
    // } else {
    // for child_dir in &curr_node.child_dirs {
    // if child_dir.name == name {
    // return Some(child_dir);
    // }
    // }

    fn change_dir(target_dir: &str) {
        
    }

    // for child_dir in &curr_node.child_dirs {
    // return Self::find_dir(child_dir, name);
    // }
    // }

    // None
    // }
}

#[derive(Debug)]
struct Dir<'a> {
    name: String,
    parent_dir: &'a Dir<'a>,
    child_dirs: Vec<Dir<'a>>,
    child_files: Vec<File>,
}

impl Dir<'_>{
    fn new(name: &str, parent_dir: &Dir) -> Self {
        Self {
            name: name.to_string(),
            parent_dir,
            child_dirs: Vec::new(),
            child_files: Vec::new(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct File {
    name: String,
    size: usize,
}

fn main() {
    let input_contents: Vec<String> = vec![
        "$ cd /".to_string(),
        "$ ls".to_string(),
        "dir a".to_string(),
        "14848514 b.txt".to_string(),
        "8504156 c.dat".to_string(),
        "dir d".to_string(),
        "$ cd a".to_string(),
        "$ ls".to_string(),
        "dir e".to_string(),
        "29116 f".to_string(),
        "2557 g".to_string(),
        "62596 h.lst".to_string(),
        "$ cd e".to_string(),
        "$ ls".to_string(),
        "584 i".to_string(),
        "$ cd ..".to_string(),
        "$ cd ..".to_string(),
        "$ cd d".to_string(),
        "$ ls".to_string(),
        "4060174 j".to_string(),
        "8033020 d.log".to_string(),
        "5626152 d.ext".to_string(),
        "7214296 k".to_string(),
    ];

    // let input_contents = get_file_contents("./input.txt");

    let mut filesystem = FileSystem::new("/");

    let mut current_dir = &mut filesystem.root_dir;

    let mut current_cmd = "".to_string();

    for (i, line) in input_contents.iter().enumerate() {
        match line.split_whitespace().next() {
            Some("$") => {
                println!("{}, Found command", i);

                current_cmd = line.split_whitespace().collect::<Vec<_>>()[1].to_string();
                println!("{}", current_cmd);
                match current_cmd.as_str() {
                    "cd" => {
                        println!("cd command");
                        let new_dir = line.rsplit(' ').next().unwrap();

                        // Get reference for the directory being changed to

                        // if current_dir.name != new_dir {
                            // for dir in &current_dir.child_dirs {
                                // if dir.name == new_dir {
                                    // *current_dir = dir;
                                // }
                            // }
                        // }
                    }
                    _ => println!("Not implemented command"),
                }
            }
            Some("dir") => {
                println!("{}, Found dir", i);
                current_dir.child_dirs.push(Dir {
                    name: line.rsplit(' ').next().unwrap().to_string(),
                    child_dirs: Vec::new(),
                    child_files: Vec::new(),
                });
            }
            Some(x) => {
                println!("{}, Found file of size {}", i, x);
                current_dir.child_files.push(File {
                    name: line.rsplit(' ').next().unwrap().to_string(),
                    size: line
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                });
            }
            None => println!("Not Implemented"),
        }
    }

    println!("{:?}", filesystem.root_dir);
}
