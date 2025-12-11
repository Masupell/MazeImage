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
    let cells_x = 75;
    let cell_size = 1280.0 / cells_x as f32;
    let cells_y = (720.0 / cell_size) as usize;
    let max = cells_x*cells_y;

    let mut grid = vec![false; max];

    let start = 0;

    grid[start] = true;
    let mut frontier = sides(start, cells_x, max);

    while !frontier.is_empty()
    {
        let idx = gen_range(0, frontier.len());
        let cell = frontier[idx];

        let neighbours = sides(cell, cells_x, max);

        let (visited, unvisited): (Vec<usize>, Vec<usize>) = neighbours.iter().partition(|&n| grid[*n]);

        if visited.len() == 1
        {
            grid[cell] = true;
            for n in unvisited 
            {
                if !frontier.contains(&n) 
                {
                    frontier.push(n);
                }
            }
            // frontier.append(&mut unvisited);
        }
        frontier.swap_remove(idx);
    }
    
    loop 
    {
        clear_background(Color::new(0.164705882, 0.164705882, 0.164705882, 1.0));


        for (i, draw) in grid.iter().enumerate()
        {
            if !draw { continue; }

            let x = (i % cells_x) as f32 * cell_size;
            let y = (i / cells_x) as f32 * cell_size;
            draw_rectangle(x, y, cell_size, cell_size, WHITE);
        }


        next_frame().await
    }
}

fn sides(pos: usize, width: usize, max: usize) -> Vec<usize>
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