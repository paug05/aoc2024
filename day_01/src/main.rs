use std::fs::read_to_string;

fn main() {
    puzzle_1();
    puzzle_2()
}


fn puzzle_1() {
    let input = read_to_string("./input.txt").expect("error");

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let split = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        list1.push(split[0]);
        list2.push(split[1]);
    }

    list1.sort();
    list2.sort();

    let solution: i32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("Solution 1: {}", solution);
}

fn puzzle_2() {
    let input = read_to_string("./input.txt").expect("error");
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let split = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        list1.push(split[0]);
        list2.push(split[1]);
    }
    let mut similarity_score = 0;

    for elem in list1 {
        let mut counter = 0;
        for i in 0..list2.len() {
            if (list2[i] == elem){
                counter += 1;
            }
        }
        similarity_score += counter * elem;
    }
    println!("Similarity score: {}", similarity_score);



}
