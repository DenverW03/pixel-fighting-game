use crate::Config;
use crate::components::{Player, Position, Size, Velocity};
use crate::ecs::{Entity, World};
use crate::renderer::{create_app, create_event_loop, run};

// An RGBA pixel requires 4 numbers for the R,G,B,A values
const RGBA_SIZE: u32 = 4;

// Game state, includes entity+component storage
pub struct GameState {
    pub width: u32,
    pub height: u32,
    pub world: World,
}

impl GameState {
    pub fn new(width: u32, height: u32) -> Self {
        let mut game_state: GameState = GameState {
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
            .add_component(player, Velocity { x: 0.0, y: 0.0 });
        game_state.world.add_component(
            player,
            Size {
                width: 50.0,
                height: 50.0,
            },
        );
        game_state.world.add_component(player, Player {});

        game_state
    }

    // Generate a frame for the current game state
    pub fn generate_frame(&mut self) -> Vec<u8> {
        // Start by updating all physics logic, that will affect rendering
        self.update_entity_positions();

        let mut frame = vec![0x10; (self.width * self.height * RGBA_SIZE) as usize];

        let player_storage = self.world.get_storage::<Player>();
        let player: Entity = {
            let mut player: Entity = Entity(0);
            for (entity, _) in &player_storage.components {
                player = *entity;
            }
            player
        };

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
                pixel.copy_from_slice(&[0x5e, 0x48, 0xe8, 0xff]); // purple
            } else {
                pixel.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff]); // sky color
            }
        }

        frame
    }

    // Updates all position component for entities with both a position and a velocity
    fn update_entity_positions(&mut self) {
        let entity_velocities: Vec<(Entity, Velocity)> = {
            let storage = self.world.get_storage::<Velocity>();
            storage
                .components
                .iter()
                .map(|(e, v)| (*e, v.clone()))
                .collect()
        };
        for (entity, velocity) in entity_velocities {
            let position = self.world.get_component_mut::<Position>(entity).unwrap();
            position.x += velocity.x;
            position.y += velocity.y;
        }
    }

    pub fn update_player_velocity(&mut self, direction: &str) {
        let storage = self.world.get_storage::<Player>();
        let mut player_list: Vec<Entity> = Vec::new();

        for (entity, _component) in &storage.components {
            player_list.push(*entity);
        }
        for player in player_list {
            let velocity = self.world.get_component_mut::<Velocity>(player).unwrap();
            match direction {
                "up" => {
                    if velocity.y > 0.0 {
                        velocity.y = 0.0
                    }
                    velocity.y = (velocity.y - 1.0).clamp(-5.0, 5.0);
                }
                "down" => {
                    if velocity.y < 0.0 {
                        velocity.y = 0.0
                    }
                    velocity.y = (velocity.y + 1.0).clamp(-5.0, 5.0);
                }
                "left" => {
                    if velocity.x > 0.0 {
                        velocity.x = 0.0
                    }
                    velocity.x = (velocity.x - 1.0).clamp(-5.0, 5.0);
                }
                "right" => {
                    if velocity.x < 0.0 {
                        velocity.x = 0.0
                    }
                    velocity.x = (velocity.x + 1.0).clamp(-5.0, 5.0);
                }
                _ => (),
            }
        }
    }
}

pub fn initialize_game(config: Config) {
    // MacOS moment: event_loop must be created on main thread
    let event_loop = create_event_loop();

    // Create app with game state
    let game_state = GameState::new(config.width as u32, config.height as u32);
    let app = create_app(config, game_state);

    run(app, event_loop).unwrap();
}
