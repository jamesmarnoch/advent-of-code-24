use macroquad::prelude::*;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum GuardFacing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum PositionState {
    Visited,
    NotVisited,
    Obstacle,
    Guard,
}
impl Display for PositionState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                PositionState::Visited => 'X',
                PositionState::NotVisited => '.',
                PositionState::Obstacle => '#',
                PositionState::Guard => '^',
            }
        )
    }
}
impl GameMap {
    fn position_from_coordinate(&self, coordinate: (i16, i16)) -> i16 {
        (coordinate.1 * self.width) + coordinate.0
    }
    fn is_on_map(&self, coordinate: (i16, i16)) -> bool {
        coordinate.0 >= 0
            && coordinate.0 < self.height
            && coordinate.1 >= 0
            && coordinate.1 < self.width
    }

    fn set_found_obstacle(&mut self, coordinate: (i16, i16)) {
        let found = (coordinate, self.guard_coordinate, self.guard_facing);
        self.found_obstacles.push(found);
    }

    fn is_obstacle(&self, coordinate: (i16, i16)) -> bool {
        self.map[self.position_from_coordinate(coordinate) as usize] == PositionState::Obstacle
    }

    fn set_visited(&mut self, coordinate: (i16, i16)) {
        let new_pos = self.position_from_coordinate(coordinate);
        self.map[new_pos as usize] = PositionState::Visited;
    }

    fn set_guard(&mut self, coordinate: (i16, i16)) {
        let new_pos = self.position_from_coordinate(coordinate);
        self.guard_position = new_pos;
        self.guard_coordinate = coordinate;
        self.map[new_pos as usize] = PositionState::Guard;
    }
    fn facing_to_direction(facing: GuardFacing) -> (i16, i16) {
        let up: (i16, i16) = (0, -1);
        let down: (i16, i16) = (0, 1);
        let right: (i16, i16) = (1, 0);
        let left: (i16, i16) = (-1, 0);

        let direction = match facing {
            GuardFacing::Up => up,
            GuardFacing::Down => down,
            GuardFacing::Left => left,
            GuardFacing::Right => right,
        };

        direction
    }

    fn position_after_move(&self, direction: (i16, i16)) -> (i16, i16) {
        (
            self.guard_coordinate.0 + direction.0,
            self.guard_coordinate.1 + direction.1,
        )
    }

    fn turn(&mut self) {
        self.guard_facing = match self.guard_facing {
            GuardFacing::Up => GuardFacing::Right,
            GuardFacing::Down => GuardFacing::Left,
            GuardFacing::Left => GuardFacing::Up,
            GuardFacing::Right => GuardFacing::Down,
        }
    }

    fn visits_count(&self) -> usize {
        self.map
            .iter()
            .filter(|&p| p == &PositionState::Visited)
            .count()
    }

    fn find_obstacle_positions_for_loops(&self) -> Vec<(i16, i16)> {
        let mut found_positions = Vec::new();
        let mut new_obstacles = Vec::new();
        for i in 0..self.found_obstacles.len() - 3 {
            let found_obstacle = &self.found_obstacles[i];
            let triplet = &self.found_obstacles[i..i + 3];
            let use_col = match found_obstacle.2 {
                GuardFacing::Up => (triplet, GuardFacing::Left),
                GuardFacing::Down => (triplet, GuardFacing::Right),
                GuardFacing::Left => (triplet, GuardFacing::Down),
                GuardFacing::Right => (triplet, GuardFacing::Up),
            };
            found_positions.push(use_col);
        }
        for fp in found_positions {
            let first = fp.0[0];
            let third = fp.0[2];
            let pos = match fp.1 {
                GuardFacing::Left => (first.0.0 - 1, third.1 .1),
                GuardFacing::Right => (first.0.0 + 1, third.1 .1),
                GuardFacing::Up => (third.1.0, first.1.1 - 1),
                GuardFacing::Down => (third.1.0, first.0.1 + 1),
            };
            println!("{:?}", fp);
            println!("{:?}", pos);
            println!();
            new_obstacles.push(pos)
        }
        new_obstacles
    }
    fn move_forward(&mut self) {
        let direction = GameMap::facing_to_direction(self.guard_facing);

        let new_guard_coordinate = self.position_after_move(direction);

        if self.is_on_map(new_guard_coordinate) {
            if !self.is_obstacle(new_guard_coordinate) {
                self.set_visited(self.guard_coordinate);
                self.set_guard(new_guard_coordinate);
                self.set_visited(self.guard_coordinate);
            } else if self.is_on_map(new_guard_coordinate) {
                self.set_found_obstacle(new_guard_coordinate);
                self.turn();
            }
        } else {
            self.set_visited(self.guard_coordinate);
            self.stopped = true;
        }
    }

