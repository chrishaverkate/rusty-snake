pub const BOARD_WIDTH: i32 = 20; // in number of cells
pub const BOARD_HEIGHT: i32 = 20; // in number of cells

pub const METADATA_HEIGHT: i32 = 0;
pub const FRAME_WIDTH: i32 = BOARD_WIDTH;
pub const FRAME_HEIGHT: i32 = BOARD_HEIGHT + METADATA_HEIGHT;

pub const CELL_SIZE: f64 = 20.0; // in pixels
pub const FOOD_SIZE: f64 = CELL_SIZE / 2.0;
pub const FRAMES_PER_SECOND: u64 = 4; // Effects the speed of the game

pub const COLOR_BG: [f32; 4] = [0.0, 0.80, 0.40, 1.0];
pub const COLOR_SNAKE: [f32; 4] = [0.9, 0.10, 0.10, 1.0];
pub const COLOR_FOOD: [f32; 4] = [0.50, 0.30, 0.0, 1.0];
pub const COLOR_METADATA: [f32; 4] = [0.50, 0.50, 1.0, 1.0];
// const COLOR_TEXT: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
