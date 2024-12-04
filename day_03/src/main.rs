use std::fs::read_to_string;
use regex::Regex;


fn main() {
    let input = read_to_string("./input.txt").expect("error");
    let mut count_1 = 0;
    let mul_reg = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    for line in input.lines() {
        let matches: Vec<_> = mul_reg.find_iter(line).map(|m| m.as_str()).collect();
        for mut mul in matches {
            let test = mul[4..mul.len() - 1].to_string();
            let numbers:Vec<_> = test
                .split(',')
                .filter_map(|n| n.trim().parse::<i32>().ok())
                .collect();
            count_1 += numbers[0]* numbers[1];
        }

    }
    println!("count: {}", count_1);


    let mut count_2 = 0;
    let mul_reg_2 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut do_enabled = true;

    for elem in mul_reg_2.captures_iter(&input) {
        if let Some(m) = elem.get(1) {
            if do_enabled {
                count_2 += elem[1].parse::<i32>().unwrap() * elem[2].parse::<i32>().unwrap();
            }
        } else if elem.get(0).map_or("",|m| m.as_str()) == "do()"{
            do_enabled = true;}

        else if elem.get(0).map_or("",|m| m.as_str()) == "don't()" {
            do_enabled = false;
        }
    }
    println!("count: {}", count_2);

}

