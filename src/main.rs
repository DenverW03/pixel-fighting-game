use render_lib::{Config, run};

fn main() {
    let cfg = Config {
        title: "Pixel Fighting Game".into(),
        width: 320,
        height: 240,
        scale: 2.0,
    };
    run(cfg).unwrap();

    let mut _frame: [[u8; 320]; 240] = [[0; 320]; 240];
}
