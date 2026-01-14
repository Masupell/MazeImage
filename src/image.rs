use image::{DynamicImage, GrayImage, ImageBuffer, Luma};
use macroquad::{color::BLACK, texture::Image};

use crate::GridConfig;


pub fn get_grid_from_path(path: &str, grid_config: &GridConfig) -> (Vec<bool>, Image) 
{
    let mut input = match image::open(path) 
    {
        Ok(img) => img,
        Err(e) => 
        {
            println!("Error\n{}", e);
            return (Vec::new(), Image::gen_image_color(16, 16, BLACK));
        }
    };

    input = input.blur(5.0);
    input.to_luma8();
    let output = sobel(&input, 0.05);
    let macroquad_image = luma_to_macroquad_image(&output);

    get_input_grid(&macroquad_image, grid_config)
}

pub fn get_grid_from_image(image: Image, grid_config: &GridConfig) -> (Vec<bool>, Image) 
{
    get_input_grid(&image, grid_config)
}

pub fn get_input_grid(input: &Image, grid_config: &GridConfig) -> (Vec<bool>, Image)
{
    let grid_width = grid_config.grid_width;
    let grid_height = grid_config.grid_height;

    let mut grid: Vec<bool> = vec![false; grid_config.grid_size];

    let image_width = input.width() as usize;
    let image_height = input.height() as usize;

    for gy in 0..grid_height
    {
        let y0 = (gy * image_height) / grid_height;
        let y1 = ((gy + 1) * image_height) / grid_height;

        for gx in 0..grid_width
        {
            let x0 = (gx * image_width) / grid_width;
            let x1 = ((gx + 1) * image_width) / grid_width;

            let mut white = 0;
            for y in y0..y1
            {
                for x in x0..x1
                {
                    let pixel = input.get_pixel(x as u32, y as u32);
                    if pixel.r == 1.0 { white += 1; }
                }
            }

            let cell_pixel_count = (x1 - x0) * (y1 - y0);

            if white * 2 >= cell_pixel_count
            {
                // grid.push(gy * grid_width + gx);
                grid[gy * grid_width + gx] = true;
            }
        }
    }

    (grid, input.clone())
}

// Simple Edge detection
pub fn sobel(img: &DynamicImage, threshold: f32,) -> GrayImage 
{
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    let mut out = GrayImage::new(w, h);

    let gx: [[f32; 3]; 3] = 
    [
        [-1.0, 0.0, 1.0],
        [-2.0, 0.0, 2.0],
        [-1.0, 0.0, 1.0],
    ];

    let gy: [[f32; 3]; 3] = 
    [
        [-1.0, -2.0, -1.0],
        [ 0.0,  0.0,  0.0],
        [ 1.0,  2.0,  1.0],
    ];

    for y in 1..h - 1 
    {
        for x in 1..w - 1 
        {
            let mut sx = 0.0;
            let mut sy = 0.0;

            for ky in 0..3 
            {
                for kx in 0..3 
                {
                    let px = gray.get_pixel(x + kx - 1, y + ky - 1)[0] as f32 / 255.0;

                    sx += gx[ky as usize][kx as usize] * px;
                    sy += gy[ky as usize][kx as usize] * px;
                }
            }

            let edge_strength = sx * sx + sy * sy;

            out.put_pixel(x, y, Luma([ if edge_strength >= threshold { 255 } else { 0 } ]));
        }
    }

    out
}


fn luma_to_macroquad_image(src: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Image 
{
    let (w, h) = src.dimensions();
    let mut img = Image::gen_image_color(w as u16, h as u16, BLACK);

    let dst = &mut img.bytes;

    for (i, pixel) in src.pixels().enumerate() 
    {
        let v = pixel[0];
        let di = i * 4;
        dst[di]     = v;
        dst[di + 1] = v;
        dst[di + 2] = v;
        dst[di + 3] = 255;
    }

    img
}