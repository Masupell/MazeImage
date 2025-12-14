use image::{DynamicImage, GrayImage, ImageBuffer, Luma};
use macroquad::{color::BLACK, texture::Image};

use crate::constants::{GRID_HEIGHT, GRID_SIZE, GRID_WIDTH};

pub fn get_input_grid(path: &str) -> (Vec<bool>, Image)
{
    let mut grid = vec![false; GRID_SIZE];

    let input = image::open(path);
    if input.is_err()
    {
        println!("Error\n(Could be wrong path/does not exist");
        return (grid, Image::gen_image_color(16, 16, BLACK));
    }
    let mut image = input.unwrap();
    image = image.blur(5.0);
    image.to_luma8();
    let output = sobel(&image, 0.05);
    // output.save("src/res/output.png").unwrap();

    let macroquad_image = luma_to_macroquad_image(&output);


    let width = output.width() as f32;
    let height = output.height() as f32;

    // Based on GRID_SIZE now not on image_size
    for gy in 0..GRID_HEIGHT 
    {
        let y0 = gy * height as usize / GRID_HEIGHT;
        let y1 = (gy + 1) * height as usize / GRID_HEIGHT;

        for gx in 0..GRID_WIDTH 
        {
            let x0 = gx * width as usize / GRID_WIDTH;
            let x1 = (gx + 1) * width as usize / GRID_WIDTH;

            let mut white = 0;
            for y in y0..y1 
            {
                for x in x0..x1 
                {
                    let pixel = output.get_pixel(x as u32, y as u32);
                    if pixel[0] == 255 { white += 1; }
                }
            }

            let cell_pixel_count = (x1 - x0) * (y1 - y0);
            if white * 4 >= cell_pixel_count 
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