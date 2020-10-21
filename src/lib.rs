mod utils;

use wasm_bindgen::prelude::*;

use getrandom;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn living_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_col == 0 && delta_row == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.living_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next
    }

    pub fn new(width: u32, height: u32) -> Universe {
        let cells = load_spaceships(width, height);
        
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

use itertools::Itertools;

fn load_spaceships(width: u32, height: u32) -> Vec<Cell> {
    let raw_contents = include_str!("../../spaceshiptypes.rle");

    let xy_line = raw_contents
        .lines()
        .skip(1)
        .next()
        .expect("cannot read ship source.");
    let x: String = xy_line.chars().skip(4).take(3).collect();
    let width_from_file: u32 = x.parse().expect(&format!("x (`{}`) is not a number", x));
    let y: String = xy_line.chars().skip(13).take(3).collect();
    let height_from_file: u32 = y.parse().expect(&format!("y (`{}`) is not a number", y));
    unsafe {
        log!("width_from_file: {}", width_from_file);
        log!("height_from_file: {}", height_from_file);
        log!("width: {}", width);
        log!("height: {}", height);
    }

    let width_of_grid = if width > width_from_file { width } else { width_from_file };
    let height_of_grid = if height > height_from_file { height } else { height_from_file };
    println!("width: {}, height: {}", width, height);

    #[derive(Eq, PartialEq, Debug)]
    enum RleGroups {
        Number,
        Dead,
        Alive,
    }

    #[derive(Debug, Copy, Clone)]
    enum Rle {
        Number(u32),
        Dead,
        Alive,
    }

    let coordinates_string: String = raw_contents
        .lines()
        .skip(2)
        .flat_map(|line| line.chars())
        .collect::<String>();
    let token_stream: Vec<Rle> = coordinates_string
        .split("$")
        .map(|line| {
            let unprocessed_groups = line
                .chars()
                .filter(|&c| c != '!')
                // .map(|c| c.clone())
                .group_by(|c| match *c {
                    'b' => RleGroups::Dead,
                    'o' => RleGroups::Alive,
                     _  => RleGroups::Number,
                });
            let token: Vec<Rle> = unprocessed_groups
                .into_iter()
                .map(|(key, group)| {
                    match key {
                        RleGroups::Dead => Rle::Dead,
                        RleGroups::Alive => Rle::Alive,
                        RleGroups::Number => Rle::Number(group.collect::<String>().parse::<u32>().unwrap())
                    }
                }).collect();
                let tokenized_line_uncomplete: Vec<_> = token
                    .iter()
                    .batching(|it| {
                        match it.next() {
                            None => None,
                            Some(Rle::Dead) => Some(vec![Rle::Dead]),
                            Some(Rle::Alive) => Some(vec![Rle::Alive]),
                            Some(Rle::Number(x)) => {
                                match it.next() {
                                    None => None,
                                    Some(Rle::Dead) => Some(vec![Rle::Dead; *x as usize]),
                                    Some(Rle::Alive) => Some(vec![Rle::Alive; *x as usize]),
                                    Some(Rle::Number(_)) => panic!("not suppose to be here"),
                                }
                            } 
                        }
                    })
                    .flatten()
                    .collect();
                tokenized_line_uncomplete

        })
        .map(|incomplete_tokened_line| {
            let mut complete_tokened_line: Vec<Rle> = Vec::new();
            if width >= width_from_file {
                let missing_tokens = vec![Rle::Dead; width as usize - incomplete_tokened_line.len()];
                complete_tokened_line.extend(incomplete_tokened_line.iter());
                complete_tokened_line.extend(missing_tokens.iter());
            } else {
                if incomplete_tokened_line.len() < width as usize {
                    let missing_tokens = vec![Rle::Dead; width as usize - incomplete_tokened_line.len()];
                    complete_tokened_line.extend(incomplete_tokened_line.iter());
                    complete_tokened_line.extend(missing_tokens.iter());
                } else {
                    complete_tokened_line.extend(incomplete_tokened_line.iter().take(width as usize));
                }
            }
            complete_tokened_line
        })
        .flatten()
        .collect();
    let array_size = width as usize * height as usize;
    let mut token_stream: Vec<Rle> = token_stream
        .into_iter()
        .take(array_size)
        .collect();
    if token_stream.len() < array_size {
        let missing_tokens = vec![Rle::Dead; array_size - token_stream.len()];
        token_stream.extend(missing_tokens.iter());
    }
    token_stream
        .iter()
        .map(|cell| match cell {
            Rle::Dead => Cell::Dead,
            Rle::Alive => Cell::Alive,
            Rle::Number(_) => panic!("No numbers should be here"),
        })
        .collect()
}
