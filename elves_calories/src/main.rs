use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("wrong fucking path idiot");
    println!("Part one: ");
    part_one(&contents); 
    println!("Part two: ");
    part_two(&contents);
}

fn part_one(contents: &String) {
    let split = contents.split("\n");       
    let mut largest: u32 = 0;
    let mut largest_elf: usize = 0;
    let mut current_elf: usize = 0;
    let mut current_count: u32 = 0;

    for line in split.into_iter() {
        if line == "" {
             if largest < current_count {
                largest = current_count;
                largest_elf = current_elf;
            }
            current_elf += 1;
            current_count = 0;
        } else {
            current_count += line.parse::<u32>().expect("YOU FUCKED UP BRUV");
        }
    }
    println!("{}", largest_elf);
    println!("{}: largest", largest);

}

fn part_two(contents: &String) {
    let split = contents.split("\n");       
    let mut current_count: u32 = 0;
    let mut largest_three: [u32; 3] = [0; 3];

    for line in split.into_iter() {
        if line == "" {
            for i in 0..largest_three.len() {
                if current_count > largest_three[i] {
                    largest_three[i] = current_count;
                    largest_three.sort();
                    break;
                 }
            }
            current_count = 0;
        } else {
            current_count += line.parse::<u32>().expect("YOU FUCKED UP BRUV");
        }
    }

    let mut count: u32 = 0;
    for amount in largest_three {
        count += amount;
    }
    println!("{:?}; largest three ", largest_three);
    println!("{}: total", count);
}
