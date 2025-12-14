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
    commands: Vec<UiCommand>,
    state: AppState
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
            commands: Vec::new(),
            state: AppState::Maze
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
                    ui.heading(match self.state 
                    {
                        AppState::Maze => "Maze",
                        AppState::Draw => "Draw",
                    });

                    ui.separator();

                    ui.horizontal(|ui|
                    {
                        if ui.selectable_label(self.state == AppState::Maze, "Maze").clicked() 
                        {
                            self.state = AppState::Maze;
                        }
                        if ui.selectable_label(self.state == AppState::Draw, "Draw").clicked() 
                        {
                            self.state = AppState::Draw;
                        }
                    });

                    ui.separator();

                    match self.state
                    {
                        AppState::Maze => self.maze_ui(ui),
                        AppState::Draw => self.draw_ui(ui),
                    }
                });
            }
        });
        
        egui_macroquad::draw();

        block_input
    }

    fn maze_ui(&mut self, ui: &mut egui::Ui)
    {
        if ui.button("Regenerate Maze").clicked()
        {
            self.commands.push(UiCommand::RegenerateMaze
            {
                use_image: self.include_image,
                threshold: self.image_strength,
            });
        }

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
    }

    fn draw_ui(&mut self, ui: &mut egui::Ui)
    {
        ui.label("Brush settings");
        // ui.add(egui::Slider::new(&mut self.brush_size, 1.0..=50.0).text("Brush Size"));
        // ui.add(egui::Slider::new(&mut self.smoothing, 0.0..=1.0).text("Smoothing"));
    }

    pub fn drain_commands(&mut self) -> Vec<UiCommand>
    {
        std::mem::take(&mut self.commands)
    }

    pub fn get_path(&self) -> &str
    {
        &self.image_path
    }

    pub fn state(&self) -> AppState
    {
        self.state
    }
}

pub enum UiCommand
{
    RegenerateMaze { use_image: bool, threshold: f32 }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AppState
{
    Maze,
    Draw
}