#[cfg(test)]
#[path = "./chip8_test.rs"]
mod chip8_test;

use crate::memory::Memory;
use crate::window::{HEIGHT_HI_RES, HEIGHT_LO_RES, WIDTH_HI_RES, WIDTH_LO_RES};
use rand::Rng;

const TIMER_EVERY_X_TICKS: usize = 8;
const OPCODE_SIZE: u16 = 2;

pub struct OutputState<'a> {
    pub memory: &'a mut Memory,
    pub draw_flag: bool,
    pub _beep: bool,
    pub hi_res: bool,
}

enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}

impl ProgramCounter {
    fn skip_if(condition: bool) -> ProgramCounter {
        if condition {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }
}

pub struct Chip8 {
    memory: Memory,
    draw_flag: bool,
    stack: [u16; 16],
    v: [u8; 16],
    i: u16,
    pc: u16,
    sp: u16,
    dt: u8,
    st: u8,
    keypad: [bool; 16],
    keypad_waiting: bool,
    keypad_register: usize,
    ticks: usize,
    super_chip: bool,
    hi_res: bool,
}

impl Chip8 {
    pub fn new(super_chip: bool) -> Self {
        Chip8 {
            memory: Memory::new(super_chip),
            draw_flag: false,
            stack: [0; 16],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            dt: 0,
            st: 0,
            keypad: [false; 16],
            keypad_waiting: false,
            keypad_register: 0,
            ticks: 0,
            super_chip,
            hi_res: false,
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;

            if addr < 4096 {
                self.memory.write_byte(0x200 + i, byte);
            } else {
                break;
            }
        }
    }

    pub fn tick(&mut self, keypad: [bool; 16]) -> OutputState {
        self.keypad = keypad;
        self.draw_flag = false;

        if self.keypad_waiting {
            for i in 0..keypad.len() {
                if keypad[i] {
                    self.keypad_waiting = false;
                    self.v[self.keypad_register] = i as u8;
                    break;
                }
            }
        } else {
            if self.ticks == TIMER_EVERY_X_TICKS - 1 {
                self.ticks = 0;

                if self.dt > 0 {
                    self.dt -= 1
                }

                if self.st > 0 {
                    self.st -= 1
                }
            } else {
                self.ticks += 1;
            }

            let opcode = self.get_opcode();
            self.run_opcode(opcode);
        }

        OutputState {
            memory: &mut self.memory,
            draw_flag: self.draw_flag,
            _beep: self.st > 0,
            hi_res: self.hi_res,
        }
    }

    fn get_opcode(&mut self) -> u16 {
        self.memory.read_word(self.pc as usize)
    }

