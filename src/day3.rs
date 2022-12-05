/*
--- Day 3: Rucksack Reorganization ---

One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).

The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

    The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
    The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
    The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
    The fourth rucksack's compartments only share item type v.
    The fifth rucksack's compartments only share item type t.
    The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be converted to a priority:

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?

--- Part Two ---

As you finish identifying the misplaced items, the Elves come to you with another issue.

For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves. That is, if a group's badge is item type B, then all three Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.

The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is the right one is by finding the one item type that is common between all three Elves in each group.

Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type. So, in the above example, the first group's rucksacks are the first three lines:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg

And the second group's rucksacks are the next three lines:

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges. In the second group, their badge item type must be Z.

Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.

Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
*/

use std::collections::HashSet;

fn get_item_type_priority(item_type: &char) -> u8 {
    if item_type.is_lowercase() {
        *item_type as u8 - b'a' + 1
    } else {
        *item_type as u8 - b'A' + 1 + 26
    }
}

pub fn part1(input: &str) -> Result<String, &'static str> {
    let mut sum_of_priorities: i64 = 0;

    for line in input.lines() {
        let mut second_half = String::from(line);
        let first_half = second_half.split_off(line.len() / 2);

        let items_in_first_compartment: HashSet<char> = first_half.chars().collect();
        let items_in_second_compartment: HashSet<char> = second_half.chars().collect();

        let shared_item_type: Vec<_> = items_in_first_compartment
            .intersection(&items_in_second_compartment)
            .collect();

        sum_of_priorities += get_item_type_priority(shared_item_type[0]) as i64;
    }

    Ok(sum_of_priorities.to_string())
}

fn get_intersection(group: &mut Vec<HashSet<char>>) -> Option<char> {
    if let Some(mut intersection) = group.pop() {
        for rucksack in group {
            intersection = intersection
                .intersection(rucksack)
                .copied()
                .collect::<HashSet<char>>();
        }

        if intersection.len() == 1 {
            Some(intersection.into_iter().collect::<Vec<char>>()[0])
        } else {
            None
        }
    } else {
        None
    }
}

pub fn part2(input: &str) -> Result<String, &'static str> {
    let mut sum_of_priorities: i64 = 0;
    let mut list_of_group_item_types: Vec<HashSet<char>> = Vec::new();

    for (position, line) in input.lines().enumerate() {
        list_of_group_item_types.push(line.chars().collect());

        if ((position + 1) % 3) == 0 {
            if let Some(intersection) = get_intersection(&mut list_of_group_item_types) {
                sum_of_priorities += get_item_type_priority(&intersection) as i64;
            }

            list_of_group_item_types.clear()
        }
    }

    Ok(sum_of_priorities.to_string())
}

#[cfg(test)]
mod tests {
    use crate::day3::{get_intersection, part1, part2};
    use std::collections::HashSet;

    static TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_get_intersection_with_no_common_item_type() {
        let mut group = vec![
            HashSet::from(['a', 'b', 'c']),
            HashSet::from(['d', 'e', 'f']),
        ];

        assert_eq!(get_intersection(&mut group), None)
    }

    #[test]
    fn test_get_intersection_with_one_rucksack_in_group() {
        let mut group = vec![HashSet::from(['a', 'b', 'c'])];

        assert_eq!(get_intersection(&mut group), None)
    }

    #[test]
    fn test_get_intersection_with_empty_group() {
        let mut group = vec![];

        assert_eq!(get_intersection(&mut group), None)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(157.to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Ok(70.to_string()));
    }
}
