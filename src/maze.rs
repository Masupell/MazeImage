use std::{collections::HashSet, time::{Duration, Instant}};

use macroquad::{prelude::*, rand::gen_range};

use crate::{GridConfig, solver::Solver};

pub struct Maze
{
    pub grid: Vec<Cell>,//Vec<bool>,
    lines: Vec<Line>,
    pub solver: Solver,
    pub start: usize,
    pub end: usize,
    pub started: bool
}

impl Maze
{

    pub fn new(grid_config: &GridConfig) -> Self
    {
        let grid = create_maze(None, 0.1, grid_config);//create_maze(None, 0.1);
        let lines = compute_wall_lines(&grid, grid_config.grid_width, grid_config.grid_height, grid_config.cell_size, grid_config.offset);

        Maze
        {
            grid,
            lines,
            solver: Solver::new(0, 100, grid_config),
            start: 0,
            end: 0,
            started: false
        }
    }

    pub fn update(&mut self, timer: &mut Instant, time_stop: &Duration, block_input: bool, grid_config: &GridConfig)
    {
        
        if !block_input { self.handle_input(grid_config); }
        self.update_solver(timer, time_stop, grid_config);
    }

    pub fn draw(&self, grid_config: &GridConfig)
    {
        self.draw_solver(grid_config);
        self.draw_ends(grid_config);
        self.draw_maze(grid_config);
    }

    fn draw_maze(&self, grid_config: &GridConfig)
    {
        // for line in self.lines.iter()
        // {
        //     draw_line(line.x0, line.y0, line.x1, line.y1, 2.0, WHITE);
        // }
        let cell_size = grid_config.cell_size;

        for (i, cell) in self.grid.iter().enumerate()
        {
            let x_idx = i % grid_config.grid_width;
            let y_idx = i / grid_config.grid_width;
            let x = x_idx as f32 * cell_size + grid_config.offset.0;
            let y = y_idx as f32 * cell_size + grid_config.offset.1;
            if cell.up
            {
                draw_line(x, y, x + cell_size, y, 2.0, WHITE);
            }
            if cell.down
            {
                draw_line(x, y + cell_size, x + cell_size, y + cell_size, 2.0, WHITE);
            }
            if cell.left
            {
                draw_line(x, y, x, y + cell_size, 2.0, WHITE);
            }
            if cell.right
            {
                draw_line(x + cell_size, y, x + cell_size, y + cell_size, 2.0, WHITE);
            }
        }
    }

    fn handle_input(&mut self, grid_config: &GridConfig)
    {
        if is_mouse_button_released(MouseButton::Left) || is_mouse_button_pressed(MouseButton::Left)// Start
        {
            if let Some(i) = mouse_to_cell(grid_config)
            {
                self.start = i;
                self.solver.redo(self.start, self.end);
                println!("Mouse-idx: {}", i);
            }
        }

        if is_mouse_button_released(MouseButton::Right) || is_mouse_button_pressed(MouseButton::Right) // End
        {
            if let Some(i) = mouse_to_cell(grid_config)
            {
                self.end = i;
                self.solver.redo(self.start, self.end);
            }
        }

        if is_mouse_button_released(MouseButton::Middle) || is_mouse_button_pressed(MouseButton::Middle)
        {
            if let Some(_) = mouse_to_cell(grid_config)
            {
                // self.grid[i] = !self.grid[i];
            }
        }

        if is_key_released(KeyCode::Enter) { self.solver.redo(self.start, self.end); }

        if is_key_released(KeyCode::Space) { self.started = !self.started; }
    }


    fn update_solver(&mut self, timer: &mut Instant, time_stop: &Duration, grid_config: &GridConfig)
    {
        if timer.elapsed() >= *time_stop && self.started
        {
            if !self.solver.found
            {
                self.solver.step(&self.grid, grid_config);
            }
            else
            {
                self.solver.reconstruction_step();
            }
            
            *timer = Instant::now();
        }
    }

    fn draw_solver(&self, grid_config: &GridConfig)
    {
        let cell_size = grid_config.cell_size;
        let grid_width = grid_config.grid_width;
        
        if !self.solver.finished
        {
            for i in 0..self.grid.len()
            {
                if self.solver.visited[i]
                {
                    let x = (i % grid_width) as f32 * cell_size + grid_config.offset.0;
                    let y = (i / grid_width) as f32 * cell_size + grid_config.offset.1;
                    draw_rectangle(x, y, cell_size, cell_size, Color::new(0.4, 0.8, 0.4, 1.0));
                }
            }
        }

        for i in self.solver.final_path.iter()
        {
            let x = (i % grid_width) as f32 * cell_size + grid_config.offset.0;
            let y = (i / grid_width) as f32 * cell_size + grid_config.offset.1;
            draw_rectangle(x, y, cell_size, cell_size, Color::new(0.4, 0.4, 0.8, 1.0));
        }
    }

