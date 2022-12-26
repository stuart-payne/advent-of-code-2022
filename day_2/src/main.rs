use std::io::{self, BufRead, BufReader, Lines};
use std::{collections::HashMap, fs};

const FILE_INPUT_PATH: &str = "./input.txt";

/*
 *   a b c
 *
 * x 3 6 0
 * y 0 3 6
 * z 6 0 3
 */

const SOLUTIONS: [[u32; 3]; 3] = [[3, 0, 6], [6, 3, 0], [0, 6, 3]];

fn main() {
    println!("Hello, world!");
    let game_indices: HashMap<String, usize> = HashMap::from([
        ("A".to_string(), 0),
        ("B".to_string(), 1),
        ("C".to_string(), 2),
        ("X".to_string(), 0),
        ("Y".to_string(), 1),
        ("Z".to_string(), 2),
    ]).;

    let game_scores: HashMap<String, u32> = HashMap::from([
        ("X".to_string(), 1),
        ("Y".to_string(), 2),
        ("Z".to_string(), 3),
    ]);
    part_one(&game_indices, &game_scores);
    /*
     *   a b c
     * x Z X Y
     * y X Y Z
     * z Y Z X
     *
     */
    let p2_solutions: [[String; 3]; 3] = [
        ["Z".to_string(), "X".to_string(), "Y".to_string()],
        ["X".to_string(), "Y".to_string(), "Z".to_string()],
        ["Y".to_string(), "Z".to_string(), "X".to_string()],
    ];
    part_two(&game_indices, &game_scores, p2_solutions);
}

fn part_one(indices: &HashMap<String, usize>, scores: &HashMap<String, u32>) {
    let file = fs::File::open(FILE_INPUT_PATH).expect("PATH ERROR HEHEXD");
    let reader = io::BufReader::new(file).lines();
    let mut total_score = 0;
    for line in reader {
        let round = line
            .expect("yeah no")
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let player_one = &round[1];
        let player_two = &round[0];
        let score = SOLUTIONS[*indices.get(player_one).expect("PLAYER ONE IS NO EXIST")]
            [*indices.get(player_two).expect("PLAYER ONE IS NO EXIST")]
            + scores.get(player_one).expect("SCORE FAILED");
        total_score += score;
    }
    println!("PART ONE");
    println!("Total score is: {}", total_score);
}

fn part_two(
    indices: &HashMap<String, usize>,
    scores: &HashMap<String, u32>,
    solutions: [[String; 3]; 3],
) {
    let file = fs::File::open(FILE_INPUT_PATH).expect("PATH ERROR HEHEXD");
    let reader = io::BufReader::new(file).lines();
    let result_scoring: HashMap<String, u32> = HashMap::from([
        ("X".to_string(), 0),
        ("Y".to_string(), 3),
        ("Z".to_string(), 6),
    ]);
    let mut total_score = 0;
    for line in reader {
        let round = line
            .expect("yeah no")
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let player_one = &round[0];
        let needed_result = &round[1];
        let move_to_play = &solutions[*indices.get(player_one).expect("PLAYER ONE IS NO EXIST")]
            [*indices.get(needed_result).expect("PLAYER ONE IS NO EXIST")];
        let score = result_scoring[needed_result]
            + scores.get(move_to_play).expect("FAILED ON MOVE TO PLAY");
        total_score += score;
    }
    println!("PART TWO");
    println!("Total score is: {}", total_score);
}
