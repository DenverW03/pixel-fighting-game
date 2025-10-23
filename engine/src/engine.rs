use crate::components::{Position, Size, Velocity};
use crate::ecs::{Entity, World};
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
        let mut game_state: GameState = GameState {
            frame_counter: 0,
            width,
            height,
            world: World::new(),
        };

        // Create a player entity
        let player = game_state.world.create_entity();
        game_state
            .world
            .add_component(player, Position { x: 100.0, y: 100.0 });
        game_state
            .world
            .add_component(player, Velocity { x: 1.0, y: 0.0 });
        game_state.world.add_component(
            player,
            Size {
                width: 50.0,
                height: 50.0,
            },
        );

        game_state
    }

    // Update game state for the next frame
    pub fn update(&mut self) {
        self.frame_counter += 2; // Same increment as before
    }

    // Generate a frame for the current game state
    pub fn generate_frame(&self) -> Vec<u8> {
        let mut frame = vec![0x10; (self.width * self.height * 4) as usize];

        let player: Entity = Entity(0);

        // Getting the position and size of the player from the world storage
        let position: &Position = self.world.get_component::<Position>(player).unwrap();
        let size: &Size = self.world.get_component::<Size>(player).unwrap();

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as i16;
            let y = (i / self.width as usize) as i16;
            let box_x = position.x as i16;
            let box_y = position.y as i16;

            let inside = x >= box_x
                && x < box_x + size.width as i16
                && y >= box_y
                && y < box_y + size.height as i16;
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
