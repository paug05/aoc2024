use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("error");
    let mut safe_count1 = 0;
    let mut safe_count2 = 0;
    for line in input.lines() {
        let split = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if check_line(&split, safe_count1) {
            safe_count1 += 1;
        }

        for i in 0..split.len(){
            let mut help_list = split.clone();
            help_list.remove(i);
            if check_line(&help_list, safe_count2){
                safe_count2 += 1;
                break;
            }
        }

    }
    println!("safe_count: {}", safe_count1);
    println!("safe_count2: {}", safe_count2);
}

fn check_line(split: &Vec<i32>, safe_count: i32) -> bool {
    let increasing = split.windows(2).all(|w| w[0] < w[1] && (w[0]-w[1]).abs() <= 3);
    let decreasing = split.windows(2).all(|w| w[0] > w[1] && (w[0]-w[1]).abs() <= 3);
    if increasing == true || decreasing == true {
        true
    } else { false }
}
