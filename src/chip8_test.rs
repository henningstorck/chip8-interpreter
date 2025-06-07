use super::*;

const START_PC: u16 = 0xF00;
const NEXT_PC: u16 = START_PC + OPCODE_SIZE;
const SKIPPED_PC: u16 = START_PC + (2 * OPCODE_SIZE);

fn create_cpu(super_chip: bool) -> Chip8 {
    let mut cpu = Chip8::new(super_chip);
    cpu.pc = START_PC;
    cpu.v = [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7];
    cpu
}

fn test_math_op(super_chip: bool, v1: u8, v2: u8, op: u16, result: u8, vf: u8) {
    let mut cpu = create_cpu(super_chip);
    cpu.v[0] = v1;
    cpu.v[1] = v2;
    cpu.v[0x0f] = 0;
    cpu.run_opcode(0x8010 + op);
    assert_eq!(cpu.v[0], result);
    assert_eq!(cpu.v[0x0f], vf);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_00e0() {
    let mut cpu = create_cpu(false);

    for y in 0..HEIGHT_LO_RES {
        for x in 0..WIDTH_LO_RES {
            cpu.memory.write_vram(x, y, 128);
        }
    }

    cpu.run_opcode(0x00e0);

    for y in 0..HEIGHT_LO_RES {
        for x in 0..WIDTH_LO_RES {
            assert_eq!(cpu.memory.read_vram(x, y), 0);
        }
    }

    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_00ee() {
    let mut cpu = create_cpu(false);
    cpu.sp = 5;
    cpu.stack[4] = 0x6666;
    cpu.run_opcode(0x00ee);
    assert_eq!(cpu.sp, 4);
    assert_eq!(cpu.pc, 0x6666);
}

#[test]
fn test_op_1nnn() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0x1666);
    assert_eq!(cpu.pc, 0x0666);
}

#[test]
fn test_op_2nnn() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0x2666);
    assert_eq!(cpu.pc, 0x0666);
    assert_eq!(cpu.sp, 1);
    assert_eq!(cpu.stack[0], NEXT_PC);
}

#[test]
fn test_op_3xkk_skip() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0x3201);
    assert_eq!(cpu.pc, SKIPPED_PC);
}

#[test]
fn test_op_3xkk_next() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0x3200);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_4xkk_skip() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0x4200);
    assert_eq!(cpu.pc, SKIPPED_PC);
}

#[test]
fn test_op_4xkk_next() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0x4201);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_5xy0_skip() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0x5010);
    assert_eq!(cpu.pc, SKIPPED_PC);
}

#[test]
fn test_op_5xy0_next() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0x5020);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_6xkk() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0x65ff);
    assert_eq!(cpu.v[5], 0xff);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_7xkk() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0x781f);
    assert_eq!(cpu.v[8], 0x23);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_8xy0() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0x8050);
    assert_eq!(cpu.v[0], 0x02);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_8xy1() {
    test_math_op(false, 0x0F, 0xF0, 1, 0xFF, 0);
}

#[test]
fn test_op_8xy2() {
    test_math_op(false, 0xFF, 0xF0, 2, 0xF0, 0);
}

#[test]
fn test_op_8xy3() {
    test_math_op(false, 0xFF, 0xF0, 3, 0x0F, 0);
}

#[test]
fn test_op_8xy4() {
    test_math_op(false, 0x0F, 0x0F, 4, 0x1E, 0);
}

#[test]
fn test_op_8xy4_overflow() {
    test_math_op(false, 0xFF, 0xFF, 4, 0xFE, 1);
}

#[test]
fn test_op_8xy5() {
    test_math_op(false, 0x0F, 0x01, 5, 0x0E, 1);
}

#[test]
fn test_op_8xy5_overflow() {
    test_math_op(false, 0x0F, 0xFF, 5, 0x10, 0);
}

#[test]
fn test_op_8xy5_equality() {
    test_math_op(false, 0x0F, 0x0F, 5, 0x00, 1);
}

#[test]
fn test_op_8xy6() {
    test_math_op(false, 0, 0x04, 6, 0x02, 0);
}

#[test]
fn test_op_8xy6_overflow() {
    test_math_op(false, 0, 0x05, 6, 0x02, 1);
}

#[test]
fn test_op_8xy6_chip48() {
    test_math_op(true, 0x04, 0, 6, 0x02, 0);
}

