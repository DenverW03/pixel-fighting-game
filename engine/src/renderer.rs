use pixels::{Pixels, SurfaceTexture};
use std::sync::{Arc, mpsc::Receiver};
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
    frame_rx: Option<Receiver<Vec<u8>>>,
}

impl App {
    pub fn with_config(config: Config, frame_rx: Receiver<Vec<u8>>) -> Self {
        let mut app = App::default();
        app.title = config.title;
        app.size = LogicalSize::new(config.width as f64, config.height as f64);
        app.scaled_size = LogicalSize::new(
            config.width as f64 * config.scale,
            config.height as f64 * config.scale,
        );
        app.frame_rx = Some(frame_rx);
        app
    }

    fn try_update_frame(&mut self) {
        if let Some(rx) = &self.frame_rx {
            if let Ok(new_frame) = rx.try_recv() {
                self.push_frame(&new_frame);
            }
        }
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
                // called every redraw
                self.try_update_frame();
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

// Creates an App, passing in the Config and frame receiver
pub fn create_app(config: Config, frame_rx: Receiver<Vec<u8>>) -> App {
    App::with_config(config, frame_rx)
}

// Runs the event loop and app
pub fn run(mut app: App, event_loop: EventLoop<()>) -> Result<(), EventLoopError> {
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app)
}
