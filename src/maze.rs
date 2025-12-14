use std::{collections::HashSet, time::{Duration, Instant}};

use macroquad::{prelude::*, rand::gen_range};

use crate::{constants::*, image::run, solver::Solver};

pub struct Maze
{
    pub grid: Vec<bool>,
    pub solver: Solver,
    pub start: usize,
    pub end: usize,
    pub started: bool
}

impl Maze
{

    pub fn new() -> Self
    {
        let mut grid = run();
        let walls = get_all_walls(&grid);
        grid = create_maze(Some(grid), Some(walls));

        Maze
        {
            grid,
            solver: Solver::new(1378, 100),
            start: 1378,
            end: 100,
            started: false
        }
    }

    pub fn update(&mut self, timer: &mut Instant, time_stop: &Duration, block_input: bool)
    {
        
        if !block_input { self.handle_input(); }
        self.update_solver(timer, time_stop);
    }

    pub fn draw(&self)
    {
        self.draw_maze();
        self.draw_solver();
        self.draw_ends();
    }

    fn draw_maze(&self)
    {
        let cell_size = CELL_SIZE as f32;

        for (i, draw) in self.grid.iter().enumerate()
        {
            if !draw { continue; }

            let x = (i % GRID_WIDTH) as f32 * cell_size;
            let y = (i / GRID_WIDTH) as f32 * cell_size;
            draw_rectangle(x, y, cell_size, cell_size, Color::new(0.8, 0.8, 0.8, 1.0));
        }
    }

    fn handle_input(&mut self)
    {
        if is_mouse_button_released(MouseButton::Left) // Start
        {
            let pos = mouse_position();

            let x = (pos.0/CELL_SIZE as f32) as usize;
            let y = (pos.1/CELL_SIZE as f32) as usize;
            let i = y*GRID_WIDTH+x;
            self.start = i;
            self.solver.redo(self.start, self.end);
        }

        if is_mouse_button_released(MouseButton::Right) // End
        {
            let pos = mouse_position();

            let x = (pos.0/CELL_SIZE as f32) as usize;
            let y = (pos.1/CELL_SIZE as f32) as usize;
            let i = y*GRID_WIDTH+x;
            self.end = i;
            self.solver.redo(self.start, self.end);
        }

        if is_mouse_button_released(MouseButton::Middle)
        {
            let pos = mouse_position();
            
            let x = (pos.0/CELL_SIZE as f32) as usize;
            let y = (pos.1/CELL_SIZE as f32) as usize;
            let i = y*GRID_WIDTH+x;

            self.grid[i] = !self.grid[i];
        }

        if is_key_released(KeyCode::Enter)
        {
            self.solver.redo(self.start, self.end);
        }

        if is_key_released(KeyCode::Space) { self.started = !self.started; }
    }

    fn update_solver(&mut self, timer: &mut Instant, time_stop: &Duration)
    {
        if timer.elapsed() >= *time_stop && self.started
        {
            if !self.solver.found
            {
                self.solver.step(&self.grid);
            }
            else 
            {
                self.solver.reconstruction_step();
            }
            
            *timer = Instant::now();
        }
    }

    fn draw_solver(&self)
    {
        let cell_size = CELL_SIZE as f32;
        
        if !self.solver.finished
        {
            for i in 0..self.grid.len()
            {
                if self.solver.visited[i]
                {
                    let x = (i % GRID_WIDTH) as f32 * cell_size;
                    let y = (i / GRID_WIDTH) as f32 * cell_size;
                    draw_rectangle(x, y, cell_size, cell_size, Color::new(0.4, 0.8, 0.4, 1.0));
                }
            }
        }

        for i in self.solver.final_path.iter()
        {
            let x = (i % GRID_WIDTH) as f32 * cell_size;
            let y = (i / GRID_WIDTH) as f32 * cell_size;
            draw_rectangle(x, y, cell_size, cell_size, Color::new(0.4, 0.4, 0.8, 1.0));
        }
    }

    fn draw_ends(&self)
    {
        let cell_size = CELL_SIZE as f32;

        let start_x = (self.start % GRID_WIDTH) as f32 * cell_size;
        let start_y = (self.start / GRID_WIDTH) as f32 * cell_size;
        draw_rectangle(start_x, start_y, CELL_SIZE as f32, CELL_SIZE as f32, Color::new(0.8, 0.8, 0.4, 1.0));

        let end_x = (self.end % GRID_WIDTH) as f32 * cell_size;
        let end_y = (self.end / GRID_WIDTH) as f32 * cell_size;
        draw_rectangle(end_x, end_y, CELL_SIZE as f32, CELL_SIZE as f32, Color::new(0.8, 0.4, 0.4, 1.0));
    }

    pub fn regenerate_maze(&mut self)
    {
        self.grid = create_maze(None, None);
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

// Might keep this as hashset later, as it would be faster for the maze creation algorithm
fn get_all_walls(grid: &Vec<bool>) -> Vec<usize>
{
    let mut wall_set: HashSet<usize> = HashSet::new();
    
    for y in 0..GRID_HEIGHT
    {
        if y % 2 == 0 { continue; }
        for x in 0..GRID_WIDTH
        {
            if x % 2 == 0 { continue; }
            let idx = y * GRID_WIDTH + x;
            if !grid[idx] { continue; }

            for &neighbour_idx in sides(idx, GRID_WIDTH, GRID_SIZE).iter()
            {
                if !grid[neighbour_idx]
                {
                    wall_set.insert(neighbour_idx);
                }
            }
        }
    }

    wall_set.into_iter().collect()
}

fn create_maze(grid_input: Option<Vec<bool>>, wall_input: Option<Vec<usize>>) -> Vec<bool>
{
    let mut protected = vec![false; GRID_SIZE];
    
    let mut grid = if grid_input.is_some() { grid_input.unwrap() } else { vec![false; GRID_SIZE] };
    
    let start = random_start();
    grid[start] = true;

    if let Some(wall_inputs) = &wall_input
    {
        for &idx in wall_inputs.iter() 
        {
            protected[idx] = true;
        }
    }

    let mut walls = if wall_input.is_some() { wall_input.unwrap() } else { sides(start, GRID_WIDTH, GRID_SIZE) };

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

            let allow_carve = !protected[cell] || rand::gen_range(0.0, 1.0) < 0.1;
            if allow_carve
            {
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
        }
        walls.remove(idx);
    }
    
    grid
}