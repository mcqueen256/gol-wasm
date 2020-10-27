use crate::utils;
use crate::config;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

#[wasm_bindgen]
pub struct Universe {
    // universe_width: u32,
    // universe_height: u32,
    // cells: Vec<Cell>,
    canvas: Option<web_sys::HtmlCanvasElement>,
    config: config::UniverseConfig,
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
        }
    }

    pub fn from(conf: config::UniverseConfig) -> Self {
        utils::set_panic_hook();
        Universe {
            canvas: None,
            config: conf,
        }
    }
    
    /// Connects a Canvas DOM reference to the Universe and constructs the
    /// internal data structures.
    pub fn connect_canvas(&mut self, canvas: web_sys::HtmlCanvasElement) {
        log!("{:?}", canvas);
        self.canvas = Some(canvas);
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
            context.begin_path();
            // Draw the outer circle.
            context
                .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
                .unwrap();

            // Draw the mouth.
            context.move_to(110.0, 75.0);
            context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

            // Draw the left eye.
            context.move_to(65.0, 65.0);
            context
                .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
                .unwrap();

            // Draw the right eye.
            context.move_to(95.0, 65.0);
            context
                .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
                .unwrap();

            context.stroke();
        }
    }
}