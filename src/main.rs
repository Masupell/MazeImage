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
use crate::ui::{UI, UiCommand};

#[macroquad::main(window_config)]
async fn main() 
{
    srand(miniquad::date::now() as u64);

    let mut maze = maze::Maze::new();

    let mut timer = Instant::now();
    let time_stop = Duration::from_millis(10);

    let mut ui = UI::new();

    let mut block_input: bool = false;


    // let image: Image = Image::gen_image_color(128, 128, BLACK);
    // let mut texture = Texture2D::from_image(&image);

    let mut draw = false;
    let mut canvas = Canvas::new(1280, 720);

    loop 
    {
        clear_background(Color::new(0.164705882, 0.164705882, 0.164705882, 1.0));

        if is_key_released(KeyCode::Tab)
        {
            draw = !draw;
        }
        
        if draw
        {
            canvas.update(block_input);
            draw_texture_ex(&canvas.texture, 0.0, 0.0, WHITE, DrawTextureParams { dest_size: Some(canvas.get_size()), ..Default::default() });
            //draw_texture_ex(&texture, 0.0, 0.0, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(1280.0, 720.0)), ..Default::default() });
        }
        else 
        {
            maze.update(&mut timer, &time_stop, block_input);
            maze.draw();
        }


        block_input = ui.draw();

        for command in ui.drain_commands()
        {
            match command
            {
                UiCommand::RegenerateMaze { use_image, threshold } => 
                {
                    let (grid, walls) = if use_image
                    {
                        let (grid, image) = crate::image::get_input_grid(ui.get_path());
                        let walls = crate::maze::get_all_walls(&grid);

                        canvas.set_image(image);

                        (Some(grid), Some(walls))
                    }
                    else 
                    {
                        (None, None)
                    };
                    
                    maze.regenerate_maze(grid, walls, threshold);
                }
            }
        }

        next_frame().await
    }
}