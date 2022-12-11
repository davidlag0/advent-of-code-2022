/*
--- Day 8: Treetop Tree House ---

The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.

The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:

30373
25512
65332
33549
35390

Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:

    The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
    The top-middle 5 is visible from the top and right.
    The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
    The left-middle 5 is visible, but only from the right.
    The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
    The right-middle 3 is visible from the right.
    In the bottom row, the middle 5 is visible, but the 3 and 4 are not.

With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.

Consider your map; how many trees are visible from outside the grid?

--- Part Two ---

Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: they would like to be able to see a lot of trees.

To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration. (If a tree is right on the edge, at least one of its viewing distances will be zero.)

The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large eaves to keep it dry, so they wouldn't be able to see higher than the tree house anyway.

In the example above, consider the middle 5 in the second row:

30373
25512
65332
33549
35390

    Looking up, its view is not blocked; it can see 1 tree (of height 3).
    Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to it).
    Looking right, its view is not blocked; it can see 2 trees.
    Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of height 5 that blocks its view).

A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).

However, you can do even better: consider the tree of height 5 in the middle of the fourth row:

30373
25512
65332
33549
35390

    Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
    Looking left, its view is not blocked; it can see 2 trees.
    Looking down, its view is also not blocked; it can see 1 tree.
    Looking right, its view is blocked at 2 trees (by a massive tree of height 9).

This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.

Consider each tree on your map. What is the highest scenic score possible for any tree?
*/

struct TreeGrid {
    size_x: usize,
    size_y: usize,
    grid: Vec<Vec<u32>>,
}

impl TreeGrid {
    pub fn new(input: &str) -> Self {
        let mut tree_grid: Vec<Vec<u32>> = Vec::new();

        for line in input.lines() {
            tree_grid.push(
                line.chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>(),
            )
        }

        Self {
            size_x: tree_grid[0].len(),
            size_y: tree_grid.len(),
            grid: tree_grid,
        }
    }

    pub fn is_visible(&self, x: usize, y: usize) -> Option<bool> {
        if (x < self.size_x) && (y < self.size_y) {
            // External borders.
            if (y == 0) || (y == self.size_y - 1) || (x == 0) || (x == self.size_x - 1) {
                Some(true)
            }
            // Interior.
            else {
                // Verify up to top.
                for y_delta in 1..=y {
                    if self.grid[y - y_delta][x] >= self.grid[y][x] {
                        break;
                    } else if (y - y_delta) == 0 {
                        return Some(true);
                    }
                }

                // Verify down to bottom.
                for y_delta in 1..self.size_y - y {
                    if self.grid[y + y_delta][x] >= self.grid[y][x] {
                        break;
                    } else if y + y_delta == self.size_y - 1 {
                        return Some(true);
                    }
                }

                // Verify towards right.
                for x_delta in 1..self.size_x - x {
                    if self.grid[y][x + x_delta] >= self.grid[y][x] {
                        break;
                    } else if x + x_delta == self.size_x - 1 {
                        return Some(true);
                    }
                }

                // Verify towards left.
                for x_delta in 1..=x {
                    if self.grid[y][x - x_delta] >= self.grid[y][x] {
                        break;
                    } else if (x - x_delta) == 0 {
                        return Some(true);
                    }
                }

                Some(false)
            }
        } else {
            None
        }
    }

    pub fn scenic_score(&self, x: usize, y: usize) -> Option<u64> {
        if (x < self.size_x) && (y < self.size_y) {
            let mut viewing_distance_top: u64 = 0;
            for y_delta in 1..=y {
                if self.grid[y - y_delta][x] >= self.grid[y][x] {
                    viewing_distance_top += 1;
                    break;
                } else {
                    viewing_distance_top += 1;
                }
            }

            let mut viewing_distance_down: u64 = 0;
            for y_delta in 1..self.size_y - y {
                if self.grid[y + y_delta][x] >= self.grid[y][x] {
                    viewing_distance_down += 1;
                    break;
                } else {
                    viewing_distance_down += 1;
                }
            }

            let mut viewing_distance_right: u64 = 0;
            for x_delta in 1..self.size_x - x {
                if self.grid[y][x + x_delta] >= self.grid[y][x] {
                    viewing_distance_right += 1;
                    break;
                } else {
                    viewing_distance_right += 1;
                }
            }

            let mut viewing_distance_left: u64 = 0;
            for x_delta in 1..=x {
                if self.grid[y][x - x_delta] >= self.grid[y][x] {
                    viewing_distance_left += 1;
                    break;
                } else {
                    viewing_distance_left += 1;
                }
            }

            let scenic_score = viewing_distance_top
                * viewing_distance_down
                * viewing_distance_right
                * viewing_distance_left;

            Some(scenic_score)
        } else {
            None
        }
    }
}

