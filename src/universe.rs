use crate::utils;
use crate::config;
use crate::rle_loader;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use getrandom;

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
    canvas: Option<web_sys::HtmlCanvasElement>,
    canvas_cell:Option<web_sys::HtmlCanvasElement>,
    config: config::UniverseConfig,
    visible_rows: u32,
    visible_columns: u32,
    visible_row_start_position: u32,
    visible_column_start_position: u32,
}

/// Keep track of count of rows and columns
struct RowColCount {
    rows: u32,
    cols: u32,
}

impl Universe {
    fn canvas_width(&self) -> u32 {
        if let Some(canvas) = &self.canvas {
            canvas.width()
        } else {
            0
        }
    }

    fn canvas_height(&self) -> u32 {
        if let Some(canvas) = &self.canvas {
            canvas.height()
        } else {
            0
        }
    }

    fn calculate_visible_grid_size(&self) -> RowColCount {
        let canvas_width = self.canvas.as_ref().unwrap().width();
        let canvas_height = self.canvas.as_ref().unwrap().height();
        let line_width = self.config.border_width;
        let cell_width = self.config.get_cell_size();
        let cell_height = self.config.get_cell_size();

        let visible_columns = if self.config.allow_overflow {
            (canvas_width + line_width) / (cell_width + line_width)
        } else {
            let columns = (canvas_width + line_width) as f64 / (cell_width + line_width) as f64;
            columns.ceil() as u32
        };
        log!("visible_columns: {}", visible_columns);

        let visible_rows = if self.config.allow_overflow {
            (canvas_height + line_width) / (cell_height + line_width)
        } else {
            let rows = (canvas_height + line_width) as f64 / (cell_height + line_width) as f64;
            rows.ceil() as u32
        };
        log!("visible_rows: {}", visible_rows);


        RowColCount {
            rows: visible_rows,
            cols: visible_columns,
        }
    }

    // fn visible_grid_width(&self) -> u32 {

    // }

    /// Called when a canvas is available
    fn build(&mut self) {
        // calculate the visibility of 
        let row_col_count = self.calculate_visible_grid_size();
        self.visible_rows = row_col_count.rows;
        self.visible_columns = row_col_count.cols;

        let padding = self.config.get_padding();

        if let Some((w, h)) = self.config.get_override_size() {
            self.width = w + 2 * padding;
            self.height = h + 2 * padding;
            if self.width < self.visible_columns {
                self.visible_columns = self.width;
            }
            if self.height < self.visible_rows {
                self.visible_rows = self.height;
            }
            log!("Overriding visible_columns ({}) visible_rows({})", self.visible_columns, self.visible_rows);
        } else {
        }


        self.visible_row_start_position = (self.height - self.visible_rows) / 2;
        self.visible_column_start_position = (self.width - self.visible_columns) / 2;

        // Generate random cells
        let mut rand_cells = vec![0u8; (self.width * self.height) as usize];
        getrandom::getrandom(&mut rand_cells[..]).expect("random cell generation failed");
        self.cells = rand_cells.into_iter().map(|cell| if cell%2==0 {Cell::Dead} else {Cell::Alive}).collect();


        // self.cells = vec![Cell::Dead; (self.width * self.height) as usize];
        // let cells = rle_loader::load_spaceships(self.width, self.height);
    }

    /// Check if the cell is within the visibility bounding box.
    fn is_visible(&self, row: u32, col: u32) -> bool {
        let row_lower = self.visible_row_start_position;
        let row_upper = row_lower + self.visible_rows; // up to but not including
        let col_lower = self.visible_column_start_position;
        let col_upper = col_lower + self.visible_columns; // up to but not including
        // log!("row_lower = {}", row_lower);
        // log!("row_upper = {}", row_upper);
        // log!("col_lower = {}", col_lower);
        // log!("col_upper = {}", col_upper);
        // log!("row = {}", row);
        // log!("col = {}", col);

        (row_lower <= row && row < row_upper) && (col_lower <= col && col < col_upper)
    }

    fn translate_to_visible_row_col(&self, row: u32, col:u32) -> (u32, u32) {
        (row - self.visible_row_start_position, col - self.visible_column_start_position)
    }

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
    /// Create a new Universe with default parameters.
    /// 
    /// ```
    /// let universe = Universe::new();
    /// ```
    pub fn new() -> Self {
        utils::set_panic_hook();
        Universe {
            canvas: None,
            config: config::UniverseConfig::new(),
            width: 0,
            height: 0,
            cells: vec![],
            visible_rows: 0,
            visible_columns: 0,
            visible_row_start_position: 0,
            visible_column_start_position: 0,
        }
    }

    pub fn from(conf: config::UniverseConfig) -> Self {
        utils::set_panic_hook();
        Universe {
            canvas: None,
            config: conf,
            width: 0,
            height: 0,
            cells: vec![],
            visible_columns: 0,
            visible_rows: 0,
            visible_row_start_position: 0,
            visible_column_start_position: 0,
        }
    }
    
