use image_manipulation::image_shader::image_shader;

use crate::constants::{GRID_SIZE, GRID_WIDTH};

pub fn run() -> Vec<bool>
{
    let mut grid = vec![false; GRID_SIZE];

    let input = image::open("src/res/546727.jpg");
    if input.is_err()
    {
        println!("Error\n(Could be wrong path/does not exist");
        return grid;
    }
    let mut image = input.unwrap();
    image = image.blur(5.0);
    let output = pollster::block_on(image_shader(image, "src/sobel_operator.wgsl"));
    output.save("src/res/output.png").unwrap();


    let width = output.width() as f32;
    let height = output.height() as f32;

    let image_cell_size = (width as f32 / GRID_WIDTH as f32) as u32;
    let image_grid_width = (width / image_cell_size as f32) as u32;
    let image_grid_height = (height / image_cell_size as f32) as u32;

    let pixel_cells = image_cell_size*image_cell_size;
    let threshold = pixel_cells/4;

    // Better for gpu, do that later
    for y in 0..image_grid_height // Grid
    {
        for x in 0..image_grid_width
        {
            let mut white = 0;
            for yy in 0..image_cell_size // Each Cell in the grid
            {
                for xx in 0..image_cell_size
                {
                    let image_x = x*image_cell_size + xx;
                    let image_y = y*image_cell_size + yy;
                    let pixel = output.get_pixel(image_x, image_y);
                    
                    if pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255
                    {
                        white += 1;
                    }
                }
            }

            if white >= threshold
            {
                let idx = y*image_grid_width+x;
                grid[idx as usize] = true;
            }
        }
    }

    grid
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test() {
        run();
    }
}