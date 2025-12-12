use std::{collections::VecDeque, time::{Duration, Instant}};

use macroquad::{prelude::*, rand::gen_range};

pub mod image;

const CELLS_X: usize = 25;
const GRID_WIDTH: usize = 2*CELLS_X+1;
const CELL_SIZE: usize = 1280/GRID_WIDTH;
const GRID_HEIGHT: usize = 720/CELL_SIZE;
const GRID_SIZE: usize = GRID_WIDTH*GRID_HEIGHT;

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
    println!("GridWidth: {}, GridHeight: {}", GRID_WIDTH, GRID_HEIGHT);
    println!("CellSize: {}", CELL_SIZE);

    let mut grid = vec![false; GRID_SIZE];

    let start = GRID_WIDTH+1;
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

    let mut timer = Instant::now();
    let time_stop = Duration::from_millis(10);

    let mut started = false;

    let mut solver = Solver::new(1378, 100); // Bottom-left to Top-right

    let mut start = 1378;
    let mut end = 100;

    loop 
    {
        clear_background(Color::new(0.164705882, 0.164705882, 0.164705882, 1.0));

        let cell_size_f = CELL_SIZE as f32;

        for (i, draw) in grid.iter().enumerate()
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
            start = i;
        }

        if is_mouse_button_released(MouseButton::Right) // End
        {
            let pos = mouse_position();

            let x = (pos.0/CELL_SIZE as f32) as usize;
            let y = (pos.1/CELL_SIZE as f32) as usize;
            let i = y*GRID_WIDTH+x;
            end = i;
        }

        if is_key_released(KeyCode::Enter)
        {
            solver.redo(start, end);
        }

        if is_key_released(KeyCode::Space) { started = !started; }

        if timer.elapsed() >= time_stop && started
        {
            if !solver.found
            {
                solver.step(&grid);
            }
            else 
            {
                solver.reconstruction_step();
            }
            
            timer = Instant::now();
        }

        if !solver.finished
        {
            for i in 0..grid.len()
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

        let start_x = (start % GRID_WIDTH) as f32 * cell_size_f;
        let start_y = (start / GRID_WIDTH) as f32 * cell_size_f;
        draw_rectangle(start_x, start_y, CELL_SIZE as f32, CELL_SIZE as f32, Color::new(0.8, 0.8, 0.4, 1.0));

        let end_x = (end % GRID_WIDTH) as f32 * cell_size_f;
        let end_y = (end / GRID_WIDTH) as f32 * cell_size_f;
        draw_rectangle(end_x, end_y, CELL_SIZE as f32, CELL_SIZE as f32, Color::new(0.8, 0.4, 0.4, 1.0));

        // draw_line(0.0, 0.0, (grid_width*cell_size) as f32, 0.0, 5.0, RED);
        // draw_line((grid_width*cell_size) as f32, 0.0, (grid_width*cell_size) as f32, (grid_height*cell_size) as f32, 5.0, RED);
        // draw_line(0.0, 0.0, 0.0, (grid_height*cell_size) as f32, 5.0, RED);
        // draw_line(0.0, (grid_height*cell_size) as f32, (grid_width*cell_size) as f32, (grid_height*cell_size) as f32, 5.0, RED);

        next_frame().await
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


pub struct Solver // Quite inefficient
{
    pub start: usize,
    pub end: usize,
    pub queue: VecDeque<usize>,
    pub visited: Vec<bool>,
    pub path: Vec<Option<usize>>,
    pub path_pos: usize,
    pub final_path: Vec<usize>,
    pub found: bool,
    pub finished: bool
}

impl Solver
{
    pub fn new(start: usize, end: usize) -> Self
    {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut visited = vec![false; GRID_SIZE];
        visited[start] = true;

        Solver
        {
            start,
            end,
            queue,
            visited,
            path: vec![None; GRID_SIZE],
            path_pos: end,
            final_path: Vec::new(),
            found: false,
            finished: false
        }
    }

    pub fn step(&mut self, grid: &Vec<bool>)
    {
        // if self.found { return; }

        let cell_option = self.queue.pop_front();
        if cell_option.is_none() { println!("Error?"); return; }
        let cell = cell_option.unwrap();

        let neighbours = solver_sides(cell, GRID_WIDTH, GRID_HEIGHT, grid);//sides(cell, GRID_WIDTH, GRID_SIZE);
        let viable: Vec<usize> = neighbours.into_iter().filter(|pos| !self.visited[*pos]).collect(); // All not yet visited cells
        for i in viable
        {
            self.visited[i] = true;
            self.path[i] = Some(cell);
            if i == self.end
            {
                self.found = true;
                self.queue.clear();
                break;
            }
            self.queue.push_back(i);
        }
    }

    pub fn reconstruction_step(&mut self)
    {
        if let Some(new_pos) = self.path[self.path_pos]
        {
            self.path_pos = new_pos;
            self.final_path.push(self.path_pos);
        }
        else { self.finished = true; }
    }

    pub fn redo(&mut self, start: usize, end: usize)
    {
        self.queue.clear(); // Cleared before, but to be sure
        self.queue.push_back(start);
        self.visited.fill(false);
        self.visited[start] = true;
        self.start = start;
        self.end = end;
        self.path_pos = end;
        self.path.fill(None);
        self.final_path.clear();
        self.found = false;
        self.finished = false;
    }
}

fn solver_sides(pos: usize, width: usize, height: usize, grid: &Vec<bool>) -> Vec<usize> 
{
    let mut neighbours = Vec::new();
    let x = pos % width;
    let y = pos / width;

    if x > 0 && grid[pos - 1] { neighbours.push(pos - 1); }
    if x + 1 < width && grid[pos + 1] { neighbours.push(pos + 1); }
    if y > 0 && grid[pos - width] { neighbours.push(pos - width); }
    if y + 1 < height && grid[pos + width] { neighbours.push(pos + width); }

    neighbours
}
