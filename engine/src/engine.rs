use crate::renderer::{Config, create_app, create_event_loop, run};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const FRAMERATE: u8 = 60;

pub fn initialize_game() {
    let cfg = Config {
        title: "Pixel Fighting Game".into(),
        width: 320,
        height: 240,
        scale: 2.0,
    };

    // MacOS moment: event_loop must be created on main thread
    let event_loop = create_event_loop();

    // Create a channel for sending frames
    let (frame_tx, frame_rx) = mpsc::channel::<Vec<u8>>();

    // Create app and pass it the frame receiver
    let app = create_app(cfg, frame_rx);

    // Spawn a worker thread to push frames
    thread::spawn(move || {
        let mut t = 0;
        loop {
            // Create a simple test frame (moving colored box)
            let mut frame = vec![0x10; 320 * 240 * 4];

            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let x = (i % 320) as i16;
                let y = (i / 320) as i16;
                let box_x = (t % 200) as i16;
                let box_y = 100;

                let inside = x >= box_x && x < box_x + 50 && y >= box_y && y < box_y + 50;
                if inside {
                    pixel.copy_from_slice(&[0x5e, 0x48, 0xe8, 0xff]); // purple box
                } else {
                    pixel.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff]); // sky color
                }
            }

            // Send frame to main thread
            if frame_tx.send(frame).is_err() {
                println!("Main thread closed. Exiting frame thread.");
                break;
            }

            t += 2;
            thread::sleep(Duration::from_millis(1 / FRAMERATE as u64));
        }
    });

    run(app, event_loop).unwrap();
}
