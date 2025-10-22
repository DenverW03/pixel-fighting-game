use crate::renderer::{Config, create_app, create_event_loop, run};

// Game state, includes entity+component storage
pub struct GameState {
    pub frame_counter: u32,
    pub width: u32,
    pub height: u32,
    pub world: World,
}

impl GameState {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            frame_counter: 0,
            width,
            height,
        }
    }

    // Update game state for the next frame
    pub fn update(&mut self) {
        self.frame_counter += 2; // Same increment as before
    }

    // Generate a frame for the current game state
    pub fn generate_frame(&self) -> Vec<u8> {
        let mut frame = vec![0x10; (self.width * self.height * 4) as usize];

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as i16;
            let y = (i / self.width as usize) as i16;
            let box_x = (self.frame_counter % 200) as i16;
            let box_y = 100;

            let inside = x >= box_x && x < box_x + 50 && y >= box_y && y < box_y + 50;
            if inside {
                pixel.copy_from_slice(&[0x5e, 0x48, 0xe8, 0xff]); // purple box
            } else {
                pixel.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff]); // sky color
            }
        }

        frame
    }
}

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
    let game_state = GameState::new(cfg.width as u32, cfg.height as u32);
    let app = create_app(cfg, game_state);

    run(app, event_loop).unwrap();
}
