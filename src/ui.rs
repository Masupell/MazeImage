use macroquad::prelude::*;
use egui_macroquad::egui;

const HOVER_WIDTH: f32 = 130.0;
const PANEL_HEIGHT: f32 = 250.0;

pub struct UI
{
    visible: bool,
    hovered: bool,
    image_path: String,
    include_image: bool,
    image_strength: f32, //0.0to1.0
    commands: Vec<UiCommand>
}

impl UI
{
    pub fn new() -> UI
    {
        UI 
        {
            visible: false,
            hovered: false,
            image_path: String::new(),
            include_image: true,
            image_strength: 0.1,
            commands: Vec::new()
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
                .frame(egui::Frame::NONE)
                .fixed_pos(egui::pos2(2.0, center_y - 20.0))
                .fixed_size(egui::vec2(24.0, 80.0))
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
                .fixed_pos(egui::pos2(24.0, center_y - PANEL_HEIGHT * 0.5))
                .fixed_size(egui::vec2(260.0, PANEL_HEIGHT))
                .movable(false)
                .resizable(false)
                .collapsible(false)
                .show(egui_ctx, |ui| 
                {
                    ui.heading("TestStuff");

                    ui.separator();
                    ui.separator();

                    if ui.button("Regenerate Maze").clicked()
                    {
                        if self.include_image
                        {
                            let (grid, image) = crate::image::get_input_grid(&self.image_path);
                            let walls = crate::maze::get_all_walls(&grid);

                            self.commands.push(UiCommand::RegenerateMaze { grid_input: Some(grid), wall_input: Some(walls), image: Some(image), threshold: self.image_strength });
                        }
                        else 
                        {
                            self.commands.push(UiCommand::RegenerateMaze { grid_input: None, wall_input: None, image: None, threshold: self.image_strength });    
                        }
                    }

                    ui.separator();
                    ui.separator();

                    ui.horizontal(|ui| 
                    {
                        ui.checkbox(&mut self.include_image, "Include Image");
                        ui.text_edit_singleline(&mut self.image_path);
                        if ui.button("Browse").clicked() 
                        {
                            if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Image files", &["png", "jpg", "jpeg", "bmp", "gif", "tiff"])
                            .pick_file()
                            {
                                self.image_path = path.to_string_lossy().to_string();
                            }
                        }
                    });

                    ui.add(egui::Slider::new(&mut self.image_strength, 0.0..=1.0).text("Threshold"));

                    ui.separator();
                    ui.separator();
                    ui.separator();
                    ui.separator();
                    ui.separator();
                    ui.separator();
                    ui.separator();

                    ui.label("TestStuff2");
                });
            }
        });
        
        egui_macroquad::draw();

        block_input
    }

    pub fn drain_commands(&mut self) -> Vec<UiCommand>
    {
        std::mem::take(&mut self.commands)
    }
}

pub enum UiCommand
{
    RegenerateMaze { grid_input: Option<Vec<bool>>, wall_input: Option<Vec<usize>>, image: Option<Image>, threshold: f32 }
}