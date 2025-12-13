use std::time::{Duration, Instant};

use macroquad::prelude::*;
use macroquad::rand::srand;

pub mod image;
pub mod maze;
pub mod constants;
pub mod solver;
pub mod ui;

use crate::constants::window_config;
use crate::ui::UI;

#[macroquad::main(window_config)]
async fn main() 
{
    srand(miniquad::date::now() as u64);
    let mut maze = maze::Maze::new();

    let mut timer = Instant::now();
    let time_stop = Duration::from_millis(10);

    let ui = UI::new();

    let mut block_input: bool = false;

    loop 
    {
        clear_background(Color::new(0.164705882, 0.164705882, 0.164705882, 1.0));

        maze.update(&mut timer, &time_stop, block_input);
        maze.draw();

        block_input = ui.draw();

        next_frame().await
    }
}