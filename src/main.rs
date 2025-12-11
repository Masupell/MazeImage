use macroquad::prelude::*;

fn window_config() -> Conf
{
    Conf
    {
        window_title: "Maze Image".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() 
{
    loop 
    {
        clear_background(Color::new(0.164705882, 0.164705882, 0.164705882, 1.0));
        

        next_frame().await
    }
}
