use std::time::{Duration, Instant};

use macroquad::prelude::*;
use macroquad::rand::srand;

pub mod image;
pub mod maze;
pub mod constants;
pub mod solver;

use crate::constants::window_config;
use crate::solver::Solver;

#[macroquad::main(window_config)]
async fn main() 
{
    srand(miniquad::date::now() as u64);
    let mut maze = maze::Maze::new();

    let mut timer = Instant::now();
    let time_stop = Duration::from_millis(10);

    let mut solver = Solver::new(1378, 100); // Bottom-left to Top-right

    loop 
    {
        clear_background(Color::new(0.164705882, 0.164705882, 0.164705882, 1.0));

        maze.draw(&mut timer, &time_stop, &mut solver);

        next_frame().await
    }
}