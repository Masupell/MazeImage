use std::time::{Duration, Instant};

use macroquad::prelude::*;
use macroquad::rand::srand;

pub mod image;
pub mod maze;
pub mod constants;
pub mod solver;
pub mod ui;
pub mod canvas;

use crate::canvas::Canvas;
use crate::constants::window_config;
use crate::ui::{InputImage, UI, UiCommand};

#[macroquad::main(window_config)]
async fn main() 
{
    srand(miniquad::date::now() as u64);

    let mut maze = maze::Maze::new();

    let mut timer = Instant::now();
    let time_stop = Duration::from_millis(10);

    let mut ui = UI::new();

    let mut block_input: bool = false;

    let mut canvas = Canvas::new(1280, 720);

    let mut state = AppState::Maze;

    let mut brush_size = 6.0;
    let mut smoothing = 0.5;

    let mut color = WHITE;

    loop 
    {
        clear_background(Color::new(0.164705882, 0.164705882, 0.164705882, 1.0));

        match state
        {
            AppState::Draw =>
            {
                canvas.update(block_input, brush_size, smoothing, color);
                canvas.draw();
            },
            AppState::Maze =>
            {
                maze.update(&mut timer, &time_stop, block_input);
                maze.draw();
            }
        }

        block_input = ui.draw(&state, &mut brush_size, &mut smoothing, color);
        ui.update();

        for command in ui.drain_commands()
        {
            match command
            {
                UiCommand::RegenerateMaze { use_image, threshold } => 
                {
                    let grid = if use_image == InputImage::Image
                    {
                        let (grid, image) = crate::image::get_grid_from_path(ui.get_path());

                        canvas.set_image(image);

                        Some(grid)
                    }
                    else if use_image == InputImage::Drawing
                    {
                        let (grid, _) = crate::image::get_grid_from_image(canvas.get_image());

                        Some(grid)
                    }
                    else 
                    {
                        None
                    };
                    
                    maze.regenerate_maze(grid, threshold);
                },
                UiCommand::SwitchState(new_state) => state = new_state,
                UiCommand::SwitchColor(new_color) => color = new_color,
                UiCommand::ShowGrid(show) => canvas.show_grid(show),
                UiCommand::SwitchFillMode(new_fill) =>
                {
                    canvas.set_fill(new_fill);
                }
            }
        }

        next_frame().await
    }
}


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AppState
{
    Maze,
    Draw
}