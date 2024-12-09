use macroquad::prelude::*;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::io::{BufRead, BufReader};



#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum PositionState {
    Visited,
    NotVisited,
    Obstacle,
    Guard
}
impl Display for PositionState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", match self {
            PositionState::Visited => 'X',
            PositionState::NotVisited => '.',
            PositionState::Obstacle => '#',
            PositionState::Guard => '^'
        })
    }
}
impl GameMap {
    fn new(file: &str) -> GameMap {
        let buf_reader = BufReader::new(fs::File::open(file).unwrap());
        let lines = buf_reader.lines();
        let mut row_count: usize = 0;
        let mut column_count: usize = 0;
        let mut counted_char = false;
        let mut vec_chars = Vec::new();
        lines.for_each(|line| {
            row_count += 1;
            line.unwrap().chars().for_each(|c| {
                if !counted_char {
                    column_count += 1;
                }
                let position = match c {
                    '.' => PositionState::NotVisited,
                    '#' => PositionState::Obstacle,
                    '^' => PositionState::Guard,
                    _ => PositionState::NotVisited
                };
                vec_chars.push(position);
            });
            counted_char = true;
        });
        let game_map = vec_chars;
        let guard_pos = game_map.iter().position(|p| {
            match p {
                PositionState::Guard => true,
                _ => false
            }
        }).unwrap();
        GameMap {
            map: game_map,
            width: column_count,
            height: row_count,
            guard_position: guard_pos,
            guard_coordinate: (
                guard_pos.div_euclid(row_count),
                guard_pos.rem_euclid(row_count),
            )

        }
    }
}


impl Display for GameMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "map: ")?;
        for chunk in self.map.chunks(10) {
            write!(f, "[")?;
            for position in chunk {
                write!(f, "{},", position.to_string())?;
            }
            writeln!(f, "\x08]")?;
        };

        writeln!(
            f,
            "size: ({},{}), guard pos: {}, guard coord ({},{})",
            self.width,
            self.height,
            self.guard_position,
            self.guard_coordinate.0,
            self.guard_coordinate.1
        )?;

        Ok(())
    }
}

struct GameMap {
    map: Vec<PositionState>,
    width: usize,
    height: usize,
    guard_position: usize,
    guard_coordinate: (usize, usize),
}

#[macroquad::main("Day6")]
async fn main() {
    let game_map = GameMap::new("input.txt");
    println!("{}", game_map);
    let squares = game_map.width as i16;

    loop {
        clear_background(DARKGRAY);

        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / squares as f32;
        draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., LIGHTGRAY);

        for i in 1..squares {
            draw_line(
                offset_x,
                offset_y + sq_size * i as f32,
                screen_width() - offset_x,
                offset_y + sq_size * i as f32,
                2.,
                LIGHTGRAY,
            );
        }

        for i in 1..squares {
            draw_line(
                offset_x + sq_size * i as f32,
                offset_y,
                offset_x + sq_size * i as f32,
                screen_height() - offset_y,
                2.,
                LIGHTGRAY,
            );
        }

        for i in 0 ..game_map.map.len() {
            draw_rectangle(
                offset_x + 1. + (i / game_map.width) as f32 * sq_size,
                offset_y + 1. + (i % game_map.width) as f32 * sq_size,
                sq_size - 2.,
                sq_size - 2.,
                match game_map.map[i] {
                    PositionState::Visited => PINK,
                    PositionState::NotVisited => WHITE,
                    PositionState::Obstacle => ORANGE,
                    PositionState::Guard => GREEN,
                }
            );
        }

        next_frame().await
    }
}