pub fn part1(input: &str) -> Result<String, String> {
    let tree_grid = TreeGrid::new(input);
    let mut number_of_visible_trees: u64 = 0;

    for y in 0..tree_grid.size_y {
        for x in 0..tree_grid.size_x {
            match tree_grid.is_visible(x, y) {
                Some(true) => number_of_visible_trees += 1,
                _ => continue,
            }
        }
    }

    Ok(number_of_visible_trees.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let tree_grid = TreeGrid::new(input);
    let mut scenic_scores: Vec<u64> = Vec::new();

    for y in 0..tree_grid.size_y {
        for x in 0..tree_grid.size_x {
            scenic_scores.push(tree_grid.scenic_score(x, y).unwrap());
        }
    }

    scenic_scores.sort();

    Ok(scenic_scores.pop().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use crate::day8::{part1, part2, TreeGrid};

    static TEST_INPUT: &str = "30373
25512
65332
33549
35390
";

    static TEST_INPUT_WITH_TREE_ONLY_VISIBLE_FROM_LEFT: &str = "55555
11555
55555
";

    #[test]
    fn test_treegrid_new() {
        assert_eq!(
            TreeGrid::new(TEST_INPUT).grid,
            vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0]
            ]
        )
    }

    #[test]
    fn test_treegrid_is_visible() {
        let tree_grid = TreeGrid::new(TEST_INPUT);

        // Top row.
        assert_eq!(tree_grid.is_visible(0, 0), Some(true));
        assert_eq!(tree_grid.is_visible(1, 0), Some(true));
        assert_eq!(tree_grid.is_visible(2, 0), Some(true));
        assert_eq!(tree_grid.is_visible(3, 0), Some(true));
        assert_eq!(tree_grid.is_visible(4, 0), Some(true));

        // Right column.
        assert_eq!(tree_grid.is_visible(4, 0), Some(true));
        assert_eq!(tree_grid.is_visible(4, 1), Some(true));
        assert_eq!(tree_grid.is_visible(4, 2), Some(true));
        assert_eq!(tree_grid.is_visible(4, 3), Some(true));
        assert_eq!(tree_grid.is_visible(4, 4), Some(true));

        // Bottom row.
        assert_eq!(tree_grid.is_visible(0, 4), Some(true));
        assert_eq!(tree_grid.is_visible(1, 4), Some(true));
        assert_eq!(tree_grid.is_visible(2, 4), Some(true));
        assert_eq!(tree_grid.is_visible(3, 4), Some(true));
        assert_eq!(tree_grid.is_visible(4, 4), Some(true));

        // Left column.
        assert_eq!(tree_grid.is_visible(0, 0), Some(true));
        assert_eq!(tree_grid.is_visible(0, 1), Some(true));
        assert_eq!(tree_grid.is_visible(0, 2), Some(true));
        assert_eq!(tree_grid.is_visible(0, 3), Some(true));
        assert_eq!(tree_grid.is_visible(0, 4), Some(true));

        // Non-existing trees.
        assert_eq!(tree_grid.is_visible(0, 5), None);
        assert_eq!(tree_grid.is_visible(5, 0), None);
        assert_eq!(tree_grid.is_visible(5, 5), None);

        // Interior trees.
        assert_eq!(tree_grid.is_visible(1, 1), Some(true));
        assert_eq!(tree_grid.is_visible(2, 1), Some(true));
        assert_eq!(tree_grid.is_visible(3, 1), Some(false));
        assert_eq!(tree_grid.is_visible(1, 2), Some(true));
        assert_eq!(tree_grid.is_visible(2, 2), Some(false));
        assert_eq!(tree_grid.is_visible(3, 2), Some(true));
        assert_eq!(tree_grid.is_visible(1, 3), Some(false));
        assert_eq!(tree_grid.is_visible(2, 3), Some(true));
        assert_eq!(tree_grid.is_visible(3, 3), Some(false));
    }

    #[test]
    fn test_treegrid_is_visible_from_left_only() {
        let tree_grid = TreeGrid::new(TEST_INPUT_WITH_TREE_ONLY_VISIBLE_FROM_LEFT);
        assert_eq!(tree_grid.is_visible(2, 1), Some(true));
    }

    #[test]
    fn test_treegrid_scenico_score_with_non_existing_trees() {
        let tree_grid = TreeGrid::new(TEST_INPUT);

        assert_eq!(tree_grid.scenic_score(0, 5), None);
        assert_eq!(tree_grid.scenic_score(5, 0), None);
        assert_eq!(tree_grid.scenic_score(5, 5), None);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(21.to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Ok(8.to_string()));
    }
}
