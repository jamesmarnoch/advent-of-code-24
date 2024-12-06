use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let re_mul = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();
    let mut total = 0;
    for (_, [a, b]) in re_mul.captures_iter(&input).map(|c| c.extract()) {
        total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }
    println!("{}", total);

    let mut do_total = 0;
    let dont_blocks = input.split("don't()").collect::<Vec<&str>>();

    for (_, [a, b]) in re_mul.captures_iter(dont_blocks[0]).map(|c| c.extract()) {
        do_total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }

    for i in 1..dont_blocks.len() {
        match dont_blocks[i].split_once("do()") {
            Some( (_, v)) => {
                for (_, [a, b]) in re_mul.captures_iter(v).map(|c| c.extract()) {
                    do_total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
                }
            }
            _ => {}
        }
    }
    println!("{}", do_total);
}
