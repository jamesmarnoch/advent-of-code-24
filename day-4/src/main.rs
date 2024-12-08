use ndarray::*;
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::ops::Range;

fn main() {
    let buf_reader = BufReader::new(fs::File::open("input.txt").unwrap());

    let lines = buf_reader.lines();
    let mut row_count: i32 = 0;
    let mut column_count: i32 = 0;
    let mut counted_char = false;
    let mut vec_chars = Vec::new();
    lines.for_each(|line| {
        row_count += 1;
        line.unwrap().chars().for_each(|c| {
            if !counted_char {
                column_count += 1;
            }
            vec_chars.push(c);
        });
        counted_char = true;
    });

    let xmas_puzzle =
        Array2::from_shape_vec((row_count as usize, column_count as usize), vec_chars).unwrap();

    find_xmas(row_count, column_count, &xmas_puzzle);
    find_x_mas(column_count, &xmas_puzzle);
}

fn find_x_mas(column_count: i32, puzzle: &Array2<char>) {
    let re_x_mas = Regex::new(r"(M.S.A.M.S)|(S.S.A.M.M)|(S.M.A.S.M)|(M.M.A.S.S)").unwrap();
    // re_x_mas.push(Regex::new(r"M.S.A.M.S").unwrap());
    // re_x_mas.push(Regex::new(r"S.S.A.M.M").unwrap());
    // re_x_mas.push(Regex::new(r"S.M.A.S.M").unwrap());
    // re_x_mas.push(Regex::new(r"M.M.A.S.S").unwrap());

    let square_size = 3;

    let range_rows = Range {
        start: 0,
        end: column_count - square_size + 1,
    };
    let range_cols = Range {
        start: 0,
        end: column_count - square_size + 1,
    };

    let mut count = 0;

    for i in range_rows {
        for j in range_cols.clone() {
            let square = extract_square(i, j, square_size, &puzzle);
            println!("{:?}", square);
            count += re_x_mas.find_iter(&square).count();
        }
    }

    println!("count X-MAS: {}", count);
}

fn extract_square(row: i32, col: i32, size: i32, puzzle: &Array2<char>) -> String {
    let mut result = String::new();
    for i in row..(row + size) {
        for j in col..(col + size) {
            result.push_str(&puzzle[[i as usize, j as usize]].to_string());
        }
    }
    result
}

fn find_xmas(line_count: i32, column_count: i32, puzzle: &Array2<char>) {
    let mut search_items: Vec<String> = Vec::new();

    puzzle.rows().into_iter().for_each(|row| {
        search_items.push(row.iter().collect());
        search_items.push(row.clone().iter().rev().collect());
    });

    puzzle.columns().into_iter().for_each(|column| {
        search_items.push(column.iter().collect());
        search_items.push(column.clone().iter().rev().collect());
    });

    let diagonal_lines: i32 = puzzle.nrows() as i32 + puzzle.ncols() as i32 - 1;
    let mid_point = (diagonal_lines / 2) + 1;

    let mut items_in_diagonal = 0;
    let mut row_index;
    let mut col_index;
    let mut item = Vec::new();

    for i in 1..diagonal_lines + 1 {
        if i <= mid_point {
            items_in_diagonal += 1;
            for j in 0..items_in_diagonal {
                row_index = i - j - 1;
                col_index = j;
                let c = puzzle[[row_index as usize, col_index as usize]];
                item.push(c);
            }
        } else {
            items_in_diagonal -= 1;
            for j in 0..items_in_diagonal {
                row_index = column_count - 1 - j;
                col_index = i - line_count + j;
                let c = puzzle[[row_index as usize, col_index as usize]];
                item.push(c);
            }
        }
        let mut rev = item.clone();
        rev.reverse();
        search_items.push(item.iter().collect());
        search_items.push(rev.into_iter().collect::<String>());
        item = Vec::new();
    }

    items_in_diagonal = 0;
    for i in 1..diagonal_lines + 1 {
        if i <= mid_point {
            items_in_diagonal += 1;
            for j in 0..items_in_diagonal {
                row_index = j;
                col_index = column_count + j - i;
                let c = puzzle[[row_index as usize, col_index as usize]];
                item.push(c);
            }
        } else {
            items_in_diagonal -= 1;
            for j in 0..items_in_diagonal {
                row_index = i - line_count + j;
                col_index = j;
                let c = puzzle[[row_index as usize, col_index as usize]];
                item.push(c);
            }
        }
        let mut rev = item.clone();
        rev.reverse();
        search_items.push(item.iter().collect());
        search_items.push(rev.into_iter().collect::<String>());
        item = Vec::new();
    }

    let re_xmas = Regex::new(r"XMAS").unwrap();

    let mut count = 0;
    search_items.iter().for_each(|row| {
        count += re_xmas.find_iter(row).count();
    });

    println!("count XMAS: {}", count);
}