#[test]
fn test_op_8xy6_overflow_chip48() {
    test_math_op(true, 0x05, 0, 6, 0x02, 1);
}

#[test]
fn test_op_8xy7() {
    test_math_op(false, 0xFF, 0x0F, 7, 0x10, 0);
}

#[test]
fn test_op_8xy7_overflow() {
    test_math_op(false, 0x01, 0x0F, 7, 0x0E, 1);
}

#[test]
fn test_op_8xy7_equality() {
    test_math_op(false, 0x0F, 0x0F, 7, 0x00, 1);
}

#[test]
fn test_op_8xye() {
    test_math_op(false, 0, 0b00000111, 0x0E, 0b00001110, 0);
}

#[test]
fn test_op_8xye_overflow() {
    test_math_op(false, 0, 0b11000000, 0x0E, 0b10000000, 1);
}

#[test]
fn test_op_8xye_chip48() {
    test_math_op(true, 0b00000111, 0, 0x0E, 0b00001110, 0);
}

#[test]
fn test_op_8xye_overflow_chip48() {
    test_math_op(true, 0b11000000, 0, 0x0E, 0b10000000, 1);
}

#[test]
fn test_op_9xy0_skip() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0x90e0);
    assert_eq!(cpu.pc, SKIPPED_PC);
}

#[test]
fn test_op_9xy0_next() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0x9010);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_annn() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0xa123);
    assert_eq!(cpu.i, 0x123);
}

#[test]
fn test_op_bnnn() {
    let mut cpu = create_cpu(false);
    cpu.v[0] = 3;
    cpu.run_opcode(0xb123);
    assert_eq!(cpu.pc, 0x126);
}

#[test]
fn test_op_bxnn() {
    let mut cpu = create_cpu(true);
    cpu.v[1] = 3;
    cpu.run_opcode(0xb123);
    assert_eq!(cpu.pc, 0x126);
}

#[test]
fn test_op_cxkk_and() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0xc000);
    assert_eq!(cpu.v[0], 0);
    cpu.run_opcode(0xc00f);
    assert_eq!(cpu.v[0] & 0xf0, 0);
}