    fn new(file: &str) -> GameMap {
        let buf_reader = BufReader::new(fs::File::open(file).unwrap());
        let lines = buf_reader.lines();
        let mut row_count = 0;
        let mut column_count = 0;
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
                    _ => PositionState::NotVisited,
                };
                vec_chars.push(position);
            });
            counted_char = true;
        });
        let game_map = vec_chars;
        let guard_pos = game_map
            .iter()
            .position(|p| match p {
                PositionState::Guard => true,
                _ => false,
            })
            .unwrap() as i16;
        GameMap {
            map: game_map,
            width: column_count,
            height: row_count,
            guard_facing: GuardFacing::Up,
            guard_position: guard_pos,
            guard_coordinate: (
                guard_pos.rem_euclid(column_count),
                guard_pos.div_euclid(column_count),
            ),
            stopped: false,
            found_obstacles: Vec::new(),
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
        }

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
    width: i16,
    height: i16,
    guard_facing: GuardFacing,
    guard_position: i16,
    guard_coordinate: (i16, i16),
    stopped: bool,
    found_obstacles: Vec<((i16, i16), (i16, i16), GuardFacing)>,
}

#[macroquad::main("Day6")]
async fn main() {
    let mut game_map = GameMap::new("input2.txt");
    println!("{}", game_map);

    loop {
        let squares = game_map.width;
        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / squares as f32;

        draw_rectangle(
            offset_x,
            offset_y,
            game_size - 20.,
            game_size - 20.,
            LIGHTGRAY,
        );

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

        for i in 0..game_map.map.len() {
            draw_rectangle(
                offset_x + 1. + (i as i16 % game_map.width) as f32 * sq_size,
                offset_y + 1. + (i as i16 / game_map.width) as f32 * sq_size,
                sq_size - 2.,
                sq_size - 2.,
                match game_map.map[i] {
                    PositionState::Visited => PINK,
                    PositionState::NotVisited => WHITE,
                    PositionState::Obstacle => ORANGE,
                    PositionState::Guard => GREEN,
                },
            );
        }
        if !game_map.stopped {
            game_map.move_forward();
        } else {
            let font_size = 30.;
            let text = &format!("Visited Count: {}", game_map.visits_count());
            let text_size = measure_text(text, None, font_size as _, 1.0);
            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                DARKPURPLE,
            );
            let new_obstacles = game_map.find_obstacle_positions_for_loops();
            let mut count = 0;
            for obs_pos in new_obstacles
                .iter()
                .map(|&o| game_map.position_from_coordinate(o))
            {
                count+=1;
                draw_rectangle(
                    offset_x + 1. + (obs_pos % game_map.width) as f32 * sq_size,
                    offset_y + 1. + (obs_pos / game_map.width) as f32 * sq_size,
                    sq_size - 2.,
                    sq_size - 2.,
                    VIOLET,
                );
                draw_text(
                    &format!("{}", count),
                    offset_x + 1. + (obs_pos % game_map.width) as f32 * sq_size,
                    offset_y + 1. + (obs_pos / game_map.width) as f32 * sq_size,
                    font_size,
                    LIME
                );
            }
        }
        next_frame().await
    }
}
