use crate::components::{Player, Position, Size, Velocity};
use crate::ecs::{Entity, World};
use crate::renderer::{create_app, create_event_loop, run};

// An RGBA pixel requires 4 numbers for the R,G,B,A values
const RGBA_SIZE: u8 = 4;
// Velocity constants
const MAX_VELOCITY: u8 = 5;
const VELOCITY_STEP: u8 = 1;

#[derive(Clone)]
pub struct Config {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub scale: f64,
}

pub struct GameState {
    pub width: u32,
    pub height: u32,
    pub world: World,
    pub player: Entity,
}

impl GameState {
    pub fn new(width: u32, height: u32) -> Self {
        let mut world = World::new();

        // Create a player entity
        let player = world.create_entity();
        world.add_component(player, Position { x: 100.0, y: 100.0 });
        world.add_component(player, Velocity { x: 0.0, y: 0.0 });
        world.add_component(
            player,
            Size {
                width: 50.0,
                height: 50.0,
            },
        );
        world.add_component(player, Player {});

        GameState {
            width,
            height,
            world,
            player,
        }
    }

    pub fn generate_frame(&self) -> Vec<u8> {
        let mut frame = vec![0x10; (self.width * self.height * RGBA_SIZE as u32) as usize];

        // Use tracked player entity
        let position = self.world.get_component::<Position>(self.player).unwrap();
        let size = self.world.get_component::<Size>(self.player).unwrap();

        let mut switch = false;
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
                if switch {
                    pixel.copy_from_slice(&[0x5e, 0x48, 0xe8, 0xff]);
                } else {
                    pixel.copy_from_slice(&[0x60, 0x32, 0x10, 0xff]);
                }
            } else {
                pixel.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff]);
            }

            switch = !switch;
        }

        frame
    }

    // TODO: Implement
    fn draw_player(&mut self, frame: Vec<u8>) {}

    pub fn update_entity_positions(&mut self) {
        let entity_velocities: Vec<(Entity, Velocity)> = {
            let storage = self.world.get_storage::<Velocity>();
            storage.components.iter().map(|(e, v)| (*e, *v)).collect()
        };

        for (entity, velocity) in entity_velocities {
            if let Some(position) = self.world.get_component_mut::<Position>(entity) {
                position.x += velocity.x;
                position.y += velocity.y;
            }
        }
    }

    pub fn update_player_velocity(&mut self, direction: &str) {
        if let Some(velocity) = self.world.get_component_mut::<Velocity>(self.player) {
            match direction {
                "up" => {
                    if velocity.y > 0.0 {
                        velocity.y = 0.0;
                    }
                    velocity.y = (velocity.y - VELOCITY_STEP as f64)
                        .clamp(-1.0 * (MAX_VELOCITY as f64), MAX_VELOCITY as f64);
                }
                "down" => {
                    if velocity.y < 0.0 {
                        velocity.y = 0.0;
                    }
                    velocity.y = (velocity.y + VELOCITY_STEP as f64)
                        .clamp(-1.0 * (MAX_VELOCITY as f64), MAX_VELOCITY as f64);
                }
                "left" => {
                    if velocity.x > 0.0 {
                        velocity.x = 0.0;
                    }
                    velocity.x = (velocity.x - VELOCITY_STEP as f64)
                        .clamp(-1.0 * (MAX_VELOCITY as f64), MAX_VELOCITY as f64);
                }
                "right" => {
                    if velocity.x < 0.0 {
                        velocity.x = 0.0;
                    }
                    velocity.x = (velocity.x + VELOCITY_STEP as f64)
                        .clamp(-1.0 * (MAX_VELOCITY as f64), MAX_VELOCITY as f64);
                }
                _ => (),
            }
        }
    }

    pub fn zero_player_vel(&mut self, x: bool, y: bool) {
        if let Some(velocity) = self.world.get_component_mut::<Velocity>(self.player) {
            if x {
                velocity.x = 0.0;
            }
            if y {
                velocity.y = 0.0;
            }
        }
    }
}

pub fn start_game(config: Config) {
    let game_state = GameState::new(config.width as u32, config.height as u32);
    let event_loop = create_event_loop();
    let app = create_app(config, game_state);
    run(app, event_loop).unwrap();
}
