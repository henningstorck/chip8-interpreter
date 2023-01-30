package emulation

import (
	"fmt"
	"math/rand"
)

func (chip8 *Chip8) processOpcode() error {
	nibbles := []uint16{
		chip8.oc & 0xF000 >> 12,
		chip8.oc & 0x0F00 >> 8,
		chip8.oc & 0x00F0 >> 4,
		chip8.oc & 0x000F,
	}

	nnn := chip8.oc & 0x0FFF
	kk := chip8.oc & 0x00FF
	x := nibbles[1]
	y := nibbles[2]
	n := nibbles[3]

	switch chip8.oc & 0xF000 {
	case 0x0000:
		switch chip8.oc & 0x000F {
		case 0x0000:
			chip8.Video.Reset()
			chip8.shouldDraw = true
			chip8.pc += 2

		case 0x000E:
			chip8.sp = chip8.sp - 1
			chip8.pc = chip8.stack[chip8.sp]
			chip8.pc += 2

		default:
			return fmt.Errorf("invalid opcode %X", chip8.oc)
		}

	case 0x1000:
		chip8.pc = nnn

	case 0x2000:
		chip8.stack[chip8.sp] = chip8.pc
		chip8.sp = chip8.sp + 1
		chip8.pc = nnn

	case 0x3000:
		if uint16(chip8.vx[x]) == kk {
			chip8.pc += 4
		} else {
			chip8.pc += 2
		}

	case 0x4000:
		if uint16(chip8.vx[x]) != kk {
			chip8.pc += 4
		} else {
			chip8.pc += 2
		}

	case 0x5000:
		if chip8.vx[x] == chip8.vx[y] {
			chip8.pc += 4
		} else {
			chip8.pc += 2
		}

	case 0x6000:
		chip8.vx[x] = uint8(kk)
		chip8.pc += 2

	case 0x7000:
		chip8.vx[x] = chip8.vx[x] + uint8(kk)
		chip8.pc += 2

	case 0x8000:
		switch chip8.oc & 0x000F {
		case 0x0000:
			chip8.vx[x] = chip8.vx[y]
			chip8.pc += 2

		case 0x0001:
			chip8.vx[x] = chip8.vx[x] | chip8.vx[y]
			chip8.pc += 2

		case 0x0002:
			chip8.vx[x] = chip8.vx[x] & chip8.vx[y]
			chip8.pc += 2

		case 0x0003:
			chip8.vx[x] = chip8.vx[x] ^ chip8.vx[y]
			chip8.pc += 2

		case 0x0004:
			if chip8.vx[y] > 0xFF-chip8.vx[x] {
				chip8.vx[0xF] = 1
			} else {
				chip8.vx[0xF] = 0
			}

			chip8.vx[x] = chip8.vx[x] + chip8.vx[y]
			chip8.pc += 2

		case 0x0005:
			if chip8.vx[y] > chip8.vx[x] {
				chip8.vx[0xF] = 0
			} else {
				chip8.vx[0xF] = 1
			}

			chip8.vx[x] = chip8.vx[x] - chip8.vx[y]
			chip8.pc += 2

		case 0x0006:
			chip8.vx[0xF] = chip8.vx[x] & 0x1
			chip8.vx[x] = chip8.vx[x] >> 1
			chip8.pc += 2

		case 0x0007:
			if chip8.vx[x] > chip8.vx[y] {
				chip8.vx[0xF] = 0
			} else {
				chip8.vx[0xF] = 1
			}

			chip8.vx[x] = chip8.vx[y] - chip8.vx[x]
			chip8.pc += 2

		case 0x000E:
			chip8.vx[0xF] = chip8.vx[x] >> 7
			chip8.vx[x] = chip8.vx[x] << 1
			chip8.pc += 2
		default:
			return fmt.Errorf("invalid opcode %X", chip8.oc)
		}

	case 0x9000:
		if chip8.vx[x] != chip8.vx[y] {
			chip8.pc += 4
		} else {
			chip8.pc += 2
		}

	case 0xA000:
		chip8.iv = nnn
		chip8.pc += 2

	case 0xB000:
		chip8.pc = (nnn) + uint16(chip8.vx[0x0])

	case 0xC000:
		chip8.vx[x] = uint8(rand.Intn(256)) & uint8(kk)
		chip8.pc += 2

	case 0xD000:
		chip8.vx[0xF] = 0

		for j := uint16(0); j < n; j++ {
			pixel := chip8.Memory.Data[chip8.iv+j]

			for i := uint16(0); i < 8; i++ {
				if (pixel & (0x80 >> i)) != 0 {
					k := chip8.vx[y] + uint8(j)
					n := chip8.vx[x] + uint8(i)

					if chip8.Video.Read(n, k) {
						chip8.vx[0xF] = 1
					}

					chip8.Video.Invert(n, k)
				}
			}
		}

		chip8.shouldDraw = true
		chip8.pc += 2

	case 0xE000:
		switch kk {
		case 0x009E:
			if chip8.Keypad[chip8.vx[x]] == 1 {
				chip8.pc += 4
			} else {
				chip8.pc += 2
			}

		case 0x00A1:
			if chip8.Keypad[chip8.vx[x]] == 0 {
				chip8.pc += 4
			} else {
				chip8.pc += 2
			}

		default:
			return fmt.Errorf("invalid opcode %X", chip8.oc)
		}

	case 0xF000:
		switch kk {
		case 0x0007:
			chip8.vx[x] = chip8.delayTimer
			chip8.pc += 2

		case 0x000A:
			pressed := false

			for i := 0; i < len(chip8.Keypad); i++ {
				if chip8.Keypad[i] != 0 {
					chip8.vx[x] = uint8(i)
					pressed = true
				}
			}

			if !pressed {
				return nil
			}

			chip8.pc += 2

		case 0x0015:
			chip8.delayTimer = chip8.vx[x]
			chip8.pc += 2

		case 0x0018:
			chip8.soundTimer = chip8.vx[x]
			chip8.pc += 2

		case 0x001E:
			if chip8.iv+uint16(chip8.vx[x]) > 0xFFF {
				chip8.vx[0xF] = 1
			} else {
				chip8.vx[0xF] = 0
			}

			chip8.iv = chip8.iv + uint16(chip8.vx[x])
			chip8.pc += 2

		case 0x0029:
			chip8.iv = uint16(chip8.vx[x]) * 0x5
			chip8.pc += 2

		case 0x0033:
			chip8.Memory.Data[chip8.iv] = chip8.vx[x] / 100
			chip8.Memory.Data[chip8.iv+1] = (chip8.vx[x] % 100) / 10
			chip8.Memory.Data[chip8.iv+2] = chip8.vx[x] % 10
			chip8.pc += 2

		case 0x0055:
			for i := 0; i < int(x)+1; i++ {
				chip8.Memory.Data[uint16(i)+chip8.iv] = chip8.vx[i]
			}

			chip8.iv = x + 1
			chip8.pc += 2

		case 0x0065:
			for i := 0; i < int(x)+1; i++ {
				chip8.vx[i] = chip8.Memory.Data[chip8.iv+uint16(i)]
			}

			chip8.iv = x + 1
			chip8.pc += 2
		default:
			return fmt.Errorf("invalid opcode %X", chip8.oc)
		}

	default:
		return fmt.Errorf("invalid opcode %X", chip8.oc)
	}

	return nil
}
