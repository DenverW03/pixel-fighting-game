use engine::initialize_game;

mod components;
mod ecs;
mod engine;
mod renderer;

pub struct Config {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub scale: f64,
}

pub fn run_game(config: Config) {
    initialize_game(config);
}
