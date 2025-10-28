use engine::{Config, initialize_game};

mod components;
mod ecs;
mod engine;
mod renderer;

pub fn run_game(
    title: String,
    pixel_width: i32,
    pixel_height: i32,
    _window_width: i32,
    window_height: i32,
) {
    let config = Config {
        title: title,
        width: pixel_width,
        height: pixel_height,
        scale: (window_height / pixel_height) as f64,
    };

    initialize_game(config);
}
