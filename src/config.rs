use crate::universe;

use wasm_bindgen::prelude::*;

const DEFAULT_PADDING: u32 = 0;
const DEFAULT_CELL_SIZE: u32 = 3;

/// The input data from which the universe will be constructed with. 
#[derive(Clone, Debug)]
pub enum UniverseInput {
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
pub struct UniverseConfig {
    // construction parameters
    input: UniverseInput,
    padding: u32,
    cell_size: u32,
    override_size: Option<(u32, u32)>,

    // behaviour
    edge_rule: EdgeRule,

    // styling
    pub lines_enabled: bool,
    pub line_width: u32,
    pub border_width: u32,
    pub allow_overflow: bool,
    line_color: String,
    cell_alive_color: String,
    cell_dead_color: String,
}

impl UniverseConfig {
    pub fn get_input(&self) -> UniverseInput {
        self.input.clone()
    }

    pub fn get_padding(&self) -> u32 {
        self.padding
    }

    pub fn get_cell_size(&self) -> u32 {
        self.cell_size
    }

    pub fn get_override_size(&self) -> Option<(u32, u32)> {
        self.override_size
    }


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

    /// Create a universe grid populated at random.
    pub fn set_random_input(mut self) -> Self {
        self.input = UniverseInput::Random;
        self
    }

    /// Add additional cells to the outside of the universe. If the absolute
    /// size is specified, the padding will added to the outside of the size.
    pub fn set_padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the color of the lines between cells. The color is given as string
    /// that is passed to javascript.
    ///   e.g. color = "red" or color = "#FF0000"
    pub fn set_line_color(mut self, color: &str) -> Self {
        self.line_color = String::from(color);
        self
    }

    /// Set the color of the cells when they are alive. The color is given as
    /// string that is passed to javascript.
    ///   e.g. color = "red" or color = "#FF0000"
    pub fn set_cell_alive_color(mut self, color: &str) -> Self {
        self.cell_alive_color = String::from(color);
        self
    }

    /// Set the color of the cells when they are dead. The color is given as
    /// string that is passed to javascript.
    ///   e.g. color = "red" or color = "#FF0000"
    pub fn set_cell_dead_color(mut self, color: &str) -> Self {
        self.cell_dead_color = String::from(color);
        self
    }

    /// If used, this overrides the universe size to the specified parameters.
    pub fn set_override_size(mut self, width: u32, height: u32) -> Self {
        self.override_size = Some((width, height));
        self
    }

    /// Construct a universe from a configuration.
    pub fn construct(self) -> universe::Universe {
        universe::Universe::from(self)
    }
}

