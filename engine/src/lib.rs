use engine::{Config, start_game};

mod components;
mod ecs;
mod engine;
mod renderer;

pub struct Game {
    config: Config,
}

impl Game {
    pub fn new(
        title: String,
        pixel_width: i32,
        pixel_height: i32,
        _window_width: i32,
        window_height: i32,
    ) -> Self {
        let config = Config {
            title,
            width: pixel_width,
            height: pixel_height,
            scale: (window_height / pixel_height) as f64,
        };

        Game { config }
    }

    pub fn run_game(self) {
        start_game(self.config);
    }
}
