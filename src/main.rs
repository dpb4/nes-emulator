use macroquad::prelude::*;
use nes_emulator::{
    ui::{macro_system::MacroSystem, *},
    NESSystem,
};
use std::{
    env, fs,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

pub const SCALE: i32 = 3;
fn conf() -> Conf {
    Conf {
        window_title: "NES Emulator".to_string(),
        fullscreen: false,
        window_resizable: false,
        window_width: 256 * SCALE,
        window_height: 240 * SCALE,
        ..Default::default()
    }
}

const FRAME_DURATION: Duration = Duration::from_nanos(16_666_667);
#[macroquad::main(conf)]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    if env::args().collect::<Vec<String>>().len() < 2 {
        println!("missing rom name to execute");
        return;
    }
    let rom_name: &String = &env::args().collect::<Vec<String>>()[1];

    let raw_bytes = fs::read(rom_name.to_owned() + &".nes".to_string()).unwrap();

    // let  =
    //     NESSystem::new(raw_bytes).expect("unable to create emulator struct, check rom loading");

    let mut macro_system = MacroSystem::new(&PALETTE_NTSC);
    // let nes = NESSystem::new(raw_bytes).expect("unable to create emulator struct, check rom loading");
    // nes.
    let nes_emu = Arc::new(Mutex::new(
        NESSystem::new(raw_bytes).expect("unable to create emulator struct, check rom loading"),
    ));
    let nes_emu_clone: Arc<Mutex<NESSystem>> = Arc::clone(&nes_emu);

    let mut count = 0;
    thread::spawn(move || {
        loop {
            count += 1;
            println!("frame {count}");
            let mut nes_emu = nes_emu.lock().unwrap();
            nes_emu.tick_one_frame();
            thread::sleep(Duration::from_millis(16));
        }
        // let nes_emu = nes_emu.lock().unwrap();
        // nes_emu.save_log();
    });

    // for _ in 0..4 {
    //     count += 1;
    //     println!("frame {count}");
    //     let mut nes_emu = nes_emu.lock().unwrap();
    //     nes_emu.tick_one_frame();
    //     // thread::sleep(Duration::from_millis(16));
    // }
    // let nes_emu = nes_emu.lock().unwrap();
    // nes_emu.save_log();

    // println!("Success\n");

    // next_frame().await

    // let mut pixels = Vec::new();

    // for y in 0..240 {
    //     for x in 0..256 {
    //         if 40 < x && x < 80 && 40 < y && y < 80 {
    //             pixels.push(63);
    //             // pixels.push(255);
    //             // pixels.push(255);
    //             // pixels.push(255);
    //         } else {
    //             // pixels.push((x as u8) % 64);
    //             // pixels.push((y as u8) % 64);
    //             pixels.push((x % 64) as u8);
    //             // pixels.push(64);
    //         }
    //     }
    // }
    let mut last_draw = Instant::now();

    loop {
        if last_draw.elapsed() >= FRAME_DURATION {
            last_draw = Instant::now();

            let nes_emu_ref = nes_emu_clone.lock().unwrap();
            // macro_system.draw_new_frame(
            //     pixels[0..256 * 240].try_into().unwrap(),
            //     macro_system.palette,
            // );
            macro_system.draw_new_frame(nes_emu_ref.get_frame_pixel_buffer(), macro_system.palette);
        } else {
            macro_system.redraw();
        }

        next_frame().await;
    }
}
