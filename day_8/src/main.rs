use std::{
    fs::File,
    io::{BufRead, BufReader},
    collections::HashSet,
};

const FILE_INPUT: &str = "input.txt";

fn main() {
    println!("PART_ONE: {}", part_one(FILE_INPUT));
    println!("PART_TWO: {}", part_two(FILE_INPUT));
}

fn part_one(file_path: &str) -> usize {
    let grid = build_grid(file_path);
    count_grid(&grid)
}

fn build_grid(file_path: &str) -> Vec<Vec<u32>> {
    let file_input = File::open(file_path).expect("yikes dude");
    BufReader::new(file_input)
        .lines()
        .map(|l| {
            l.expect("line failed")
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn count_grid(grid: &Vec<Vec<u32>>) -> usize {
    // left and right
    let mut visible_trees: HashSet<String> = HashSet::new();
    for line in 0..grid.len() {
        let visible_indices = count_visible(&grid[line]);
        for vis in visible_indices {
            visible_trees.insert(vis.to_string() + ":" + &line.to_string());
        }
    }
    for column_ind in 0..grid[0].len() {
        let mut column: Vec<u32> = Vec::new();
        for line in grid {
            column.push(line[column_ind]);
        } 
        let visible_indices = count_visible(&column);
        for vis in visible_indices {
            visible_trees.insert(column_ind.to_string() + ":" + &vis.to_string());
        }
    }
    visible_trees.len()
}

fn count_visible(line: &Vec<u32>) -> Vec<usize> {
    let mut visible_trees: HashSet<usize> = HashSet::new();
    let mut current_largest: usize = 0;
    visible_trees.insert(current_largest);
    for ind in 1..line.len() {
        if line[ind] > line[current_largest] {
            visible_trees.insert(ind);
            current_largest = ind;
        }
    }

    let mut reverse_largest: usize = line.len() - 1;
    visible_trees.insert(reverse_largest);
    let range = 0..line.len() - 1;
    for ind in range.rev() {
        if line[ind] > line[reverse_largest] {
            visible_trees.insert(ind);
            reverse_largest = ind;
        }
    }
    visible_trees.drain().collect()
}

#[derive(Debug)]
struct Score {
    right: u32,
    left: u32,
    down: u32,
    up: u32,
}

impl Score {
    fn new() -> Score {
        Score {
           right: 0,
           left: 0,
           down: 0,
           up: 0,
        }
    }

    fn total(&self) -> u32 {
        self.right * self.left * self.up * self.down
    }
}

fn part_two(file_input: &str) -> u32 {
    let grid = build_grid(file_input);
    let mut largest_score: u32 = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let tree_score = grid[x][y];
            let mut score = Score::new();

            //right
            for x_walker in x + 1..grid.len() {
                if grid[x_walker][y] >= tree_score {
                    score.right += 1;
                    break;
                }
                score.right += 1;
            }

            // left
            let left_range = 0..x;
            let left_iter = left_range.rev();

            for x_walker in left_iter {
                if grid[x_walker][y] >= tree_score {
                    score.left += 1;
                    break;
                }
                score.left += 1;
            }

            // down
            for y_walker in y + 1..grid.len() {
                if grid[x][y_walker] >= tree_score {
                    score.down += 1;
                    break;
                }
                score.down += 1;
            }

            let top_range = 0..y;
            let top_iter = top_range.rev();

            for y_walker in top_iter {
                if grid[x][y_walker] >= tree_score {
                    score.up += 1;
                    break;
                }
                score.up += 1;
            }

            if score.total() > largest_score {
                largest_score = score.total();
            }
        }
    }
    largest_score
}

#[cfg(test)]
mod test {
    const TEST_FILE_PATH: &str = "./test_input.txt";
    use crate::part_one;
    use crate::part_two;


    #[test]
    fn part_one_test() {
        assert_eq!(part_one(TEST_FILE_PATH), 21);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(TEST_FILE_PATH), 8);
    }

    mod grid {
        use super::TEST_FILE_PATH;
        use crate::build_grid;

        #[test]
        fn grid_test() {
            let grid = build_grid(TEST_FILE_PATH);
            assert_eq!(grid.len(), 5);
            assert_eq!(grid[0].len(), 5);
        }
    }

    mod count {
        use crate::count_visible;
        #[test]
        fn count_visible_test() {
            let mut result =count_visible(&Vec::from([1, 3, 3, 5, 4]));  
            result.sort();
            assert_eq!(result, [0, 1, 3, 4]);
        }
        #[test]
        fn count_visible_test_two() {
            let mut result =count_visible(&Vec::from([1, 3, 3, 5, 3, 1, 7]));  
            result.sort();
            assert_eq!(result, [0, 1, 3, 6]);
        }
        #[test]
        fn count_visible_test_three() {
            let mut result = count_visible(&Vec::from([6, 5, 3, 3, 2]));  
            result.sort();
            assert_eq!(result, [0, 1, 3, 4 ]);
        }
    }
}