    fn draw_ends(&self, grid_config: &GridConfig)
    {
        let cell_size = grid_config.cell_size;
        let grid_width = grid_config.grid_width;

        let start_x = (self.start % grid_width) as f32 * cell_size + grid_config.offset.0;
        let start_y = (self.start / grid_width) as f32 * cell_size + grid_config.offset.1;
        draw_rectangle(start_x, start_y, cell_size as f32, cell_size as f32, Color::new(0.8, 0.8, 0.4, 1.0));

        let end_x = (self.end % grid_width) as f32 * cell_size + grid_config.offset.0;
        let end_y = (self.end / grid_width) as f32 * cell_size + grid_config.offset.1;
        draw_rectangle(end_x, end_y, cell_size as f32, cell_size as f32, Color::new(0.8, 0.4, 0.4, 1.0));
    }

    pub fn regenerate_maze(&mut self, grid_input: Option<Vec<bool>>, threshold: f32, grid_config: &GridConfig)
    {
        self.grid = create_maze(grid_input, threshold, grid_config);
        self.lines = compute_wall_lines(&self.grid, grid_config.grid_width, grid_config.grid_height, grid_config.cell_size, grid_config.offset);
    }
}


fn mouse_to_cell(grid_config: &GridConfig) -> Option<usize>
{
    let (mx, my) = mouse_position();

    let cell_size = grid_config.cell_size;

    let mx = mx - grid_config.offset.0;
    let my = my - grid_config.offset.1;

    if mx < 0.0 || my < 0.0 { return None; }

    let cx = (mx / cell_size).floor() as isize;
    let cy = (my / cell_size).floor() as isize;

    if cx < 0 || cy < 0 { return None; }

    let cx = cx as usize;
    let cy = cy as usize;

    if cx as usize >= grid_config.grid_width || cy as usize >= grid_config.grid_height { return None; }

    Some(cy * grid_config.grid_width + cx)
}

#[derive(Clone)]
pub struct Cell
{
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool
}

impl Cell
{
    fn new() -> Self
    {
        Cell { up: true, down: true, left: true, right: true }
    }

    fn set_wall(&mut self, dir: &Dir, value: bool)
    {
        match dir
        {
            Dir::Up => self.up = value,
            Dir::Down => self.down = value,
            Dir::Left => self.left = value,
            Dir::Right => self.right = value,
        }
    }
}

