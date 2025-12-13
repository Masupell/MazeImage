use macroquad::prelude::*;
use egui_macroquad::egui;

const HOVER_WIDTH: f32 = 130.0;
const PANEL_HEIGHT: f32 = 250.0;

pub struct UI
{
    visible: bool,
    hovered: bool
}

impl UI
{
    pub fn new() -> UI
    {
        UI 
        {
            visible: false,
            hovered: false,
        }
    }

    pub fn draw(&mut self) -> bool
    {
        let mut block_input = false;

        let (mx, my) = mouse_position();
        let screen_h = screen_height();

        let center_y = screen_h * 0.5;
        let top = center_y - PANEL_HEIGHT * 0.5;
        let bottom = center_y + PANEL_HEIGHT * 0.5;

        self.hovered = mx<HOVER_WIDTH && my>top && my<bottom;

        egui_macroquad::ui(|egui_ctx| 
        {
            block_input = egui_ctx.wants_pointer_input() || egui_ctx.is_pointer_over_area();

            if self.hovered || self.visible// && !self.visible
            {
                egui::Window::new("ui_handle")
                .fixed_pos(egui::pos2(0.0, center_y - 20.0))
                .fixed_size(egui::vec2(24.0, 40.0))
                .title_bar(false)
                .resizable(false)
                .movable(false)
                .collapsible(false)
                .show(egui_ctx, |ui| 
                {
                    if ui.button("▶").clicked() 
                    {
                        self.visible = !self.visible;
                    }
                });
            }

            if self.visible
            {
                egui::Window::new("Debug Panel")
                .fixed_pos(egui::pos2(0.0, center_y - PANEL_HEIGHT * 0.5))
                .fixed_size(egui::vec2(260.0, PANEL_HEIGHT))
                .movable(false)
                .resizable(false)
                .collapsible(false)
                .show(egui_ctx, |ui| 
                {
                    ui.heading("TestStuff");

                    ui.separator();

                    ui.label("TestStuff2");
                });
            }
        });
        
        egui_macroquad::draw();

        block_input
    }
}