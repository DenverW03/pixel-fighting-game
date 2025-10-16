use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub struct Config {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub scale: f64,
}

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    size: LogicalSize<f64>,
    scaled_size: LogicalSize<f64>,
    title: String,
    frame_counter: u32,
}

impl App {
    pub fn with_config(config: Config) -> Self {
        let mut app = App::default();
        app.title = config.title;
        app.size = LogicalSize::new(config.width as f64, config.height as f64);
        app.scaled_size = LogicalSize::new(
            config.width as f64 * config.scale,
            config.height as f64 * config.scale,
        );
        app.frame_counter = 0;
        app
    }

    fn update_frame(&mut self) {
        // Generate frame directly in the main thread
        let frame = self.generate_frame();
        self.push_frame(&frame);
        self.frame_counter += 2; // Same increment as before
    }

    // Generate a frame for the current game state
    fn generate_frame(&self) -> Vec<u8> {
        let mut frame = vec![0x10; 320 * 240 * 4];

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % 320) as i16;
            let y = (i / 320) as i16;
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

    // Pushes a new frame to the pixels buffer
    // Passed in as a u8 slice, representing the flattened frame
    pub fn push_frame(&mut self, new_frame: &[u8]) {
        if let Some(pixels) = self.pixels.as_mut() {
            let frame = pixels.frame_mut();
            let index_max = (self.scaled_size.width * self.scaled_size.height) as usize;

            for i in 0..index_max {
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

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Update game state and render frame
                self.update_frame();
                if let Some(pixels) = &mut self.pixels {
                    if let Err(e) = pixels.render() {
                        eprintln!("pixels.render() failed: {:?}", e);
                    }
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

pub fn create_event_loop() -> EventLoop<()> {
    EventLoop::new().unwrap()
}

// Creates an App with the given Config
pub fn create_app(config: Config) -> App {
    App::with_config(config)
}

// Runs the event loop and app
pub fn run(mut app: App, event_loop: EventLoop<()>) -> Result<(), EventLoopError> {
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app)
}
