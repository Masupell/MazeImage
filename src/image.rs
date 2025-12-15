use image::{DynamicImage, GrayImage, ImageBuffer, Luma};
use macroquad::{color::BLACK, texture::Image};

use crate::constants::{GRID_HEIGHT, GRID_SIZE, GRID_WIDTH};


pub fn get_grid_from_path(path: &str) -> (Vec<bool>, Image) 
{
    let mut input = match image::open(path) 
    {
        Ok(img) => img,
        Err(_) => 
        {
            println!("Error\n(Could be wrong path/does not exist");
            return (vec![false; GRID_SIZE], Image::gen_image_color(16, 16, BLACK));
        }
    };

    input = input.blur(5.0);
    input.to_luma8();
    let output = sobel(&input, 0.05);
    let macroquad_image = luma_to_macroquad_image(&output);

    get_input_grid(&macroquad_image)
}

pub fn get_grid_from_image(image: Image) -> (Vec<bool>, Image) 
{
    get_input_grid(&image)
}

pub fn get_input_grid(input: &Image) -> (Vec<bool>, Image)
{
    let mut grid = vec![false; GRID_SIZE];

    let src = input;

    let input_width = src.width() as u32;
    let input_height = src.height() as u32;

    let input_aspect = input_width as f32 / input_height as f32;
    let target_aspect = GRID_WIDTH as f32 / GRID_HEIGHT as f32;

    let (extended_width, extended_height, offset_x, offset_y) = if input_aspect > target_aspect
    {
        let new_height = (input_width as f32 / target_aspect).round() as u32;
        let pad_y = ((new_height - input_height) / 2) as i32;
        (input_width, new_height, 0, pad_y)
    }
    else if input_aspect < target_aspect
    {
        let new_width = (input_height as f32 * target_aspect).round() as u32;
        let pad_x = ((new_width - input_width) / 2) as i32;
        (new_width, input_height, pad_x, 0)
    }
    else
    {
        (input_width, input_height, 0, 0)
    };

    let mut extended = Image::gen_image_color(extended_width as u16, extended_height as u16, BLACK);

    for y in 0..input_height 
    {
        for x in 0..input_width 
        {
            let tx = x as i32 + offset_x;
            let ty = y as i32 + offset_y;
            extended.set_pixel(tx as u32, ty as u32, src.get_pixel(x, y));
        }
    }
    let macroquad_image = extended;

    let image_width = src.width() as usize;
    let image_height = src.height() as usize;

    let image_aspect = image_width as f32 / image_height as f32;
    let grid_aspect = GRID_WIDTH as f32 / GRID_HEIGHT as f32;

    let (fit_width, fit_height, offset_x, offset_y) = if image_aspect > grid_aspect
    {
        let height= (GRID_WIDTH as f32 / image_aspect).round() as usize;
        (GRID_WIDTH, height, 0, (GRID_HEIGHT-height)/2)
    }
    else 
    {
        let width = (GRID_HEIGHT as f32 * image_aspect).round() as usize;
        (width, GRID_HEIGHT, (GRID_WIDTH-width)/2, 0)
    };

    // Based on GRID_SIZE now not on image_size
    for gy in 0..GRID_HEIGHT 
    {
        if gy < offset_y || gy >= offset_y + fit_height { continue; }
        
        let y0 = (gy-offset_y) * image_height/fit_height;
        let y1 = (gy+1-offset_y) * image_height/fit_height;

        for gx in 0..GRID_WIDTH 
        {
            if gx < offset_x || gx >= offset_x + fit_width { continue; }
            
            let x0 = (gx-offset_x) * image_width/fit_width;
            let x1 = (gx+1-offset_x) * image_width/fit_width;

            let mut white = 0;
            for y in y0..y1 
            {
                for x in x0..x1 
                {
                    let pixel = src.get_pixel(x as u32, y as u32);
                    if pixel.r == 1.0 { white += 1; }
                }
            }

            let cell_pixel_count = (x1 - x0) * (y1 - y0);
            if white * 2 >= cell_pixel_count 
            {
                grid[gy*GRID_WIDTH + gx] = true;
            }
        }
    }

    (grid, macroquad_image)
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