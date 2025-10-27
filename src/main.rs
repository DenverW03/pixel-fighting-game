use engine::{Config, run_game};

fn main() {
    let config = Config {
        title: "Pixel Fighting Game".into(),
        width: 640,
        height: 360,
        scale: 2.0,
    };

    run_game(config);
}
