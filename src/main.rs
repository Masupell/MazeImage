use std::cell;

use macroquad::{prelude::*, rand::gen_range};

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
    let cells_x = 25;
    let grid_width = 2*cells_x+1;
    let cell_size = 1280/grid_width;
    let grid_height = 720/cell_size;

    println!("GridWidth: {}, GridHeight: {}", grid_width, grid_height);
    println!("CellSize: {}", cell_size);

    let grid_size = grid_width*grid_height;

    let mut grid = vec![false; grid_size];

    let start = grid_width+1;
    grid[start] = true;
    let mut walls = sides(start, grid_width, grid_size);

    while !walls.is_empty()
    {
        let idx = gen_range(0, walls.len());
        let cell = walls[idx];

        let x = cell % grid_width;
        let y = cell / grid_width;

        let cell_one_idx;
        let cell_two_idx;
        let cell_one;
        let cell_two;

        if y % 2 == 1 && x % 2 == 0 // y is odd, x is even, means cells are to the left and right
        {
            if x == 0 || x+1 >= grid_width { walls.remove(idx); continue; }
            cell_one_idx = y * grid_width + x-1;
            cell_two_idx = y * grid_width + x+1;
            cell_one = grid[cell_one_idx];
            cell_two = grid[cell_two_idx];
        }
        else if y % 2 == 0 && x % 2 == 1 // y is even, x is odd, means cells are up and down
        {
            if y == 0 || y+1 >= grid_height { walls.remove(idx); continue; }
            cell_one_idx = (y-1) * grid_width + x;
            cell_two_idx = (y+1) * grid_width + x;
            cell_one = grid[cell_one_idx];
            cell_two = grid[cell_two_idx];
        }
        else { walls.remove(idx); continue; }

        if cell_one != cell_two // If only one is true (visited)
        {
            let unvisited = if cell_one { cell_two_idx } else { cell_one_idx };
            
            grid[cell] = true;
            grid[unvisited] = true;

            let new_neighbours = sides(unvisited, grid_width, grid_size);
            for n in new_neighbours
            {
                if !walls.contains(&n)
                {
                    walls.push(n);
                }
            }
        }
        walls.remove(idx);
    }

    loop 
    {
        clear_background(Color::new(0.164705882, 0.164705882, 0.164705882, 1.0));


        for (i, draw) in grid.iter().enumerate()
        {
            if !draw { continue; }

            let cell_size_f = cell_size as f32;
            let x = (i % grid_width) as f32 * cell_size_f;
            let y = (i / grid_width) as f32 * cell_size_f;
            draw_rectangle(x, y, cell_size_f, cell_size_f, Color::new(0.8, 0.8, 0.8, 1.0));
        }

        // draw_line(0.0, 0.0, (grid_width*cell_size) as f32, 0.0, 5.0, RED);
        // draw_line((grid_width*cell_size) as f32, 0.0, (grid_width*cell_size) as f32, (grid_height*cell_size) as f32, 5.0, RED);
        // draw_line(0.0, 0.0, 0.0, (grid_height*cell_size) as f32, 5.0, RED);
        // draw_line(0.0, (grid_height*cell_size) as f32, (grid_width*cell_size) as f32, (grid_height*cell_size) as f32, 5.0, RED);

        next_frame().await
    }
}

fn sides(pos: usize, width: usize, max: usize) -> Vec<usize> // Unnessesarily returns non-walls as well, but works for now
{
    let mut neighbours = Vec::new();

    let x = pos % width;
    let y = pos / width;
    let max_y = max / width;

    if x > 0 { neighbours.push(pos - 1); }
    if x < width - 1 { neighbours.push(pos + 1); }
    if y > 0 { neighbours.push(pos - width); }
    if y < max_y-1 { neighbours.push(pos + width); }

    neighbours
}