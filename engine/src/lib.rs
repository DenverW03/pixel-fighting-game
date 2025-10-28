use engine::{Config, GameState, start_game};

mod components;
mod ecs;
mod engine;
mod renderer;

pub struct Game {
    config: Config,
    game_state: GameState,
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
            title: title,
            width: pixel_width,
            height: pixel_height,
            scale: (window_height / pixel_height) as f64,
        };
        let game_state: GameState = GameState::new(config.width as u32, config.height as u32);

        let game: Game = Game {
            config: config,
            game_state: game_state,
        };
        game
    }

    pub fn run_game(&mut self) {
        start_game(self);
    }
}
