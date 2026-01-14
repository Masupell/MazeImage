use std::collections::VecDeque;

use crate::{GridConfig, maze::Cell};

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
    pub fn new(start: usize, end: usize, grid_config: &GridConfig) -> Self
    {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut visited = vec![false; grid_config.grid_size];
        visited[start] = true;

        Solver
        {
            start,
            end,
            queue,
            visited,
            path: vec![None; grid_config.grid_size],
            path_pos: end,
            final_path: Vec::new(),
            found: false,
            finished: false
        }
    }

    pub fn step(&mut self, grid: &Vec<Cell>, grid_config: &GridConfig)
    {
        // if self.found { return; }

        let cell_option = self.queue.pop_front();
        if cell_option.is_none() { self.found = true; self.finished = true; println!("Error?"); return; }
        let cell = cell_option.unwrap();

        let neighbours = solver_sides(cell, grid_config.grid_width, grid_config.grid_height, grid);
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

fn solver_sides(pos: usize, width: usize, height: usize, grid: &Vec<Cell>) -> Vec<usize>
{
    let mut neighbours = Vec::new();
    let x = pos % width;
    let y = pos / width;
    let cell = &grid[pos];

    if x > 0 && !cell.left { neighbours.push(pos - 1); }
    if x + 1 < width && !cell.right { neighbours.push(pos + 1); }
    if y > 0 && !cell.up { neighbours.push(pos - width); }
    if y + 1 < height && !cell.down { neighbours.push(pos + width); }

    neighbours
}