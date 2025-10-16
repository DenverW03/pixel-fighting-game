use crate::renderer::{Config, create_app, create_event_loop, run};

pub fn initialize_game() {
    let cfg = Config {
        title: "Pixel Fighting Game".into(),
        width: 320,
        height: 240,
        scale: 2.0,
    };

    // MacOS moment: event_loop must be created on main thread
    let event_loop = create_event_loop();

    // Create app with game state
    let app = create_app(cfg);

    run(app, event_loop).unwrap();
}
