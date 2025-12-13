use macroquad::prelude::*;

pub const CELLS_X: usize = 25;
pub const GRID_WIDTH: usize = 2*CELLS_X+1;
pub const CELL_SIZE: usize = 1280/GRID_WIDTH;
pub const GRID_HEIGHT: usize = 720/CELL_SIZE;
pub const GRID_SIZE: usize = GRID_WIDTH*GRID_HEIGHT;

pub fn window_config() -> Conf 
{
    Conf
    {
        window_title: "Maze Image".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}