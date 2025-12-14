use std::time::{Duration, Instant};

use macroquad::prelude::*;
use macroquad::rand::srand;

pub mod image;
pub mod maze;
pub mod constants;
pub mod solver;
pub mod ui;

use crate::constants::{GRID_SIZE, window_config};
use crate::ui::{UI, UiCommand};

#[macroquad::main(window_config)]
async fn main() 
{
    println!("GridSize: {}", GRID_SIZE);
    srand(miniquad::date::now() as u64);
    let mut maze = maze::Maze::new();

    let mut timer = Instant::now();
    let time_stop = Duration::from_millis(10);

    let mut ui = UI::new();

    let mut block_input: bool = false;


    let image: Image = Image::gen_image_color(128, 128, BLACK);
    let mut texture = Texture2D::from_image(&image);
    // let texture = Texture2D::
    // texture.update_from_bytes(width, height, bytes);

    loop 
    {
        clear_background(Color::new(0.164705882, 0.164705882, 0.164705882, 1.0));

        maze.update(&mut timer, &time_stop, block_input);
        maze.draw();

        if is_key_down(KeyCode::Tab)
        {
            draw_texture_ex(&texture, 0.0, 0.0, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(1280.0, 720.0)), ..Default::default() });
        }

        block_input = ui.draw();

        for command in ui.drain_commands()
        {
            match command
            {
                UiCommand::RegenerateMaze { grid_input, wall_input, image, threshold } => 
                {
                    if let Some(unwrapped_image) = image
                    {
                        texture = Texture2D::from_image(&unwrapped_image);
                    }
                    maze.regenerate_maze(grid_input, wall_input, threshold);
                }
            }
        }

        next_frame().await
    }
}