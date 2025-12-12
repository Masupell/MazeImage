use std::time::{Duration, Instant};

use macroquad::{prelude::*, rand::gen_range};

use crate::{constants::*, solver::Solver};

pub struct Maze
{
    pub grid: Vec<bool>,
    pub start: usize,
    pub end: usize,
    pub started: bool
}

impl Maze
{

    pub fn new() -> Self
    {
        let mut grid = vec![false; GRID_SIZE];
    
        let start = random_start(); //GRID_WIDTH+1;
        grid[start] = true;
        let mut walls = sides(start, GRID_WIDTH, GRID_SIZE);
    
        while !walls.is_empty()
        {
            let idx = gen_range(0, walls.len());
            let cell = walls[idx];
    
            let x = cell % GRID_WIDTH;
            let y = cell / GRID_WIDTH;
    
            let cell_one_idx;
            let cell_two_idx;
            let cell_one;
            let cell_two;
    
            if y % 2 == 1 && x % 2 == 0 // y is odd, x is even, means cells are to the left and right
            {
                if x == 0 || x+1 >= GRID_WIDTH { walls.remove(idx); continue; }
                cell_one_idx = y * GRID_WIDTH + x-1;
                cell_two_idx = y * GRID_WIDTH + x+1;
                cell_one = grid[cell_one_idx];
                cell_two = grid[cell_two_idx];
            }
            else if y % 2 == 0 && x % 2 == 1 // y is even, x is odd, means cells are up and down
            {
                if y == 0 || y+1 >= GRID_HEIGHT { walls.remove(idx); continue; }
                cell_one_idx = (y-1) * GRID_WIDTH + x;
                cell_two_idx = (y+1) * GRID_WIDTH + x;
                cell_one = grid[cell_one_idx];
                cell_two = grid[cell_two_idx];
            }
            else { walls.remove(idx); continue; }
    
            if cell_one != cell_two // If only one is true (visited)
            {
                let unvisited = if cell_one { cell_two_idx } else { cell_one_idx };
                
                grid[cell] = true;
                grid[unvisited] = true;
    
                let new_neighbours = sides(unvisited, GRID_WIDTH, GRID_SIZE);
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

        Maze
        {
            grid,
            start: 1378,
            end: 100,
            started: false
        }
    }

    pub fn input(&mut self)
    {

    }

    pub fn draw(&mut self, timer: &mut Instant, time_stop: &Duration, solver: &mut Solver)
    {
        let cell_size_f = CELL_SIZE as f32;

        for (i, draw) in self.grid.iter().enumerate()
        {
            if !draw { continue; }

            let x = (i % GRID_WIDTH) as f32 * cell_size_f;
            let y = (i / GRID_WIDTH) as f32 * cell_size_f;
            draw_rectangle(x, y, cell_size_f, cell_size_f, Color::new(0.8, 0.8, 0.8, 1.0));
        }

        if is_mouse_button_released(MouseButton::Left) // Start
        {
            let pos = mouse_position();

            let x = (pos.0/CELL_SIZE as f32) as usize;
            let y = (pos.1/CELL_SIZE as f32) as usize;
            let i = y*GRID_WIDTH+x;
            self.start = i;
        }

        if is_mouse_button_released(MouseButton::Right) // End
        {
            let pos = mouse_position();

            let x = (pos.0/CELL_SIZE as f32) as usize;
            let y = (pos.1/CELL_SIZE as f32) as usize;
            let i = y*GRID_WIDTH+x;
            self.end = i;
        }

        if is_key_released(KeyCode::Enter)
        {
            solver.redo(self.start, self.end);
        }

        if is_key_released(KeyCode::Space) { self.started = !self.started; }

        if timer.elapsed() >= *time_stop && self.started
        {
            if !solver.found
            {
                solver.step(&self.grid);
            }
            else 
            {
                solver.reconstruction_step();
            }
            
            *timer = Instant::now();
        }

        if !solver.finished
        {
            for i in 0..self.grid.len()
            {
                if solver.visited[i]
                {
                    let x = (i % GRID_WIDTH) as f32 * cell_size_f;
                    let y = (i / GRID_WIDTH) as f32 * cell_size_f;
                    draw_rectangle(x, y, cell_size_f, cell_size_f, Color::new(0.4, 0.8, 0.4, 1.0));
                }
            }
        }

        for i in solver.final_path.iter()
        {
            let x = (i % GRID_WIDTH) as f32 * cell_size_f;
            let y = (i / GRID_WIDTH) as f32 * cell_size_f;
            draw_rectangle(x, y, cell_size_f, cell_size_f, Color::new(0.4, 0.4, 0.8, 1.0));
        }

        let start_x = (self.start % GRID_WIDTH) as f32 * cell_size_f;
        let start_y = (self.start / GRID_WIDTH) as f32 * cell_size_f;
        draw_rectangle(start_x, start_y, CELL_SIZE as f32, CELL_SIZE as f32, Color::new(0.8, 0.8, 0.4, 1.0));

        let end_x = (self.end % GRID_WIDTH) as f32 * cell_size_f;
        let end_y = (self.end / GRID_WIDTH) as f32 * cell_size_f;
        draw_rectangle(end_x, end_y, CELL_SIZE as f32, CELL_SIZE as f32, Color::new(0.8, 0.4, 0.4, 1.0));

        // draw_line(0.0, 0.0, (grid_width*cell_size) as f32, 0.0, 5.0, RED);
        // draw_line((grid_width*cell_size) as f32, 0.0, (grid_width*cell_size) as f32, (grid_height*cell_size) as f32, 5.0, RED);
        // draw_line(0.0, 0.0, 0.0, (grid_height*cell_size) as f32, 5.0, RED);
        // draw_line(0.0, (grid_height*cell_size) as f32, (grid_width*cell_size) as f32, (grid_height*cell_size) as f32, 5.0, RED);
    }
}

fn sides(pos: usize, width: usize, max: usize) -> Vec<usize>
{
    let mut neighbours = Vec::new();

    let x = pos % width;
    let y = pos / width;
    let max_y = max / width;

    if x > 0 && (y%2==1) { neighbours.push(pos - 1); }
    if x < width - 1 && (y%2==1) { neighbours.push(pos + 1); }
    if y > 0 && (x%2==1){ neighbours.push(pos - width); }
    if y < max_y-1 && (x%2==1) { neighbours.push(pos + width); }

    neighbours
}

fn random_start() -> usize
{
    let x = gen_range(0, GRID_WIDTH/2) * 2+1;
    let y = gen_range(0, GRID_HEIGHT/2) * 2+1;

    y * GRID_WIDTH + x
}