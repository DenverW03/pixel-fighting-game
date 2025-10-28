use crate::Config;
use crate::engine::GameState;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::error::EventLoopError;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowId};
use winit_input_helper::WinitInputHelper;

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    input: WinitInputHelper,
    size: LogicalSize<f64>,
    scaled_size: LogicalSize<f64>,
    title: String,
    game_state: Option<GameState>,
}

impl App {
    pub fn with_config(config: Config, game_state: GameState) -> Self {
        let mut app = App::default();
        app.title = config.title;
        app.size = LogicalSize::new(config.width as f64, config.height as f64);
        app.scaled_size = LogicalSize::new(
            config.width as f64 * config.scale,
            config.height as f64 * config.scale,
        );
        app.game_state = Some(game_state);
        app
    }

    fn update_frame(&mut self) {
        if let Some(game_state) = &mut self.game_state {
            // Generate frame directly in the main thread
            let frame = game_state.generate_frame();
            self.push_frame(&frame);
        }
    }

    // Pushes a new frame to the pixels buffer
    // Passed in as a u8 slice, representing the flattened frame
    pub fn push_frame(&mut self, new_frame: &[u8]) {
        if let Some(pixels) = self.pixels.as_mut() {
            let frame = pixels.frame_mut();

            for i in 0..frame.len() {
                frame[i] = new_frame[i];
            }

            pixels.render().unwrap();
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(self.title.clone())
                        .with_inner_size(self.scaled_size)
                        .with_min_inner_size(self.size),
                )
                .unwrap(),
        );

        let window_size = window.inner_size();

        self.window = Some(window.clone());

        self.pixels = Some({
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, window.clone());
            Pixels::new(
                self.size.width as u32,
                self.size.height as u32,
                surface_texture,
            )
            .unwrap()
        });

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if self.input.process_window_event(&event) {
            // Update game state and render frame
            self.update_frame();

            if let Some(pixels) = &mut self.pixels {
                if let Err(e) = pixels.render() {
                    eprintln!("pixels.render() failed: {:?}", e);
                }
            }

            self.window.as_ref().unwrap().request_redraw();
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        // pass in events
        self.input.process_device_event(&event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.input.end_step();

        if self.input.key_released(KeyCode::KeyQ)
            || self.input.close_requested()
            || self.input.destroyed()
        {
            println!(
                "The application was requsted to close or the 'Q' key was pressed, quiting the application"
            );
            event_loop.exit();
            return;
        }

        let mut x_input: bool = false;
        let mut y_input: bool = false;
        if self.input.key_held(KeyCode::ArrowUp) || self.input.key_held(KeyCode::KeyW) {
            if let Some(game_state) = &mut self.game_state {
                game_state.update_player_velocity("up");
            }
            y_input = true;
        }
        if self.input.key_held(KeyCode::ArrowDown) || self.input.key_held(KeyCode::KeyS) {
            if let Some(game_state) = &mut self.game_state {
                game_state.update_player_velocity("down");
            }
            y_input = true;
        }
        if self.input.key_held(KeyCode::ArrowLeft) || self.input.key_held(KeyCode::KeyA) {
            if let Some(game_state) = &mut self.game_state {
                game_state.update_player_velocity("left");
            }
            x_input = true;
        }
        if self.input.key_held(KeyCode::ArrowRight) || self.input.key_held(KeyCode::KeyD) {
            if let Some(game_state) = &mut self.game_state {
                game_state.update_player_velocity("right");
            }
            x_input = true;
        }

        if let Some(game_state) = &mut self.game_state {
            game_state.zero_player_vel(!x_input, !y_input);
        }
    }

    fn new_events(&mut self, _: &ActiveEventLoop, _: StartCause) {
        self.input.step();
    }
}

pub fn create_event_loop() -> EventLoop<()> {
    EventLoop::new().unwrap()
}

// Creates an App with the given Config and GameState
pub fn create_app(config: Config, game_state: GameState) -> App {
    App::with_config(config, game_state)
}

// Runs the event loop and app
pub fn run(mut app: App, event_loop: EventLoop<()>) -> Result<(), EventLoopError> {
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app)
}
