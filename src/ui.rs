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

    pub fn draw(&self)
    {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
                .show(egui_ctx, |ui| {
                    ui.label("Test");
                });
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
    }
}