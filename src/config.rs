use crate::universe;

use wasm_bindgen::prelude::*;

const DEFAULT_PADDING: u32 = 0;
const DEFAULT_CELL_SIZE: u32 = 5;

/// The input data from which the universe will be constructed with. 
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
pub struct UniverseConfig {
    input: UniverseInput,
    padding: u32,
    cell_size: u32,
    override_size: Option<(u32, u32)>,
    edge_rule: EdgeRule,
}

#[wasm_bindgen]
impl UniverseConfig {
    /// Create a new UniverseBuilder with default parameters.
    /// 
    /// ```
    /// let config = UniverseConfig::new();
    /// let universe = config.configure();
    /// ```
    pub fn new() -> Self {
        Self {
            input: UniverseInput::Random,
            padding: DEFAULT_PADDING,
            cell_size: DEFAULT_CELL_SIZE,
            override_size: None,
            edge_rule: EdgeRule::Wrap,
        }
    }

    pub fn configure(self) -> universe::Universe {
        universe::Universe::from(self)
    }
}