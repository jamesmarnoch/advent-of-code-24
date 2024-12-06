use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let buf_reader = BufReader::new(fs::File::open("input.txt").unwrap());

    let lines = buf_reader.lines();
    let mut v1 : Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    for line in lines {
        let l = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();
        v1.push(l[0]);
        v2.push(l[1]);
    }
    v1.sort();
    v2.sort();
    let mut sum = 0;
    for (l1, l2) in v1.iter().zip(v2.iter()) {
        sum += (l1 - l2).abs();
    }
    println!("Part 1: {:?}", sum);
    
    
    let mut sum2 = 0;
    for l1 in v1.iter() {
        let c = v2.iter().filter(|&n| n == l1).count();
        sum2 += l1 * c as i32;
    }
    println!("Part 2: {:?}", sum2);
}
