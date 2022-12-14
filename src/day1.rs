/*
--- Day 1: Calorie Counting ---

Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas. For that, their favorite snack is a special type of star fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.

To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).

The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up with the following list:

1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

This list represents the Calories of the food carried by five Elves:

    The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
    The second Elf is carrying one food item with 4000 Calories.
    The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
    The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
    The fifth Elf is carrying one food item with 10000 Calories.

In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

--- Part Two ---

By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.

To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.

In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories). The sum of the Calories carried by these three elves is 45000.

Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
*/

pub fn calculate_top_three(temporary_sum: i64, top_three: &mut Vec<i64>) -> &mut Vec<i64> {
    if temporary_sum > top_three[2] {
        top_three.push(temporary_sum);
        top_three.sort_by(|a, b| b.cmp(a));

        while top_three.len() > 3 {
            top_three.pop();
        }
    }

    top_three
}

pub fn part1(input: &str) -> Result<String, String> {
    let mut temporary_sum: i64 = 0;
    let mut highest_sum: i64 = 0;

    for line in input.split('\n') {
        match line.parse::<i64>() {
            Ok(n) => {
                temporary_sum += n;
            }
            Err(_) => {
                if temporary_sum > highest_sum {
                    highest_sum = temporary_sum;
                }

                temporary_sum = 0;
            }
        }
    }

    Ok(highest_sum.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let mut temporary_sum: i64 = 0;
    let mut top_three: Vec<i64> = vec![0, 0, 0];

    for line in input.split('\n') {
        match line.parse::<i64>() {
            Ok(n) => {
                temporary_sum += n;
            }
            Err(_) => {
                top_three = calculate_top_three(temporary_sum, &mut top_three).to_vec();

                temporary_sum = 0;
            }
        }
    }

    Ok(calculate_top_three(temporary_sum, &mut top_three)
        .iter()
        .sum::<i64>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2};

    static TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    static TEST_INPUT_2: &str = "10000

5000

4000

1000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(24000.to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Ok(45000.to_string()));
        assert_eq!(part2(TEST_INPUT_2), Ok(19000.to_string()));
    }
}
