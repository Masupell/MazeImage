use macroquad::prelude::*;

use crate::constants::{GRID_HEIGHT, GRID_WIDTH};

pub struct Canvas
{
    canvas: Image,
    pub texture: Texture2D,
    last_pos: Option<Vec2>,
    smooth_pos: Vec2,
    show_grid: bool,
    zoom: f32,
    offset: Vec2
}

impl Canvas 
{
    pub fn new(width: u16, height: u16) -> Self 
    {
        let canvas = Image::gen_image_color(width, height, BLACK);
        let texture = Texture2D::from_image(&canvas);

        Self 
        {
            canvas,
            texture,
            last_pos: None,
            smooth_pos: vec2(0.0, 0.0),
            show_grid: false,
            zoom: 1.0,
            offset: vec2(0.0, 0.0)
        }
    }

    pub fn set_image(&mut self, image: Image)
    {
        self.canvas = image;
        self.texture = Texture2D::from_image(&self.canvas);
        self.last_pos = None;
        self.smooth_pos = vec2(0.0, 0.0);
    }

    pub fn get_image(&self) -> Image
    {
        self.canvas.clone()
    }

    pub fn draw(&self)
    {
        draw_texture_ex(&self.texture, -self.offset.x * self.zoom, -self.offset.y * self.zoom, WHITE, DrawTextureParams { dest_size: Some(self.get_size()*self.zoom), ..Default::default() });

        if self.show_grid
        {
            self.draw_grid();
        }
    }

    pub fn update(&mut self, block_input: bool, brush_size: f32, smoothing: f32, color: Color) 
    {
        if block_input { return; }

        let mouse_screen = vec2(mouse_position().0, mouse_position().1);
        let mouse = self.screen_to_canvas(mouse_screen);
        let scroll = mouse_wheel().1;

        if scroll != 0.0
        {
            let zoom_factor = 1.1_f32.powf(scroll);
            let old_zoom = self.zoom;
            self.zoom = (self.zoom * zoom_factor).clamp(0.1, 20.0);

            let before = self.offset + mouse_screen / old_zoom;
            let after = self.offset + mouse_screen / self.zoom;
            self.offset += before - after;
        }

        if is_mouse_button_down(MouseButton::Left) 
        {
            if self.last_pos.is_none()
            {
                self.smooth_pos = mouse;
            }
            self.smooth_pos = self.smooth_pos.lerp(mouse, smoothing);

            if let Some(last) = self.last_pos
            {
                self.draw_line(last, self.smooth_pos, brush_size, color);
            }
            self.last_pos = Some(self.smooth_pos);

            self.texture.update(&self.canvas); // Only need to update when mouse is down (probably only even when draw_line is called)
        }
        else
        {
            self.last_pos = None;
        }
    }

    pub fn get_size(&self) -> Vec2
    {
        vec2(self.canvas.width() as f32, self.canvas.height() as f32)
    }

    fn draw_line(&mut self, a: Vec2, b: Vec2, brush_size: f32, color: Color)
    {
        let dist = a.distance(b);
        let steps = dist.max(1.0) as i32;

        for i in 0..=steps
        {
            let t = i as f32 / steps as f32;
            let p = a.lerp(b, t);
            self.draw_brush(p, brush_size, color);
        }
    }

    fn draw_brush(&mut self, pos: Vec2, brush_size: f32, color: Color)
    {
        let r = brush_size as i32;

        for y in -r..=r
        {
            for x in -r..=r
            {
                if x * x + y * y <= r * r
                {
                    let px = pos.x as i32 + x;
                    let py = pos.y as i32 + y;

                    if px >= 0 && py >= 0 && px < self.canvas.width() as i32 && py < self.canvas.height() as i32
                    {
                        self.canvas.set_pixel(px as u32, py as u32, color);
                    }
                }
            }
        }
    }

    fn draw_grid(&self)
    {
        let (width, height) = (self.canvas.width() as f32, self.canvas.height() as f32);//screen_size();
        
        let cell_width = width / GRID_WIDTH as f32;
        let cell_height = height / GRID_HEIGHT as f32;

        let color = Color::new(1.0, 1.0, 1.0, 0.25);

        for x in 0..=GRID_WIDTH
        {
            let x_canvas = x as f32 * cell_width;
            let x_screen = (x_canvas - self.offset.x) * self.zoom;
            draw_line(x_screen, -self.offset.y * self.zoom, x_screen, (height as f32 - self.offset.y) * self.zoom, 1.0, color); // macroquads draw line
        }

        for y in 0..=GRID_HEIGHT
        {
            let y_canvas = y as f32 * cell_height;
            let y_screen = (y_canvas - self.offset.y) * self.zoom;
            draw_line(-self.offset.x * self.zoom, y_screen, (width as f32 - self.offset.x) * self.zoom, y_screen, 1.0, color);
        }
    }

    fn screen_to_canvas(&self, screen: Vec2) -> Vec2
    {
        self.offset + screen / self.zoom
    }

    pub fn show_grid(&mut self, show: bool)
    {
        self.show_grid = show;
    }
}