    fn run_opcode(&mut self, opcode: u16) {
        let nibbles = (
            (opcode & 0xF000) >> 12u8,
            (opcode & 0x0F00) >> 8u8,
            (opcode & 0x00F0) >> 4u8,
            (opcode & 0x000F) as u8,
        );

        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3;

        let pc_change = match nibbles {
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x00, 0x00, 0x0f, 0x0e) => {
                if self.super_chip {
                    self.op_00fe()
                } else {
                    ProgramCounter::Next
                }
            }
            (0x00, 0x00, 0x0f, 0x0f) => {
                if self.super_chip {
                    self.op_00ff()
                } else {
                    ProgramCounter::Next
                }
            }
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xkk(x, kk),
            (0x04, _, _, _) => self.op_4xkk(x, kk),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xkk(x, kk),
            (0x07, _, _, _) => self.op_7xkk(x, kk),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8xy6(x, y),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8xye(x, y),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0a, _, _, _) => self.op_annn(nnn),
            (0x0b, _, _, _) => {
                if self.super_chip {
                    self.op_bxnn(x, nnn)
                } else {
                    self.op_bnnn(nnn)
                }
            }
            (0x0c, _, _, _) => self.op_cxkk(x, kk),
            (0x0d, _, _, _) => self.op_dxyn(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.op_ex9e(x),
            (0x0e, _, 0x0a, 0x01) => self.op_exa1(x),
            (0x0f, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.op_fx0a(x),
            (0x0f, _, 0x01, 0x05) => self.op_fx15(x),
            (0x0f, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.op_fx65(x),
            _ => ProgramCounter::Next,
        };

        match pc_change {
            ProgramCounter::Next => self.pc += OPCODE_SIZE,
            ProgramCounter::Skip => self.pc += 2 * OPCODE_SIZE,
            ProgramCounter::Jump(addr) => self.pc = addr,
        }
    }

    fn get_lsb(&self, value: u8) -> u8 {
        value & 0x01
    }

    fn get_msb(&self, value: u8) -> u8 {
        (value & 0b10000000) >> 7
    }

    fn op_00e0(&mut self) -> ProgramCounter {
        if self.super_chip && self.hi_res {
            for y in 0..HEIGHT_HI_RES {
                for x in 0..WIDTH_HI_RES {
                    self.memory.write_vram(x, y, 0);
                }
            }
        } else {
            for y in 0..HEIGHT_LO_RES {
                for x in 0..WIDTH_LO_RES {
                    self.memory.write_vram(x, y, 0);
                }
            }
        }

        self.draw_flag = false;
        ProgramCounter::Next
    }

    fn op_00ee(&mut self) -> ProgramCounter {
        self.sp -= 1;
        ProgramCounter::Jump(self.stack[self.sp as usize])
    }

    fn op_00fe(&mut self) -> ProgramCounter {
        self.hi_res = false;
        ProgramCounter::Next
    }

    fn op_00ff(&mut self) -> ProgramCounter {
        self.hi_res = true;
        ProgramCounter::Next
    }

    fn op_1nnn(&mut self, nnn: u16) -> ProgramCounter {
        ProgramCounter::Jump(nnn)
    }

    fn op_2nnn(&mut self, nnn: u16) -> ProgramCounter {
        self.stack[self.sp as usize] = self.pc + OPCODE_SIZE;
        self.sp += 1;
        ProgramCounter::Jump(nnn)
    }

    fn op_3xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x] == kk)
    }

    fn op_4xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x] != kk)
    }

    fn op_5xy0(&mut self, x: usize, y: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x] == self.v[y])
    }

    fn op_6xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        self.v[x] = kk;
        ProgramCounter::Next
    }

    fn op_7xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        let result = self.v[x].wrapping_add(kk);
        self.v[x] = result;
        ProgramCounter::Next
    }

    fn op_8xy0(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] = self.v[y];
        ProgramCounter::Next
    }

    fn op_8xy1(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] = self.v[x] | self.v[y];
        ProgramCounter::Next
    }

    fn op_8xy2(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] = self.v[x] & self.v[y];
        ProgramCounter::Next
    }

    fn op_8xy3(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] = self.v[x] ^ self.v[y];
        ProgramCounter::Next
    }

    fn op_8xy4(&mut self, x: usize, y: usize) -> ProgramCounter {
        let result = self.v[x] as u16 + self.v[y] as u16;
        self.v[x] = result as u8;
        self.v[0x0f] = if result > 0xFF { 1 } else { 0 };
        ProgramCounter::Next
    }

    fn op_8xy5(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[0x0f] = if self.v[x] >= self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        ProgramCounter::Next
    }

    fn op_8xy6(&mut self, x: usize, y: usize) -> ProgramCounter {
        if self.super_chip {
            self.v[0xf] = self.get_lsb(self.v[x]);
            self.v[x] = self.v[x] >> 1;
        } else {
            self.v[0xf] = self.get_lsb(self.v[y]);
            self.v[x] = self.v[y] >> 1;
        }

        ProgramCounter::Next
    }

    fn op_8xy7(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[0x0f] = if self.v[y] >= self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        ProgramCounter::Next
    }

    fn op_8xye(&mut self, x: usize, y: usize) -> ProgramCounter {
        if self.super_chip {
            self.v[0xf] = self.get_msb(self.v[x]);
            self.v[x] = self.v[x] << 1;
        } else {
            self.v[0xf] = self.get_msb(self.v[y]);
            self.v[x] = self.v[y] << 1;
        }

        ProgramCounter::Next
    }

    fn op_9xy0(&mut self, x: usize, y: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x] != self.v[y])
    }

    fn op_annn(&mut self, nnn: u16) -> ProgramCounter {
        self.i = nnn;
        ProgramCounter::Next
    }

    fn op_bnnn(&mut self, nnn: u16) -> ProgramCounter {
        ProgramCounter::Jump(self.v[0] as u16 + nnn)
    }

    fn op_bxnn(&mut self, x: usize, nnn: u16) -> ProgramCounter {
        ProgramCounter::Jump(self.v[x] as u16 + nnn)
    }

    fn op_cxkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        let mut rng = rand::thread_rng();
        self.v[x] = rng.gen::<u8>() & kk;
        ProgramCounter::Next
    }

    fn op_dxyn(&mut self, x: usize, y: usize, n: u8) -> ProgramCounter {
        self.v[0x0f] = 0;

        let width = if self.hi_res {
            WIDTH_HI_RES
        } else {
            WIDTH_LO_RES
        };

        let height = if self.hi_res {
            HEIGHT_HI_RES
        } else {
            HEIGHT_LO_RES
        };

        for byte in 0..(n as usize) {
            let y = (self.v[y] as usize + byte) % height;

            for bit in 0..8 {
                let x = (self.v[x] as usize + bit) % width;
                let color = (self.memory.read_byte(self.i as usize + byte) >> (7 - bit)) & 1;
                self.v[0x0f] |= color & self.memory.read_vram(x, y);
                self.memory.xor_vram(x, y, color);
            }
        }

        self.draw_flag = true;
        ProgramCounter::Next
    }

    fn op_ex9e(&mut self, x: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.keypad[self.v[x] as usize])
    }

    fn op_exa1(&mut self, x: usize) -> ProgramCounter {
        ProgramCounter::skip_if(!self.keypad[self.v[x] as usize])
    }

    fn op_fx07(&mut self, x: usize) -> ProgramCounter {
        self.v[x] = self.dt;
        ProgramCounter::Next
    }

    fn op_fx0a(&mut self, x: usize) -> ProgramCounter {
        self.keypad_waiting = true;
        self.keypad_register = x;
        ProgramCounter::Next
    }

    fn op_fx15(&mut self, x: usize) -> ProgramCounter {
        self.dt = self.v[x];
        ProgramCounter::Next
    }

    fn op_fx18(&mut self, x: usize) -> ProgramCounter {
        self.st = self.v[x];
        ProgramCounter::Next
    }

    fn op_fx1e(&mut self, x: usize) -> ProgramCounter {
        self.i = self.v[x] as u16 + self.i;
        ProgramCounter::Next
    }

    fn op_fx29(&mut self, x: usize) -> ProgramCounter {
        self.i = (self.v[x] as u16) * 5;
        ProgramCounter::Next
    }

    fn op_fx33(&mut self, x: usize) -> ProgramCounter {
        let a = self.v[x] / 100;
        let b = (self.v[x] % 100) / 10;
        let c = self.v[x] % 10;
        self.memory.write_byte(self.i as usize, a);
        self.memory.write_byte(self.i as usize + 1, b);
        self.memory.write_byte(self.i as usize + 2, c);
        ProgramCounter::Next
    }

    fn op_fx55(&mut self, x: usize) -> ProgramCounter {
        for i in 0..x + 1 {
            self.memory.write_byte(self.i as usize + i, self.v[i]);
        }

        ProgramCounter::Next
    }

    fn op_fx65(&mut self, x: usize) -> ProgramCounter {
        for i in 0..x + 1 {
            self.v[i] = self.memory.read_byte(self.i as usize + i);
        }

        ProgramCounter::Next
    }
}
