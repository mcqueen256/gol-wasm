use crate::universe;

use wasm_bindgen::prelude::*;

const DEFAULT_PADDING: u32 = 0;
const DEFAULT_CELL_SIZE: u32 = 3;

/// The input data from which the universe will be constructed with. 
#[derive(Clone, Debug)]
enum UniverseInput {
    Random,
    RleString(String),
}

/// Defines what happens at the edge of the grid. When `EdgeRule::Wrap` is set,
/// a request for a cell beyond the edge will wrap around to the other side.
/// Otherwise, `EdgeRule::Truncate` is set and cells at the beyond the edge are
/// terminated.
#[wasm_bindgen]
pub enum EdgeRule {
    Wrap,
    Truncate,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct OverrideSize(pub u32,pub u32);



#[derive(Clone, Debug)]
pub struct AString(pub String);

#[wasm_bindgen]
pub struct UniverseConfig {
    input: UniverseInput,
    pub padding: u32,
    pub cell_size: u32,
    pub override_size: Option<OverrideSize>,
    edge_rule: EdgeRule,
    pub lines_enabled: bool,
    pub line_width: u32,
    pub border_width: u32,
    pub allow_overflow: bool,
    line_color: String,
    cell_alive_color: String,
    cell_dead_color: String,
}

impl UniverseConfig {
    // pub fn get_input(&self) -> UniverseInput {
    //     self.input.clone()
    // }

    pub fn get_padding(&self) -> u32 {
        self.padding
    }

    // pub fn get_override_size(&self) -> Option<(u32, u32)> {
    //     self.override_size
    // }


    pub fn get_line_color(&self) -> String {
        self.line_color.clone()
    }
    pub fn get_cell_alive_color(&self) -> String {
        self.cell_alive_color.clone()
    }
    pub fn get_cell_dead_color(&self) -> String {
        self.cell_dead_color.clone()
    }
}

#[wasm_bindgen]
impl UniverseConfig {
    /// Create a new UniverseBuilder with **default** parameters.
    /// 
    /// ```
    /// let config = UniverseConfig::new();
    /// let universe = config.configure();
    /// ```
    pub fn new() -> Self {
        Self {
            input: UniverseInput::RleString(String::from("this")),
            padding: DEFAULT_PADDING,
            cell_size: 10,
            override_size: None,
            edge_rule: EdgeRule::Wrap,
            lines_enabled: true,
            line_width: 2,
            border_width: 4,
            allow_overflow: false,

            line_color: String::from("white"),
            cell_alive_color: String::from("black"),
            cell_dead_color: String::from("white"),
        }
    }

    pub fn set_line_color(mut self, color: &str) -> Self {
        self.line_color = String::from(color);
        self
    }

    pub fn set_cell_alive_color(mut self, color: &str) -> Self {
        self.cell_alive_color = String::from(color);
        self
    }

    pub fn set_cell_dead_color(mut self, color: &str) -> Self {
        self.cell_dead_color = String::from(color);
        self
    }

    pub fn set_override_size(mut self, width: u32, height: u32) -> Self {
        self.override_size = Some(OverrideSize(width, height));
        self
    }

    pub fn configure(self) -> universe::Universe {
        universe::Universe::from(self)
    }
}

