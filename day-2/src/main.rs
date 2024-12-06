use std::fs;
use std::io::{BufRead, BufReader};

fn is_safe_dampener(report: &Vec<i32>) -> bool {
    let mut perms: Vec<Vec<i32>> = Vec::new();
    for (i, _elem) in report.iter().enumerate() {
        let mut xx = report.clone();
        xx.remove(i);
        perms.push(xx);
    }

    //println!("{:?}", perms);

    let mut safe = is_safe(report);

    for perm in perms.iter() {
        safe = safe || is_safe(perm);
    }

    safe
}

fn is_safe(report: &Vec<i32>) -> bool {
    let up_safe = report.is_sorted_by(|&a, &b| a < b && a - b < 0 && a - b > -4);

    let down_safe = report.is_sorted_by(|&a, &b| a > b && a - b > 0 && a - b < 4);

    //println!("line: {}, up_safe: {}, down_safe: {}", s , up_safe , down_safe);

    up_safe || down_safe
}

fn main() {
    let buf_reader = BufReader::new(fs::File::open("input.txt").unwrap());

    let lines = buf_reader.lines();

    let mut count = 0;
    let mut count_dampener = 0;
    lines.for_each(|line| {
        let report = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if is_safe(&report) {
            count += 1;
        }
        if is_safe_dampener(&report) {
            count_dampener += 1;
        }
    });

    println!("Total safe: {}", count);
    println!("Total safe dampener: {}", count_dampener);
}
