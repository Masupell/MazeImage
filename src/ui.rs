use macroquad::prelude::*;
use egui_macroquad::egui;

pub struct UI
{
    visible: bool
}

impl UI
{
    pub fn new() -> UI
    {
        UI 
        {
            visible: true
        }
    }

    pub fn draw(&self) -> bool
    {
        let mut block_input = false;

        egui_macroquad::ui(|egui_ctx| 
        {
            block_input = egui_ctx.wants_pointer_input() || egui_ctx.is_pointer_over_area();

            egui::Window::new("Hello?").anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 0.0))
            .show(egui_ctx, |ui|
            {
                ui.label("Meow :3");
                ui.add_space(10.0);
                if ui.button("Test").clicked()
                {
                    ui.label("Test2");
                }
            });
        });
        
        egui_macroquad::draw();

        block_input
    }
}