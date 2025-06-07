use crate::memory::Memory;

pub const WIDTH_LO_RES: usize = 64;
pub const HEIGHT_LO_RES: usize = 32;
pub const WIDTH_HI_RES: usize = 128;
pub const HEIGHT_HI_RES: usize = 64;
const SCALE_LO_RES: u32 = 16;
const SCALE_HI_RES: u32 = 8;
const SCREEN_WIDTH: u32 = WIDTH_LO_RES as u32 * SCALE_LO_RES;
const SCREEN_HEIGHT: u32 = HEIGHT_LO_RES as u32 * SCALE_LO_RES;
const COLOR_ON: sdl2::pixels::Color = sdl2::pixels::Color::RGB(255, 255, 255);
const COLOR_OFF: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0, 0, 0);

pub struct Window {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Window {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("CHIP-8 Interpreter", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(COLOR_OFF);
        canvas.clear();
        canvas.present();

        Window { canvas }
    }

    pub fn draw(&mut self, memory: &mut Memory, hi_res: bool) {
        let height = if hi_res { HEIGHT_HI_RES } else { HEIGHT_LO_RES };
        let width = if hi_res { WIDTH_HI_RES } else { WIDTH_LO_RES };
        let scale = if hi_res { SCALE_HI_RES } else { SCALE_LO_RES };

        for y in 0..height {
            for x in 0..width {
                let color = self.get_color(memory.read_vram(x, y));
                let x = (x as u32) * scale;
                let y = (y as u32) * scale;

                self.canvas.set_draw_color(color);

                let _ = self
                    .canvas
                    .fill_rect(sdl2::rect::Rect::new(x as i32, y as i32, scale, scale));
            }
        }

        self.canvas.present();
    }

    fn get_color(&mut self, value: u8) -> sdl2::pixels::Color {
        if value == 0 {
            COLOR_OFF
        } else {
            COLOR_ON
        }
    }
}
