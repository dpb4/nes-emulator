// use macroquad::prelude::*;
use nes_emulator::{
    ui::{pixels_renderer::PixelsRenderer, *},
    NESSystem, HEIGHT, WIDTH,
};
use pixels::{Pixels, SurfaceTexture};
use std::{
    env,
    fs,
    // time::{Duration, Instant},
};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

// const FRAME_DURATION: Duration = Duration::from_nanos(16_666_667);

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    if env::args().collect::<Vec<String>>().len() < 2 {
        println!("missing rom name to execute");
        return;
    }
    let rom_name: &String = &env::args().collect::<Vec<String>>()[1];

    let raw_bytes = fs::read(rom_name.to_owned()).unwrap();

    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as u32, HEIGHT as u32);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };
    // let mut world = World::new();

    let mut renderer = PixelsRenderer::new(PALETTE_NTSC);
    let mut emu =
        NESSystem::new(raw_bytes).expect("unable to create emulator struct, check rom loading");

    // let mut last_draw = Instant::now();

    let _ = event_loop.run(|event, elwt| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            renderer.draw_to(pixels.frame_mut());

            if let Err(_) = pixels.render() {
                println!("error in pixels.render :(");
                elwt.exit();
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            // Resize the window
            // if let Some(size) = input.window_resized() {
            //     if let Err(_) = pixels.resize_surface(size.width, size.height) {
            //         // log_error("pixels.resize_surface", err);
            //         elwt.exit();
            //         return;
            //     }
            // }

            emu.tick_one_frame();

            renderer.modify_buffer(|buf| emu.render(buf));

            window.request_redraw();
        }
    });
}
