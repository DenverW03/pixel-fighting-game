use engine::Game;

fn main() {
    let width: i32 = 640;
    let height: i32 = 360;
    let window_width: i32 = 1920;
    let window_height: i32 = 1080;

    let game: Game = Game::new(
        "Pixel Fighting Game".into(),
        width,
        height,
        window_width,
        window_height,
    );

    game.run_game()
}
