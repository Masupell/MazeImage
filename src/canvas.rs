use macroquad::prelude::*;

use crate::{GridConfig, ui::FillMode};

pub struct Canvas
{
    canvas: Image,
    pub texture: Texture2D,
    last_pos: Option<Vec2>,
    smooth_pos: Vec2,
    show_grid: bool,
    zoom: f32,
    offset: Vec2,
    pan_last: Option<Vec2>,
    grid_fill: bool,
    normal_fill: bool,
}

impl Canvas 
{
    pub fn new(width: u16, height: u16) -> Self 
    {
        let canvas = Image::gen_image_color(width, height, BLACK);
        let texture = Texture2D::from_image(&canvas);
        texture.set_filter(FilterMode::Nearest);

        Self 
        {
            canvas,
            texture,
            last_pos: None,
            smooth_pos: vec2(0.0, 0.0),
            show_grid: false,
            zoom: 1.0,
            offset: vec2(0.0, 0.0),
            pan_last: None,
            grid_fill: false,
            normal_fill: false
        }
    }

    pub fn set_image(&mut self, image: Image)
    {
        self.canvas = image;
        self.texture = Texture2D::from_image(&self.canvas);
        self.texture.set_filter(FilterMode::Nearest);
        self.last_pos = None;
        self.smooth_pos = vec2(0.0, 0.0);
    }

    pub fn get_image(&self) -> Image
    {
        self.canvas.clone()
    }

    pub fn draw(&self, grid_config: &GridConfig)
    {
        draw_texture_ex(&self.texture, -self.offset.x * self.zoom, -self.offset.y * self.zoom, WHITE, DrawTextureParams { dest_size: Some(self.get_size()*self.zoom), ..Default::default() });

        if self.show_grid
        {
            self.draw_grid(grid_config);
        }
    }

    pub fn update(&mut self, block_input: bool, brush_size: f32, smoothing: f32, color: Color, grid_config: &GridConfig) 
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

        if is_mouse_button_down(MouseButton::Middle)
        {
            if let Some(last) = self.pan_last
            {
                let delta_screen = mouse_screen - last;
                let delta_canvas = delta_screen / self.zoom;

                self.offset -= delta_canvas; // no clamping yet, so can get lost
            }

            self.pan_last = Some(mouse_screen);
        }
        else
        {
            self.pan_last = None;
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
                if self.normal_fill { self.fill_normal(mouse, color); }
                else if self.grid_fill { self.fill_grid_cell(mouse, color, grid_config); }
                else { self.draw_line(last, self.smooth_pos, brush_size, color); }

                self.texture.update(&self.canvas);
            }
            self.last_pos = Some(self.smooth_pos);
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

    fn draw_grid(&self, grid_config: &GridConfig)
    {
        let (width, height) = (self.canvas.width() as f32, self.canvas.height() as f32);//screen_size();
        
        let cell_width = width / grid_config.grid_width as f32;
        let cell_height = height / grid_config.grid_height as f32;

        let color = Color::new(1.0, 1.0, 1.0, 0.25);

        for x in 0..=grid_config.grid_width
        {
            let x_canvas = x as f32 * cell_width;
            let x_screen = (x_canvas - self.offset.x) * self.zoom;
            draw_line(x_screen, -self.offset.y * self.zoom, x_screen, (height as f32 - self.offset.y) * self.zoom, 1.0, color); // macroquads draw line
        }

        for y in 0..=grid_config.grid_height
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


    pub fn fill_grid_cell(&mut self, mouse: Vec2, color: Color, grid_config: &GridConfig)
    {
        let cell_width = self.canvas.width() as f32 / grid_config.grid_width as f32;
        let cell_height = self.canvas.height() as f32 / grid_config.grid_height as f32;

        let gx = (mouse.x / cell_width).floor().clamp(0.0, (grid_config.grid_width-1) as f32) as usize;
        let gy = (mouse.y / cell_height).floor().clamp(0.0, (grid_config.grid_height-1) as f32) as usize;

        let start_x = (gx as f32 * cell_width).floor() as u32;
        let start_y = (gy as f32 * cell_height).floor() as u32;

        let end_x = ((gx+1) as f32 * cell_width).ceil() as u32;
        let end_y = ((gy+1) as f32 * cell_height).ceil() as u32;

        for y in start_y..end_y.min(self.canvas.height() as u32)
        {
            for x in start_x..end_x.min(self.canvas.width() as u32)
            {
                self.canvas.set_pixel(x, y, color);
            }
        }
    }

    pub fn fill_normal(&mut self, start: Vec2, color: Color)
    {
        let width = self.canvas.width();
        let height = self.canvas.height();

        let mut stack = vec![(start.x as i32, start.y as i32)];
        let target_color = self.canvas.get_pixel(start.x as u32, start.y as u32);

        if target_color == color { return; }

        while let Some((x, y)) = stack.pop()
        {
            if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 { continue; }

            let current = self.canvas.get_pixel(x as u32, y as u32);
            if current != target_color
            {
                continue;
            }

            self.canvas.set_pixel(x as u32, y as u32, color);

            stack.push((x+1, y));
            stack.push((x-1, y));
            stack.push((x, y+1));
            stack.push((x, y-1));
        }
    }

    pub fn set_fill(&mut self, fill_mode: FillMode)
    {
        match fill_mode 
        {
            FillMode::NormalFill => { self.normal_fill=true; self.grid_fill=false; },
            FillMode::GridFill => { self.normal_fill=false; self.grid_fill=true; },
            FillMode::None => { self.normal_fill=false; self.grid_fill=false; },
        }
    }
}