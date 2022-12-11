/*
--- Day 7: No Space Left On Device ---

You can hear birds chirping and raindrops hitting leaves as the expedition proceeds. Occasionally, you can even hear much louder sounds in the distance; how big do the animals get out here, anyway?

The device the Elves gave you has problems with more than just its communication system. You try to run a system update:

$ system-update --please --pretty-please-with-sugar-on-top
Error: No space left on device

Perhaps you can delete some files to make space for the update?

You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input). For example:

$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k

The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.

Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:

    cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
        cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
        cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
        cd / switches the current directory to the outermost directory, /.
    ls means list. It prints out all of the files and directories immediately contained by the current directory:
        123 abc means that the current directory contains a file named abc with size 123.
        dir xyz means that the current directory contains a directory named xyz.

Given the commands and output in the example above, you can determine that the filesystem looks visually like this:

- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)

Here, there are four directories: / (the outermost directory), a and d (which are in /), and e (which is in a). These directories also contain files of various sizes.

Since the disk is full, your first step should probably be to find directories that are good candidates for deletion. To do this, you need to determine the total size of each directory. The total size of a directory is the sum of the sizes of the files it contains, directly or indirectly. (Directories themselves do not count as having any intrinsic size.)

The total sizes of the directories above can be found as follows:

    The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
    The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
    Directory d has total size 24933642.
    As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size of every file.

To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes. In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)

Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?

--- Part Two ---

Now, you're ready to choose a directory to delete.

The total disk space available to the filesystem is 70000000. To run the update, you need unused space of at least 30000000. You need to find a directory you can delete that will free up enough space to run the update.

In the example above, the total size of the outermost directory (and thus the total amount of used space) is 48381165; this means that the size of the unused space must currently be 21618835, which isn't quite the 30000000 required by the update. Therefore, the update still requires a directory with total size of at least 8381165 to be deleted before it can run.

To achieve this, you have the following options:

    Delete directory e, which would increase unused space by 584.
    Delete directory a, which would increase unused space by 94853.
    Delete directory d, which would increase unused space by 24933642.
    Delete directory /, which would increase unused space by 48381165.

Directories e and a are both too small; deleting them would not free up enough space. However, directories d and / are both big enough! Between these, choose the smallest: d, increasing unused space by 24933642.

Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. What is the total size of that directory?
*/

const MAX_DIRECTORY_SIZE_TO_CONSIDER: u64 = 100_000;
const TOTAL_DISK_SPACE: u64 = 70_000_000;
const UNUSED_SPACE_REQUIRED_FOR_UPDATE: u64 = 30_000_000;

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Directory {
    total_file_size: u64,
    subdirectories: Vec<String>,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            total_file_size: 0,
            subdirectories: Vec::new(),
        }
    }
}

fn directory_size(filesystem: &HashMap<String, Directory>, directory_name: &str) -> u64 {
    let mut total_size: u64 = 0;

    let directory: &Directory = match filesystem.get(directory_name) {
        Some(dir) => dir,
        None => {
            println!("Could not find item: {}", directory_name);
            return 0;
        }
    };
    total_size += directory.total_file_size;

    match directory.subdirectories.is_empty() {
        true => {
            return directory.total_file_size;
        }
        false => {
            for subdirectory in directory.subdirectories.iter() {
                total_size += directory_size(filesystem, subdirectory);
            }
        }
    }

    total_size
}

fn build_filesystem(input: &str) -> Result<HashMap<String, Directory>, String> {
    let mut path: Vec<&str> = Vec::new();
    let mut filesystem: HashMap<String, Directory> = HashMap::new();

    for line in input.lines() {
        // Idea for the command and 'match command' below taken from:
        // https://github.com/orlp/aoc2022/blob/master/src/bin/day07.rs#L34-L46

        let mut command = Vec::with_capacity(3);
        command.splice(.., line.split_ascii_whitespace());

        match command[..] {
            ["$", "cd", directory] => match directory {
                ".." => {
                    path.pop();
                }
                other => {
                    path.push(other);

                    filesystem
                        .entry(path.join("/"))
                        .or_insert_with(Directory::new);
                }
            },
            ["$", "ls"] => {
                continue;
            }
            ["dir", directory_name] => {
                let directory_path = path.join("/") + "/" + directory_name;

                let current_directory = match filesystem.get_mut(&path.join("/")) {
                    Some(dir) => dir,
                    None => return Err(format!("Could not find item: {}", path.join("/"))),
                };
                current_directory
                    .subdirectories
                    .push(directory_path.clone());
            }
            _ => {
                let current_directory = match filesystem.get_mut(&path.join("/")) {
                    Some(dir) => dir,
                    None => return Err(format!("Could not find item: {}", path.join("/"))),
                };

                match line.split(' ').collect::<Vec<&str>>()[0].parse::<u64>() {
                    Ok(file_size) => current_directory.total_file_size += file_size,
                    Err(err) => println!(
                        "Could not parse file size: {}. Ignoring line: {}",
                        err, line
                    ),
                }
            }
        }
    }

    Ok(filesystem)
}

