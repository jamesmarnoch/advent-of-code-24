use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufRead, BufReader};
use topo_sort::{SortResults, TopoSort};

fn is_update_good(update: &Vec<String>, rules: &HashMap<String, HashSet<String>>) -> bool {
    let mut update_is_good = true;
    update.iter().for_each(|before| {
        if update_is_good && rules.contains_key(before) {
            let afters = rules.get(before).unwrap();
            afters.iter().for_each(|after| {
                if update_is_good {
                    update_is_good = update_is_good && is_rule_satisfied(update, before, after);
                }
            });
        }
    });
    update_is_good
}

fn is_rule_satisfied(update: &Vec<String>, before: &String, after: &String) -> bool {
    match update.iter().position(|x| x == after) {
        None => true,
        Some(after_pos) => {
            let before_pos = update
                .iter()
                .position(|x| *x == before.to_string())
                .unwrap();
            before_pos < after_pos
        }
    }
}
fn main() {
    let buf_reader = BufReader::new(fs::File::open("input.txt").unwrap());

    let lines = buf_reader.lines();
    let mut rules = HashMap::<String, HashSet<String>>::new();
    let mut updates = Vec::new();
    let mut is_rules = true;
    lines.for_each(|line| {
        let l = line.unwrap();
        if is_rules {
            if l.is_empty() {
                is_rules = false;
            } else {
                let pair: (String, String) = l.split_once('|').into_iter().collect();
                rules.entry(pair.0).or_insert(HashSet::new()).insert(pair.1);
            }
        } else {
            let parts = l.split(',');
            let mut update = Vec::<String>::new();
            parts.for_each(|part| {
                update.push(part.to_string());
            });
            updates.push(update);
        }
    });

    let mut correct_update_centre_page_sum = 0;
    let mut incorrect_updates = Vec::new();
    updates.iter().for_each(|update| {
        let update_good = is_update_good(&update, &rules);
        if update_good {
            correct_update_centre_page_sum += get_centre_page(update);
        } else {
            incorrect_updates.push(update);
        }
    });
    println!(
        "correct update centre page sum: {:?}",
        correct_update_centre_page_sum
    );
    let mut fixed_update_centre_page_sum = 0;
    incorrect_updates.iter().for_each(|update| {
        let mut ts = TopoSort::new();
        update.iter().for_each(|before| {
            if rules.contains_key(before) {
                let v: Vec<_> = rules.get(before).unwrap().into_iter().collect();
                ts.insert(before, v);
            } else {
                ts.insert(before, Vec::new());
            }
        });
        match ts.into_vec_nodes() {
            SortResults::Full(nodes) => {
                let mut x = nodes.iter().map(|x| x.to_string()).collect::<Vec<String>>();
                x.reverse();
                fixed_update_centre_page_sum += get_centre_page(&x);
                let fixed = is_update_good(&x, &rules);
            },
            _ => {}
        };
    });
    println!("Fixed update centre page sum: {:?}", fixed_update_centre_page_sum);
}

fn get_centre_page(update: &Vec<String>) -> i32 {
    let centre_pos = (update.len()) / 2;
    update[centre_pos].parse::<i32>().unwrap()
}
