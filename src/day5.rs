/*
--- Day 5: Supply Stacks ---

The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3

In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3

Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3

Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3

The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?

--- Part Two ---

As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.

Again considering the example above, the crates begin in the same configuration:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

Moving a single crate from stack 2 to stack 1 behaves the same as before:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3

However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:

        [D]
        [N]
    [C] [Z]
    [M] [P]
 1   2   3

Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

        [D]
        [N]
[C]     [Z]
[M]     [P]
 1   2   3

Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:

        [D]
        [N]
        [Z]
[M] [C] [P]
 1   2   3

In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?
*/

use regex::Regex;
use std::collections::VecDeque;

fn solve(input: &str, group_crates_when_moving: bool) -> Result<String, &'static str> {
    let re_stacks = Regex::new(r"(?:\[|\s)(?P<crate>[A-Z]|\s)(?:\]|\s)\s?").unwrap();
    let re_move = Regex::new(r"^move\s(?P<number_of_crates_to_move>\d+)\sfrom\s(?P<from_stack>\d+)\sto\s(?P<to_stack>\d+)$").unwrap();
    let mut number_of_stacks: usize = 0;
    let mut stacks: Vec<VecDeque<String>> = Vec::new();
    let mut crates_at_the_top: String = String::from("");

    for line in input.lines() {
        if !line.contains("move") {
            let matches: Vec<_> = re_stacks.captures_iter(line).collect();

            if number_of_stacks == 0 {
                match matches.len() {
                    0 => return Err("Unknown format for stacks"),
                    other => number_of_stacks += other,
                }

                for _ in 0..number_of_stacks {
                    stacks.push(VecDeque::new());
                }
            }

            for (index, cap) in matches.iter().enumerate() {
                if &cap["crate"] != " " {
                    stacks[index].push_back(String::from(&cap["crate"]));
                }
            }
        } else {
            let matches: Vec<_> = re_move.captures_iter(line).collect();
            let mut from_stack: usize = 0;
            let mut to_stack: usize = 0;
            let mut number_of_crates_to_move: usize = 0;

            if !matches.is_empty() {
                if matches[0]["number_of_crates_to_move"]
                    .parse::<usize>()
                    .is_ok()
                {
                    number_of_crates_to_move = matches[0]["number_of_crates_to_move"]
                        .parse::<usize>()
                        .unwrap();
                }

                if matches[0]["from_stack"].parse::<usize>().is_ok() {
                    from_stack = matches[0]["from_stack"].parse::<usize>().unwrap() - 1;
                }

                if matches[0]["to_stack"].parse::<usize>().is_ok() {
                    to_stack = matches[0]["to_stack"].parse::<usize>().unwrap() - 1;
                }
            }

            if !group_crates_when_moving {
                for _ in 0..number_of_crates_to_move {
                    if let Some(crate_item) = &stacks[from_stack].pop_front() {
                        stacks[to_stack].push_front(String::from(crate_item));
                    }
                }
            } else {
                let mut temporary_stack = stacks[from_stack].clone();

                stacks[from_stack] = temporary_stack.split_off(number_of_crates_to_move).clone();
                temporary_stack.append(&mut stacks[to_stack]);
                stacks[to_stack] = temporary_stack.clone();
            }
        }
    }

    for mut stack in stacks {
        if let Some(crate_item) = &stack.pop_front() {
            crates_at_the_top += crate_item;
        }
    }

    Ok(crates_at_the_top)
}

pub fn part1(input: &str) -> Result<String, &'static str> {
    solve(input, false)
}

pub fn part2(input: &str) -> Result<String, &'static str> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use crate::day5::{part1, part2};

    static TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    static TEST_INPUT_BAD_MOVE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move abc from 1 to 2
";

    static TEST_INPUT_BAD_STACKS: &str = "aaaaaaaaaaa
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok("CMZ".to_string()));
    }

    #[test]
    fn test_part1_bad_move() {
        assert_eq!(part1(TEST_INPUT_BAD_MOVE), Ok("MZ".to_string()));
    }

    #[test]
    fn test_part1_bad_stacks() {
        assert_eq!(
            part1(TEST_INPUT_BAD_STACKS),
            Err("Unknown format for stacks")
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Ok("MCD".to_string()));
    }
}
