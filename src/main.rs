use engine::{create_game, run_game};

fn main() {
    let width: i32 = 640;
    let height: i32 = 360;
    let window_width: i32 = 1920;
    let window_height: i32 = 1080;

    create_game(
        "Pixel Fighting Game".into(),
        width,
        height,
        window_width,
        window_height,
    );

    run_game()
}
