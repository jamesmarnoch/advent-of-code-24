use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let buf_reader = BufReader::new(fs::File::open("input2.txt").unwrap());

    let lines = buf_reader.lines();
    let mut rules = Vec::<String>::new();
    let mut updates = Vec::<String>::new();
    let mut is_rules = true;
    lines.for_each(|line|
        {
            let l = line.unwrap();
            if is_rules {
                if l.is_empty()
                {
                    is_rules = false;
                } else {
                    rules.push(l);
                }
            } else {
                updates.push(l);
            }
        });

    println!("Rules: {:?}", rules);
    println!("Updates: {:?}", updates);
}