pub fn part1(input: &str) -> Result<String, String> {
    let mut total_size: u64 = 0;

    let filesystem = match build_filesystem(input) {
        Ok(filesystem) => filesystem,
        Err(err) => return Err(err),
    };

    for directory in filesystem.keys() {
        let size = directory_size(&filesystem, directory);

        if size <= MAX_DIRECTORY_SIZE_TO_CONSIDER {
            total_size += size;
        }
    }

    Ok(total_size.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let mut sizes_of_candidate_directories_to_delete: Vec<u64> = Vec::new();

    let filesystem = match build_filesystem(input) {
        Ok(filesystem) => filesystem,
        Err(err) => return Err(err),
    };

    let unused_space = TOTAL_DISK_SPACE - directory_size(&filesystem, "/");

    if unused_space > UNUSED_SPACE_REQUIRED_FOR_UPDATE {
        Err("Problem with the filesystem disk space! The expectation is to have just enough free space to the upgrade or less".to_string())
    } else {
        let additonal_space_required_for_update = UNUSED_SPACE_REQUIRED_FOR_UPDATE - unused_space;

        for directory in filesystem.keys() {
            let size = directory_size(&filesystem, directory);

            if size >= additonal_space_required_for_update {
                sizes_of_candidate_directories_to_delete.push(size);
            }
        }

        sizes_of_candidate_directories_to_delete.sort_by(|a, b| b.cmp(a));

        Ok(sizes_of_candidate_directories_to_delete
            .pop()
            .unwrap()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::{build_filesystem, directory_size, part1, part2, Directory};
    use std::collections::HashMap;

    static TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    static TEST_INPUT_DIR_COMMAND_AND_NO_CURRENT_DIRECTORY: &str = "dir missing_previous_cd
";

    static TEST_INPUT_FILESIZE_AND_FILENAME_AND_NO_CURRENT_DIRECTORY: &str = "1234 test.txt
";

    static TEST_INPUT_BAD_FILESIZE: &str = "$ cd /
$ ls
dir a
bad_file_size b.txt
8504156 c.dat
";

    static TEST_INPUT_EMPTY_FILESYSTEM: &str = "$ ls
";

    #[test]
    fn test_build_filesystem() {
        assert_eq!(
            build_filesystem(TEST_INPUT).unwrap(),
            HashMap::from([
                (
                    "/".to_string(),
                    Directory {
                        total_file_size: 23352670,
                        subdirectories: vec!["//a".to_string(), "//d".to_string()]
                    }
                ),
                (
                    "//a".to_string(),
                    Directory {
                        total_file_size: 94269,
                        subdirectories: vec!["//a/e".to_string()]
                    }
                ),
                (
                    "//a/e".to_string(),
                    Directory {
                        total_file_size: 584,
                        subdirectories: vec![]
                    }
                ),
                (
                    "//d".to_string(),
                    Directory {
                        total_file_size: 24933642,
                        subdirectories: vec![]
                    }
                )
            ])
        )
    }

    #[test]
    fn test_build_filesystem_with_dir_command_and_no_current_directory() {
        assert_eq!(
            build_filesystem(TEST_INPUT_DIR_COMMAND_AND_NO_CURRENT_DIRECTORY),
            Err("Could not find item: ".to_string())
        );
    }

    #[test]
    fn test_build_filesystem_with_filesize_and_filename_and_no_current_directory() {
        assert_eq!(
            build_filesystem(TEST_INPUT_FILESIZE_AND_FILENAME_AND_NO_CURRENT_DIRECTORY),
            Err("Could not find item: ".to_string())
        );
    }

    #[test]
    fn test_build_filesystem_with_bad_filesize() {
        assert_eq!(
            build_filesystem(TEST_INPUT_BAD_FILESIZE).unwrap(),
            HashMap::from([(
                "/".to_string(),
                Directory {
                    total_file_size: 8504156,
                    subdirectories: vec!["//a".to_string()]
                }
            )])
        );
    }

    #[test]
    fn test_directory_size_e() {
        let filesystem = build_filesystem(TEST_INPUT).unwrap();
        assert_eq!(directory_size(&filesystem, "//a/e"), 584);
    }

    #[test]
    fn test_directory_size_d() {
        let filesystem = build_filesystem(TEST_INPUT).unwrap();
        assert_eq!(directory_size(&filesystem, "//d"), 24933642);
    }

    #[test]
    fn test_directory_size_a() {
        let filesystem = build_filesystem(TEST_INPUT).unwrap();
        assert_eq!(directory_size(&filesystem, "//a"), 94853);
    }

    #[test]
    fn test_directory_size_root() {
        let filesystem = build_filesystem(TEST_INPUT).unwrap();
        assert_eq!(directory_size(&filesystem, "/"), 48381165);
    }

    #[test]
    fn test_directory_size_non_existing_dir() {
        let filesystem = build_filesystem(TEST_INPUT).unwrap();
        assert_eq!(directory_size(&filesystem, "//test"), 0);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(95437.to_string()));
    }

    #[test]
    fn test_part1_bad_input() {
        assert_eq!(
            part1(TEST_INPUT_DIR_COMMAND_AND_NO_CURRENT_DIRECTORY),
            Err("Could not find item: ".to_string())
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Ok(24933642.to_string()));
    }

    #[test]
    fn test_part2_bad_input() {
        assert_eq!(
            part2(TEST_INPUT_DIR_COMMAND_AND_NO_CURRENT_DIRECTORY),
            Err("Could not find item: ".to_string())
        );
    }

    #[test]
    fn test_part2_empty_filesystem() {
        assert_eq!(
            part2(TEST_INPUT_EMPTY_FILESYSTEM),
            Err("Problem with the filesystem disk space! The expectation is to have just enough free space to the upgrade or less".to_string())
        );
    }
}
