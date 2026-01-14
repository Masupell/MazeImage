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
        let grid = create_maze_2(None, 0.1, grid_config);//create_maze(None, 0.1);
        let lines = compute_wall_lines(&grid, grid_config.grid_width, grid_config.grid_height, grid_config.cell_size, grid_config.offset);

        Maze
        {
            grid,
            lines,
            solver: Solver::new(1378, 100, grid_config),
            start: 1378,
            end: 100,
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
        // self.draw_solver(grid_config);
        self.draw_ends(grid_config);
        self.draw_maze();
    }

    fn draw_maze(&self)
    {
        // let cell_size = CELL_SIZE as f32;

        // for (i, draw) in self.grid.iter().enumerate()
        // {
        //     if !draw { continue; }

        //     let x = (i % GRID_WIDTH) as f32 * cell_size;
        //     let y = (i / GRID_WIDTH) as f32 * cell_size;
        //     draw_rectangle(x, y, cell_size, cell_size, Color::new(0.8, 0.8, 0.8, 1.0));
        // }

        // let cell_size: f32 = 20.0;
        // let wall_thickness: f32 = 2.0;

        // for (i, cell) in self.grid.iter().enumerate()
        // {
        //     let x_idx = i % 50;
        //     let y_idx = i / 50;
        //     let x = x_idx as f32 * cell_size;
        //     let y = y_idx as f32 * cell_size;

        //     if cell.up
        //     {
        //         draw_line(x, y, x + cell_size, y, wall_thickness, WHITE);
        //     }
        //     if cell.down
        //     {
        //         draw_line(x, y + cell_size, x + cell_size, y + cell_size, wall_thickness, WHITE);
        //     }
        //     if cell.left
        //     {
        //         draw_line(x, y, x, y + cell_size, wall_thickness, WHITE);
        //     }
        //     if cell.right
        //     {
        //         draw_line(x + cell_size, y, x + cell_size, y + cell_size, wall_thickness, WHITE);
        //     }
        // }

        for line in self.lines.iter()
        {
            draw_line(line.x0, line.y0, line.x1, line.y1, 2.0, WHITE);
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
        // if timer.elapsed() >= *time_stop && self.started
        // {
        //     if !self.solver.found
        //     {
        //         self.solver.step(&self.grid);
        //     }
        //     else 
        //     {
        //         self.solver.reconstruction_step();
        //     }
            
        //     *timer = Instant::now();
        // }
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
                    let x = (i % grid_width) as f32 * cell_size;
                    let y = (i / grid_width) as f32 * cell_size;
                    draw_rectangle(x, y, cell_size, cell_size, Color::new(0.4, 0.8, 0.4, 1.0));
                }
            }
        }

        for i in self.solver.final_path.iter()
        {
            let x = (i % grid_width) as f32 * cell_size;
            let y = (i / grid_width) as f32 * cell_size;
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
        // self.grid = create_maze(grid_input, threshold);
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

// fn random_start() -> usize
// {
//     let x = gen_range(0, GRID_WIDTH/2) * 2+1;
//     let y = gen_range(0, GRID_HEIGHT/2) * 2+1;

//     y * GRID_WIDTH + x
// }

// // Might keep this as hashset later, as it would be faster for the maze creation algorithm
// fn get_all_walls(protected: &Vec<bool>) -> Vec<usize>
// {
//     let mut wall_set: HashSet<usize> = HashSet::new();

//     for idx in 0..GRID_SIZE
//     {
//         if !protected[idx] { continue; }
        
//         for ring1 in all_sides(idx, GRID_WIDTH, GRID_SIZE) //buffer around it
//         {
//             for ring2 in sides(ring1, GRID_WIDTH, GRID_SIZE) //actual walls
//             {
//                 if !protected[ring2]
//                 {
//                     wall_set.insert(ring2);
//                 }
//             }
//         }
//     }

//     wall_set.into_iter().collect()
// }

// fn create_maze(grid_input: Option<Vec<bool>>, threshold: f32) -> Vec<bool>
// {
//     let mut protected = vec![false; GRID_SIZE];
    
//     let mut grid = if grid_input.is_some() { grid_input.unwrap() } else { vec![false; GRID_SIZE] };

//     let mut regions: Vec<usize> = vec![0; GRID_SIZE]; // Region 0 is no region
//     let mut next_region_id: usize = 1;
    
//     for (idx, &input) in grid.iter().enumerate()
//     {
//         protected[idx] = input;
//     }
//     protected = expand_protection_zone(&protected);

//     let mut walls =  get_all_walls(&protected);

//     if walls.is_empty()
//     {
//         let start = loop
//         {
//             let s = random_start();
//             if !protected[s]
//             {
//                 break s;
//             }
//         };
//         walls = sides(start, GRID_WIDTH, GRID_SIZE);
//         grid[start] = true;
//     }
//     else 
//     {
//         let temp_copy = walls.clone();
//         for (i, &cell) in temp_copy.iter().enumerate()
//         {
//             walls.remove(i);
//             grid[cell] = true;
//             let neighbours = sides(cell, GRID_WIDTH, GRID_SIZE);
//             for &side in neighbours.iter()
//             {
//                 if !protected[side]
//                 {
//                     walls.push(side);
//                 }
//             }
//         }
//     }

//     while !walls.is_empty()
//     {        
//         let idx = gen_range(0, walls.len());
//         let cell = walls[idx];

//         let x = cell % GRID_WIDTH;
//         let y = cell / GRID_WIDTH;

//         let cell_one_idx;
//         let cell_two_idx;
//         let cell_one;
//         let cell_two;

//         if y % 2 == 1 && x % 2 == 0 // y is odd, x is even, means cells are to the left and right
//         {
//             if x == 0 || x+1 >= GRID_WIDTH { walls.remove(idx); continue; }
//             cell_one_idx = y * GRID_WIDTH + x-1;
//             cell_two_idx = y * GRID_WIDTH + x+1;
//             cell_one = grid[cell_one_idx];
//             cell_two = grid[cell_two_idx];
//         }
//         else if y % 2 == 0 && x % 2 == 1 // y is even, x is odd, means cells are up and down
//         {
//             if y == 0 || y+1 >= GRID_HEIGHT { walls.remove(idx); continue; }
//             cell_one_idx = (y-1) * GRID_WIDTH + x;
//             cell_two_idx = (y+1) * GRID_WIDTH + x;
//             cell_one = grid[cell_one_idx];
//             cell_two = grid[cell_two_idx];
//         }
//         else { walls.remove(idx); continue; }

//         if cell_one != cell_two // If only one is true (visited)
//         {
//             let (visited, unvisited) = if cell_one { (cell_one_idx, cell_two_idx) } else { (cell_two_idx, cell_one_idx) };

//             //Checks if it is connected to another region, if not: Adds cell-idx to new region
//             if protected[unvisited]
//             {
//                 let connected_cells = flood_fill_connected(visited, &grid, &protected, GRID_WIDTH, GRID_SIZE);

//                 // let has_region = connected_cells.iter().any(|&c| regions[c] != 0);

//                 let mut region_id = 0;
//                 let mut connects_to_region = false;

//                 for &c in &connected_cells
//                 {
//                     let r = regions[c];
//                     if r != 0
//                     {
//                         if region_id == 0
//                         {
//                             region_id = r;
//                         }
//                         else if region_id != r
//                         {
//                             connects_to_region = true;
//                             break;
//                         }
//                     }
//                 }

//                 if connects_to_region//has_region
//                 {
//                     walls.remove(idx);
//                     continue;
//                 }

//                 for &c in connected_cells.iter()
//                 {
//                     regions[c] = next_region_id;
//                 }
//                 next_region_id += 1;
//             }
//             grid[cell] = true;
//             grid[unvisited] = true;

//             let new_neighbours = sides(unvisited, GRID_WIDTH, GRID_SIZE);
//             for n in new_neighbours
//             {
//                 if !walls.contains(&n)
//                 {
//                     walls.push(n);
//                 }
//             }
//         }
//         walls.remove(idx);
//     }

//     grid
// }

// // Flood Search, like the solver does currently
// fn flood_fill_connected(start_cell: usize, grid: &Vec<bool>, protected: &Vec<bool>, grid_width: usize, grid_size: usize) -> Vec<usize>
// {
//     let mut connected = Vec::new();
//     let mut visited = vec![false; grid_size];
//     let mut stack = vec![start_cell];
    
//     while let Some(cell) = stack.pop()
//     {
//         if visited[cell] || !grid[cell] || protected[cell]
//         {
//             continue;
//         }

//         visited[cell] = true;
//         connected.push(cell);

//         let neighbours = all_sides(cell, grid_width, grid_size);
//         for &neighbour in neighbours.iter()
//         {
//             if !visited[neighbour] && grid[neighbour] &&!protected[neighbour]
//             {
//                 stack.push(neighbour);
//             }
//         }
//     }

//     connected
// }

// fn expand_protection_zone(protected: &Vec<bool>) -> Vec<bool>
// {
//     let mut expanded = protected.clone();
//     for idx in 0..GRID_SIZE
//     {
//         if !protected[idx] { continue; }

//         for n in all_sides(idx, GRID_WIDTH, GRID_SIZE)
//         {
//             expanded[n] = true;
//         }
//     }
//     expanded
// }

// fn all_sides(pos: usize, width: usize, max: usize) -> Vec<usize>
// {
//     let mut neighbours = Vec::new();

//     let x = pos % width;
//     let y = pos / width;
//     let max_y = max / width;

//     if x > 0 { neighbours.push(pos - 1); }
//     if x < width - 1 { neighbours.push(pos + 1); }
//     if y > 0 { neighbours.push(pos - width); }
//     if y < max_y-1 { neighbours.push(pos + width); }

//     neighbours
// }


#[derive(Clone)]
pub struct Cell
{
    up: bool,
    down: bool,
    left: bool,
    right: bool
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

fn create_maze_2(grid_input: Option<Vec<bool>>, threshold: f32, grid_config: &GridConfig) -> Vec<Cell>
{
    let grid_width = grid_config.grid_width;
    let grid_height = grid_config.grid_height;
    let grid_size = grid_config.grid_size;

    let mut grid = vec![Cell::new(); grid_size];
    let mut visited = vec![false; grid_size];

    let mut frontier_set: HashSet<(usize, Dir)> = HashSet::new();
    let mut frontier_vec: Vec<(usize, Dir)> = Vec::new();

    let start = random_start_2(grid_width, grid_height);
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
            // Input path checking here, later

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

    grid
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum Dir { Up, Down, Left, Right}

fn random_start_2(width: usize, height: usize) -> usize
{
    let x = gen_range(0, width);
    let y = gen_range(0, height);

    y * width + x
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
            lines.push(Line { x0: x0.round(), y0: y.round(), x1: x1.round(), y1: y.round() });
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