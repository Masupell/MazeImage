use std::time::{Duration, Instant};

use macroquad::prelude::*;

use crate::maze::*;

pub mod image;
pub mod maze;

fn window_config() -> Conf
{
    Conf
    {
        window_title: "Maze Image".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() 
{
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