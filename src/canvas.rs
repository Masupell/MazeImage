use macroquad::prelude::*;

pub struct Canvas
{
    canvas: Image,
    pub texture: Texture2D,
    last_pos: Option<Vec2>,
    smooth_pos: Vec2,
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
        }
    }

    pub fn set_image(&mut self, image: Image)
    {
        self.canvas = image;
        self.texture = Texture2D::from_image(&self.canvas);
        self.last_pos = None;
        self.smooth_pos = vec2(0.0, 0.0);
    }

    pub fn update(&mut self, block_input: bool, brush_size: f32, smoothing: f32, color: Color) 
    {
        if block_input { return; }

        let mouse = vec2(mouse_position().0, mouse_position().1);

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
        }
        else
        {
            self.last_pos = None;
        }

        self.texture.update(&self.canvas);
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
}