fn create_maze(grid_input: Option<Vec<bool>>, threshold: f32, grid_config: &GridConfig) -> Vec<Cell>
{
    let grid_width = grid_config.grid_width;
    let grid_height = grid_config.grid_height;
    let grid_size = grid_config.grid_size;

    let has_path = grid_input.is_some();
    let mut path = vec![false; grid_size];
    if has_path { path = grid_input.unwrap(); }

    let mut grid = vec![Cell::new(); grid_size];
    let mut visited = vec![false; grid_size];

    let mut frontier_set: HashSet<(usize, Dir)> = HashSet::new();
    let mut frontier_vec: Vec<(usize, Dir)> = Vec::new();

    let start = random_start(grid_width, grid_height);
    visited[start] = true;
    for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
    {
        frontier_vec.push((start, dir));
        frontier_set.insert((start, dir));
    }

    while !frontier_vec.is_empty()
    {
        let idx = gen_range(0, frontier_vec.len());
        let (cell, dir) = frontier_vec.swap_remove(idx);
        frontier_set.remove(&(cell, dir));

        let temp = neighbour(cell, &dir, grid_width, grid_size);
        let neighbour = if temp.is_some()
        {
            temp.unwrap()
        }
        else { continue; };

        if visited[cell] != visited[neighbour]
        {            
            grid[cell].set_wall(&dir, false);
            grid[neighbour].set_wall(&opposite(&dir), false);
            visited[neighbour] = true;

            for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
            {
                if frontier_set.insert((neighbour, dir))
                {
                    frontier_vec.push((neighbour, dir));
                }
            }
        }
    }

    // Path carving
    for idx in 0..path.len()
    {
        if !path[idx] { continue; }

        for &neighbor in get_path_neighbours(idx, &path, grid_width, grid_size).iter()
        {
            if let Some(dir) = direction_to_neighbour(idx, neighbor, grid_width, grid_size)
            {
                grid[idx].set_wall(&dir, false);
                grid[neighbor].set_wall(&opposite(&dir), false);
            }
        }
    }

    grid
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum Dir { Up, Down, Left, Right}

fn random_start(width: usize, height: usize) -> usize
{
    let x = gen_range(0, width);
    let y = gen_range(0, height);

    y * width + x
}

fn direction_to_neighbour(first: usize, second: usize, width: usize, max: usize) -> Option<Dir>
{
    for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
    {
        if let Some(neighbour) = neighbour(first, &dir, width, max)
        {
            if neighbour == second
            {
                return Some(dir)
            }
        }
    }
    None
}

fn get_path_neighbours(pos: usize, path_grid: &Vec<bool>, width: usize, max: usize) -> Vec<usize>
{
    let mut neighbours = Vec::new();

    for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
    {
        if let Some(neighbour) = neighbour(pos, &dir, width, max)
        {
            if path_grid[neighbour]
            {
                neighbours.push(neighbour);
            }
        }
    }
    neighbours
}

fn neighbour(pos: usize, dir: &Dir, width: usize, max: usize) -> Option<usize>
{
    let x = pos % width;
    let y = pos / width;
    let max_y = max / width;

    match dir
    {
        Dir::Up => if y > 0 { Some(pos-width) } else { None },
        Dir::Down => if y+1 < max_y { Some(pos+width) } else { None },
        Dir::Left => if x > 0 { Some(pos-1) } else { None },
        Dir::Right => if x+1 < width { Some(pos+1) } else { None },
    }
}

fn opposite(dir: &Dir) -> Dir
{
    match dir
    {
        Dir::Up => Dir::Down,
        Dir::Down => Dir::Up,
        Dir::Left => Dir::Right,
        Dir::Right => Dir::Left,
    }
}



struct Line
{
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32
}

fn compute_wall_lines(grid: &Vec<Cell>, grid_width: usize, grid_height: usize, cell_size: f32, offset: (f32, f32)) -> Vec<Line>
{
    let mut lines = Vec::new();

    for y_idx in 0..grid_height
    {
        let mut x_start: Option<f32> = None;

        for x_idx in 0..grid_width
        {
            let i = y_idx * grid_width + x_idx;
            let cell = &grid[i];

            // First top wall
            if y_idx == 0 && x_idx == 0
            {
                lines.push(Line
                {
                    x0: offset.0,
                    y0: offset.1,
                    x1: offset.0 + grid_width as f32 * cell_size,
                    y1: offset.1
                });
            }

            if cell.up || y_idx == 0
            {
                if x_start.is_none()
                {
                    x_start = Some(offset.0 + x_idx as f32 * cell_size);
                }
            }
            else if let Some(x0) = x_start
            {
                let x1 = offset.0 + x_idx as f32 * cell_size;
                let y = offset.1 + y_idx as f32 * cell_size;
                lines.push(Line { x0: x0.round(), y0: y.round(), x1: x1.round(), y1: y.round() });
                x_start = None;
            }

            if y_idx == grid_height - 1 && cell.down
            {
                let x = offset.0 + x_idx as f32 * cell_size;
                let y = offset.1 + (y_idx + 1) as f32 * cell_size;
                lines.push(Line { x0: x.round(), y0: y.round(), x1: (x + cell_size).round(), y1: y.round() });
            }
        }

        if let Some(x0) = x_start
        {
            let x1 = offset.0 + grid_width as f32 * cell_size;
            let y = offset.1 + y_idx as f32 * cell_size;
            lines.push(Line { x0: x0, y0: y, x1: x1, y1: y });
        }
    }

    for x_idx in 0..grid_width
    {
        let mut y_start: Option<f32> = None;

        for y_idx in 0..grid_height
        {
            let i = y_idx * grid_width + x_idx;
            let cell = &grid[i];

            // First Left wall
            if x_idx == 0 && y_idx == 0
            {
                lines.push(Line
                {
                    x0: offset.0,
                    y0: offset.1,
                    x1: offset.0,
                    y1: offset.1 + grid_height as f32 * cell_size
                });
            }

            if cell.left || x_idx == 0
            {
                if y_start.is_none()
                {
                    y_start = Some(offset.1 + y_idx as f32 * cell_size);
                }
            }
            else if let Some(y0) = y_start
            {
                let y1 = offset.1 + y_idx as f32 * cell_size;
                let x = offset.0 + x_idx as f32 * cell_size;
                lines.push(Line { x0: x.round(), y0: y0.round(), x1: x.round(), y1: y1.round() });
                y_start = None;
            }

            if x_idx == grid_width - 1 && cell.right
            {
                let x = offset.0 + (x_idx + 1) as f32 * cell_size;
                let y = offset.1 + y_idx as f32 * cell_size;
                lines.push(Line { x0: x.round(), y0: y.round(), x1: x.round(), y1: (y + cell_size).round() });
            }
        }

        if let Some(y0) = y_start
        {
            let y1 = offset.1 + grid_height as f32 * cell_size;
            let x = offset.0 + x_idx as f32 * cell_size;
            lines.push(Line { x0: x.round(), y0: y0.round(), x1: x.round(), y1: y1.round() });
        }
    }

    lines
}