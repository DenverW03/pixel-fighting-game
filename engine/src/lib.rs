use engine::initialize_game;

mod components;
mod ecs;
mod engine;
mod renderer;

pub fn run_game() {
    initialize_game();
}
