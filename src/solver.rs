use std::collections::VecDeque;
use crate::constants::*;

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