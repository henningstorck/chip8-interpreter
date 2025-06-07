extern crate sdl2;

mod cartridge;
mod chip8;
mod keypad;
mod memory;
mod window;

use cartridge::Cartridge;
use chip8::Chip8;
use keypad::Keypad;
use std::env;
use std::thread;
use std::time::Duration;
use window::Window;

pub const CHIP8_CPU_FREQUENCY: u64 = 500;
pub const CHIP8_CPU_FREQUENCY_SUPER_CHIP: u64 = 4000;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let args: Vec<String> = env::args().collect();
    let cartridge_filename = &args[1];
    let super_chip = is_super_chip_enabled();
    let sleep_duration = get_sleep_duration(super_chip);
    let cartridge = Cartridge::new(cartridge_filename);
    let mut display = Window::new(&sdl_context);
    let mut keypad = Keypad::new(&sdl_context).unwrap();
    let mut chip8 = Chip8::new(super_chip);
    chip8.load(&cartridge.rom);

    while let Ok(keypad) = keypad.poll() {
        let output = chip8.tick(keypad);

        if output.draw_flag {
            display.draw(output.memory, output.hi_res);
        }

        thread::sleep(sleep_duration);
    }
}

fn is_super_chip_enabled() -> bool {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return false;
    }

    matches!(args[2].to_lowercase().as_str(), "true")
}

fn get_sleep_duration(super_chip: bool) -> Duration {
    if super_chip {
        Duration::from_millis(1000 / CHIP8_CPU_FREQUENCY_SUPER_CHIP)
    } else {
        Duration::from_millis(1000 / CHIP8_CPU_FREQUENCY)
    }
}
