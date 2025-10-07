use pixels::{Error, Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const WIDTH: i32 = 320;
const HEIGHT: i32 = 240;

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 2.0, HEIGHT as f64 * 2.0);
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Pixel Fighting Game")
                        .with_inner_size(scaled_size)
                        .with_min_inner_size(size),
                )
                .unwrap(),
        );

        let window_size = window.inner_size();

        self.window = Some(window.clone());

        self.pixels = Some({
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, window.clone());
            Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
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
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                if let Some(pixels) = self.pixels.as_mut() {
                    let frame = pixels.frame_mut();

                    const BOX_SIZE: i16 = 64;
                    let box_x: i16 = 100;
                    let box_y: i16 = 100;

                    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                        let x = (i % WIDTH as usize) as i16;
                        let y = (i / WIDTH as usize) as i16;

                        let inside_the_box = x >= box_x
                            && x < box_x + BOX_SIZE
                            && y >= box_y
                            && y < box_y + BOX_SIZE;

                        let rgba = if inside_the_box {
                            [0x5e, 0x48, 0xe8, 0xff]
                        } else {
                            [0x48, 0xb2, 0xe8, 0xff]
                        };

                        pixel.copy_from_slice(&rgba);
                    }

                    pixels.render().unwrap();
                }

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    //event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();

    event_loop.run_app(&mut app).unwrap()
}
