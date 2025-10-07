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
}

impl App {
    pub fn with_config(config: Config) -> Self {
        let mut app = App::default();
        app.size = LogicalSize::new(config.width as f64, config.height as f64);
        app.scaled_size = LogicalSize::new(config.width as f64 * 2.0, config.height as f64 * 2.0);
        app
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
                        .with_title("Pixel Fighting Game")
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
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

pub fn run(config: Config) -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    //event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::with_config(config);

    event_loop.run_app(&mut app)
}