#[test]
fn test_op_dxyn() {
    let mut cpu = create_cpu(false);
    cpu.i = 0;
    cpu.memory.write_byte(0, 0b11111111);
    cpu.memory.write_byte(1, 0b00000000);
    cpu.memory.write_vram(0, 0, 1);
    cpu.memory.write_vram(1, 0, 0);
    cpu.memory.write_vram(0, 1, 1);
    cpu.memory.write_vram(1, 1, 0);
    cpu.v[0] = 0;
    cpu.run_opcode(0xd002);
    assert_eq!(cpu.memory.read_vram(0, 0), 0);
    assert_eq!(cpu.memory.read_vram(1, 0), 1);
    assert_eq!(cpu.memory.read_vram(0, 1), 1);
    assert_eq!(cpu.memory.read_vram(1, 1), 0);
    assert_eq!(cpu.v[0x0f], 1);
    assert!(cpu.draw_flag);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_dxyn_wrap_horizontal() {
    let mut cpu = create_cpu(false);
    let x = WIDTH_LO_RES - 4;
    cpu.i = 0;
    cpu.memory.write_byte(0, 0b11111111);
    cpu.v[0] = x as u8;
    cpu.v[1] = 0;
    cpu.run_opcode(0xd011);
    assert_eq!(cpu.memory.read_vram(x - 1, 0), 0);
    assert_eq!(cpu.memory.read_vram(x, 0), 1);
    assert_eq!(cpu.memory.read_vram(x + 1, 0), 1);
    assert_eq!(cpu.memory.read_vram(x + 2, 0), 1);
    assert_eq!(cpu.memory.read_vram(x + 3, 0), 1);
    assert_eq!(cpu.memory.read_vram(0, 0), 1);
    assert_eq!(cpu.memory.read_vram(1, 0), 1);
    assert_eq!(cpu.memory.read_vram(2, 0), 1);
    assert_eq!(cpu.memory.read_vram(3, 0), 1);
    assert_eq!(cpu.memory.read_vram(4, 0), 0);
    assert_eq!(cpu.v[0x0f], 0);
}

#[test]
fn test_op_dxyn_wrap_vertical() {
    let mut cpu = create_cpu(false);
    let y = HEIGHT_LO_RES - 1;
    cpu.i = 0;
    cpu.memory.write_byte(0, 0b11111111);
    cpu.memory.write_byte(1, 0b11111111);
    cpu.v[0] = 0;
    cpu.v[1] = y as u8;
    cpu.run_opcode(0xd012);
    assert_eq!(cpu.memory.read_vram(0, y), 1);
    assert_eq!(cpu.memory.read_vram(0, 0), 1);
    assert_eq!(cpu.v[0x0f], 0);
}

#[test]
fn test_op_ex9e() {
    let mut cpu = create_cpu(false);
    cpu.keypad[9] = true;
    cpu.v[5] = 9;
    cpu.run_opcode(0xe59e);
    assert_eq!(cpu.pc, SKIPPED_PC);
    let mut cpu = create_cpu(false);
    cpu.v[5] = 9;
    cpu.run_opcode(0xe59e);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_exa1() {
    let mut cpu = create_cpu(false);
    cpu.keypad[9] = true;
    cpu.v[5] = 9;
    cpu.run_opcode(0xe5a1);
    assert_eq!(cpu.pc, NEXT_PC);
    let mut processor = create_cpu(false);
    processor.v[5] = 9;
    processor.run_opcode(0xe5a1);
    assert_eq!(processor.pc, SKIPPED_PC);
}

#[test]
fn test_op_fx07() {
    let mut cpu = create_cpu(false);
    cpu.dt = 20;
    cpu.run_opcode(0xf507);
    assert_eq!(cpu.v[5], 20);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_fx0a() {
    let mut cpu = create_cpu(false);
    cpu.run_opcode(0xf50a);
    assert_eq!(cpu.keypad_waiting, true);
    assert_eq!(cpu.keypad_register, 5);
    assert_eq!(cpu.pc, NEXT_PC);
    cpu.tick([false; 16]);
    assert_eq!(cpu.keypad_waiting, true);
    assert_eq!(cpu.keypad_register, 5);
    assert_eq!(cpu.pc, NEXT_PC);
    cpu.tick([true; 16]);
    assert_eq!(cpu.keypad_waiting, false);
    assert_eq!(cpu.v[5], 0);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_fx15() {
    let mut cpu = create_cpu(false);
    cpu.v[5] = 9;
    cpu.run_opcode(0xf515);
    assert_eq!(cpu.dt, 9);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_fx18() {
    let mut cpu = create_cpu(false);
    cpu.v[5] = 9;
    cpu.run_opcode(0xf518);
    assert_eq!(cpu.st, 9);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_fx1e() {
    let mut cpu = create_cpu(false);
    cpu.v[5] = 9;
    cpu.i = 9;
    cpu.run_opcode(0xf51e);
    assert_eq!(cpu.i, 18);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_fx29() {
    let mut cpu = create_cpu(false);
    cpu.v[5] = 9;
    cpu.run_opcode(0xf529);
    assert_eq!(cpu.i, 5 * 9);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_fx33() {
    let mut cpu = create_cpu(false);
    cpu.v[5] = 123;
    cpu.i = 1000;
    cpu.run_opcode(0xf533);
    assert_eq!(cpu.memory.read_byte(1000), 1);
    assert_eq!(cpu.memory.read_byte(1001), 2);
    assert_eq!(cpu.memory.read_byte(1002), 3);
    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_fx55() {
    let mut cpu = create_cpu(false);
    cpu.i = 1000;
    cpu.run_opcode(0xff55);

    for i in 0..16 {
        assert_eq!(cpu.memory.read_byte(1000 + i), cpu.v[i]);
    }

    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_op_fx65() {
    let mut cpu = create_cpu(false);

    for i in 0..16usize {
        cpu.memory.write_byte(1000 + i, i as u8);
    }

    cpu.i = 1000;
    cpu.run_opcode(0xff65);

    for i in 0..16usize {
        assert_eq!(cpu.v[i], cpu.memory.read_byte(1000 + i));
    }

    assert_eq!(cpu.pc, NEXT_PC);
}

#[test]
fn test_timers() {
    let mut cpu = create_cpu(false);
    cpu.dt = 200;
    cpu.st = 100;

    for _ in 0..8 {
        cpu.tick([false; 16]);
    }

    assert_eq!(cpu.dt, 199);
    assert_eq!(cpu.st, 99);
}