    /// Connects a Canvas DOM reference to the Universe and constructs the
    /// internal data structures.
    pub fn connect_canvas(&mut self, canvas: web_sys::HtmlCanvasElement) {
        log!("{:?}", canvas);
        self.canvas = Some(canvas);
        self.build();
        log!("width: {}, height: {}", self.canvas_width(), self.canvas_height());
    }

    pub fn draw(&self) {
        if let Some(canvas) = &self.canvas {
            let context: web_sys::CanvasRenderingContext2d = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            use std::f64;
            // Draw the outer circle.
            // let cell_col_count = if self.config.allow_overlap {
            //     0
            // } else {

            // }


            let vis_col_count = self.visible_columns as f64;
            let vis_row_count = self.visible_rows as f64;
            let line_width = self.config.line_width as f64;
            let border_width = self.config.border_width as f64;
            let cell_size = self.config.get_cell_size() as f64;


            let visible_grid_width = vis_col_count * (cell_size + line_width) - line_width;
            let visible_grid_height = vis_row_count * (cell_size + line_width) - line_width;

            // log!("vis_gid_wid {:.2}", visible_grid_width);
            // log!("vis_gid_hei {:.2}", visible_grid_height);

            // calculate offsets
            let x_offset: f64  = ((canvas.width() as f64 - visible_grid_width) / 2.0).floor();
            let y_offset: f64  = ((canvas.height() as f64 - visible_grid_height) / 2.0).floor();

            // log!("offsets x={:.2}, y={:.2}", x_offset, y_offset);

            // draw border
            context.begin_path();
            context.set_stroke_style(&JsValue::from(self.config.get_line_color()));
            context.set_line_width(border_width);
            context.rect(
                x_offset - border_width / 2.0,
                y_offset - border_width / 2.0,
                visible_grid_width + border_width,
                visible_grid_height + border_width
            );
            context.stroke();

            if self.config.allow_overflow {

            } else {

            }

            // draw dividing lines
            context.begin_path();
            context.set_stroke_style(&JsValue::from(self.config.get_line_color()));
            context.set_line_width(line_width);
            for i in 1..(self.visible_columns) {
                context.move_to(
                    x_offset + line_width / 2.0 + i as f64 * (cell_size + line_width) - line_width,
                    y_offset,
                );
                context.line_to(
                    x_offset + line_width / 2.0 + i as f64 * (cell_size + line_width) - line_width,
                    y_offset + visible_grid_height,
                );
                
            }
            for i in 1..(self.visible_rows) {
                context.move_to(
                    x_offset,
                    y_offset + line_width / 2.0 + i as f64 * (cell_size + line_width) - line_width,
                );
                context.line_to(
                    x_offset + visible_grid_width,
                    y_offset + line_width / 2.0 + i as f64 * (cell_size + line_width) - line_width,
                );
                
            }
            context.stroke();

            // // draw cells
            // context.begin_path();
            // context.set_fill_style(&JsValue::from(self.config.get_cell_alive_color()));
            // for col in 0..self.visible_columns {
            //     for row in 0..self.visible_rows {
            //         context.fill_rect(
            //             x_offset + col as f64 * (cell_size + line_width),
            //             y_offset + row as f64 * (cell_size + line_width),
            //             cell_size,
            //             cell_size
            //         );
            //     }
            // }
            // context.stroke();

            // draw all dead cells
            context.begin_path();
            context.set_fill_style(&JsValue::from(self.config.get_cell_dead_color()));
            for col in 0..self.width {
                for row in 0..self.height {
                    if ! self.is_visible(row, col) {
                        continue;
                    }
                    if *self.cells.get(self.get_index(row, col)).unwrap() == Cell::Alive {
                        continue;
                    }
                    let (row, col) = self.translate_to_visible_row_col(row, col);
                    context.fill_rect(
                        x_offset + col as f64 * (cell_size + line_width),
                        y_offset + row as f64 * (cell_size + line_width),
                        cell_size,
                        cell_size
                    );
                }
            }
            context.stroke();

            // draw all alive cells
            context.begin_path();
            context.set_fill_style(&JsValue::from(self.config.get_cell_alive_color()));
            for col in 0..self.width {
                for row in 0..self.height {
                    if ! self.is_visible(row, col) {
                        continue;
                    }
                    if *self.cells.get(self.get_index(row, col)).unwrap() == Cell::Dead {
                        continue;
                    }
                    let (row, col) = self.translate_to_visible_row_col(row, col);
                    context.fill_rect(
                        x_offset + col as f64 * (cell_size + line_width),
                        y_offset + row as f64 * (cell_size + line_width),
                        cell_size,
                        cell_size
                    );
                }
            }
            context.stroke();
        }
    }

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

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

