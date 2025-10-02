use error_iter::ErrorIter as _;
use gilrs::{Button, GamepadId, Gilrs};
use pixels::{Error, Pixels, SurfaceTexture};
use simple_invaders::{Controls, Direction, FPS, HEIGHT, TIME_STEP, WIDTH, World};
use std::sync::Arc;
use std::{env, time::Duration};
use winit::{dpi::LogicalSize, event_loop::EventLoop, event_loop::ActiveEventLoop, window::WindowAttributes, keyboard::KeyCode, window::Window};
use winit_input_helper::WinitInputHelper;

const WIDTH: i32 = 320;
const HEIGHT: i32 = 240;

struct Game {
    /// Software renderer.
    pixels: Pixels<'static>,
    /// Invaders world.
    world: World,
    /// Player controls for world updates.
    controls: Controls,
    /// Event manager.
    input: WinitInputHelper,
    /// GamePad manager.
    gilrs: Gilrs,
    /// GamePad ID for the player.
    gamepad: Option<GamepadId>,
    /// Game pause state.
    paused: bool,
}

impl Game {
    fn new(pixels: Pixels<'static>, debug: bool) -> Self {
        Self {
            pixels,
            world: World::new(generate_seed(), debug),
            controls: Controls::default(),
            input: WinitInputHelper::new(),
            gilrs: Gilrs::new().unwrap(), // XXX: Don't unwrap.
            gamepad: None,
            paused: false,
        }
    }

    fn update_controls(&mut self) {
        // Pump the gilrs event loop and find an active gamepad
        while let Some(gilrs::Event { id, event, .. }) = self.gilrs.next_event() {
            let pad = self.gilrs.gamepad(id);
            if self.gamepad.is_none() {
                debug!("Gamepad with id {} is connected: {}", id, pad.name());
                self.gamepad = Some(id);
            } else if event == gilrs::ev::EventType::Disconnected {
                debug!("Gamepad with id {} is disconnected: {}", id, pad.name());
                self.gamepad = None;
            }
        }

        self.controls = {
            // Keyboard controls
            let mut left = self.input.key_held(KeyCode::ArrowLeft);
            let mut right = self.input.key_held(KeyCode::ArrowRight);
            let mut fire = self.input.key_pressed(KeyCode::Space);
            let mut pause =
                self.input.key_pressed(KeyCode::Pause) | self.input.key_pressed(KeyCode::KeyP);

            // GamePad controls
            if let Some(id) = self.gamepad {
                let gamepad = self.gilrs.gamepad(id);

                left |= gamepad.is_pressed(Button::DPadLeft);
                right |= gamepad.is_pressed(Button::DPadRight);
                fire |= gamepad.button_data(Button::South).is_some_and(|button| {
                    button.is_pressed() && button.counter() == self.gilrs.counter()
                });
                pause |= gamepad.button_data(Button::Start).is_some_and(|button| {
                    button.is_pressed() && button.counter() == self.gilrs.counter()
                });
            }
            self.gilrs.inc();

            if pause {
                self.paused = !self.paused;
            }

            let direction = if left {
                Direction::Left
            } else if right {
                Direction::Right
            } else {
                Direction::Still
            };

            Controls { direction, fire }
        };
    }

    fn reset_game(&mut self) {
        self.world.reset_game();
    }
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new().unwrap();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 2.0, HEIGHT as f64 * 2.0);
        let window = ActiveEvetLoop::create_window(WindowAttributes::default()
            .with_title("Pixel Fighting Game")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            )
            .unwrap();
        Arc::new(window)
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    }

    let game = Game::new(pixels, debug);

    let res = game_loop(
        event_loop,
        window,
        game,
        FPS as u32,
        0.1,
        move |g| {
            // Update the world
            if !g.game.paused {
                g.game.world.update(&g.game.controls);
            }
        },
        move |g| {
            // Drawing
            g.game.world.draw(g.game.pixels.frame_mut());
            if let Err(err) = g.game.pixels.render() {
                g.exit();
            }

            // Sleep the main thread to limit drawing to the fixed time step.
            // See: https://github.com/parasyte/pixels/issues/174
            let dt = TIME_STEP.as_secs_f64() - Time::now().sub(&g.current_instant());
            if dt > 0.0 {
                std::thread::sleep(Duration::from_secs_f64(dt));
            }
        },
        |g, event| {
            // Let winit_input_helper collect events to build its state.
            if g.game.input.update(event) {
                // Update controls
                g.game.update_controls();

                // Close events
                if g.game.input.key_pressed(KeyCode::Escape) || g.game.input.close_requested() {
                    g.exit();
                    return;
                }

                // Reset game
                if g.game.input.key_pressed(KeyCode::KeyR) {
                    g.game.reset_game();
                }

                // Resize the window
                if let Some(size) = g.game.input.window_resized() {
                    if let Err(err) = g.game.pixels.resize_surface(size.width, size.height) {
                        g.exit();
                    }
                }
            }
        },
    );
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}
