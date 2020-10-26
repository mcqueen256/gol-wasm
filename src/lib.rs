mod utils;

use wasm_bindgen::prelude::*;

use getrandom;

use js_sys;

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
pub fn name() -> js_sys::JsString {
    "Nic!".into()
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

#[derive(Eq, PartialEq, Debug)]
enum RleCharacter {
    Number,
    NewLine,
    Dead,
    Alive,
    End,
}

#[derive(Debug, Copy, Clone)]
enum RleToken {
    Number(u32),
    NewLine,
    Dead,
    Alive,
    End,
}

#[derive(Debug, Copy, Clone)]
enum RleElement {
    NewLine(u32),
    Dead(u32),
    Alive(u32),
    End,
}

fn construct_line(line: Vec<RleElement>, width: u32) -> Vec<Cell> {
    let width = width as usize;
    let mut line = line
        .into_iter()
        .map(|element| match element {
            RleElement::Alive(x) => vec![ Cell::Alive; x as usize],
            RleElement::Dead(x) => vec![ Cell::Dead; x as usize],
            _ => panic!("invalid line"),
        })
        .flatten()
        .collect::<Vec<Cell>>();
    if line.len() < width {
        let missing_cells = vec![ Cell::Dead; width as usize - line.len()];
        line.extend(missing_cells);
        line
    } else {
        line.into_iter().take(width).collect()
    }
}

fn load_spaceships(width: u32, height: u32) -> Vec<Cell> {
    let raw_contents = include_str!("../spaceships.rle");

    let xy_line = raw_contents
        .lines()
        .skip(1)
        .next()
        .expect("cannot read ship source.");
    let x: String = xy_line.chars().skip(4).take(3).collect();
    let width_from_file: u32 = x.parse().expect(&format!("x (`{}`) is not a number", x));
    let y: String = xy_line.chars().skip(13).take(3).collect();
    let height_from_file: u32 = y.parse().expect(&format!("y (`{}`) is not a number", y));
    // unsafe {
    //     log!("width_from_file: {}", width_from_file);
    //     log!("height_from_file: {}", height_from_file);
    //     log!("width: {}", width);
    //     log!("height: {}", height);
    // }

    let width_of_grid = if width > width_from_file { width } else { width_from_file };
    let height_of_grid = if height > height_from_file { height } else { height_from_file };
    println!("width: {}, height: {}", width, height);

    

    // let coordinates_string: String = raw_contents
    let mut stream = raw_contents
        .lines()
        .skip(2)
        .flat_map(|line| line.chars())
        .collect::<Vec<char>>()
        .into_iter()
        // Identify adjacent tokens and group them together
        .group_by(|c| match *c {
            'b' => RleCharacter::Dead,
            'o' => RleCharacter::Alive,
            '$' => RleCharacter::NewLine,
            '!' => RleCharacter::End,
             _  => RleCharacter::Number,
        })
        .into_iter()
        // Convert characters into tokens (particularly Number)
        .map(|(key, group)| {
            match key {
                RleCharacter::Dead => RleToken::Dead,
                RleCharacter::Alive => RleToken::Alive,
                RleCharacter::NewLine => RleToken::NewLine,
                RleCharacter::End => RleToken::End,
                RleCharacter::Number => RleToken::Number(group.collect::<String>().parse::<u32>().unwrap()),
            }
        })
        // A super simple parser
        .batching(|it| {
            match it.next() {
                None => None,
                Some(RleToken::Dead) => Some(RleElement::Dead(1)),
                Some(RleToken::Alive) => Some(RleElement::Alive(1)),
                Some(RleToken::NewLine) => Some(RleElement::NewLine(1)),
                Some(RleToken::End) => Some(RleElement::End),
                Some(RleToken::Number(x)) => {
                    match it.next() {
                        None => None,
                        Some(RleToken::Dead) => Some(RleElement::Dead(x)),
                        Some(RleToken::Alive) => Some(RleElement::Alive(x)),
                        Some(RleToken::NewLine) => Some(RleElement::NewLine(x)),
                        Some(RleToken::Number(_)) => panic!("have a number after a number."),
                        Some(RleToken::End) => panic!("cannot have multiple ends"),
                    }
                }
            }
        })
        // Split up by lines
        .collect::<Vec<RleElement>>()
        .into_iter()
        .group_by(|key| match key {
            RleElement::NewLine(_) | RleElement::End => false,
            _ => true
        })
        .into_iter()
        // Oranise into (line, newline_or_end)
        .map(|(_, group)| {
            group.collect::<Vec<RleElement>>()
        })
        .chunks(2)
        .into_iter()
        .map(|mut it| {
            let line = it.next().expect("No next iterator");
            let new_line_or_end = it
                .next()
                .expect("No next iterator")
                .get(0)
                .expect("Missing newline element")
                .clone();
            (line, new_line_or_end)
        })
        // Convert from RleElements to Vec<Cell>
        .map(|(line, newline_or_end): (Vec<RleElement> ,RleElement)| {
            let complete_line = construct_line(line, width);
            let newlines = if let RleElement::NewLine(x) = newline_or_end {
                vec![ Cell::Dead; ((x-1) * width) as usize]
            } else { vec![] };
            let mut section = Vec::new();
            section.extend(complete_line);
            section.extend(newlines);
            section
        })
        .flatten()
        // Truncate
        .take((width * height) as usize)
        // Expand
        .collect::<Vec<Cell>>()
    ;
    stream.extend(vec![ Cell::Dead; (width * height) as usize - stream.len()]);
    stream
}