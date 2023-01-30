package emulation

import "github.com/veandco/go-sdl2/sdl"

type Keypad [16]uint8

var mapping = map[sdl.Keycode]uint8{
	sdl.K_1: 0x1,
	sdl.K_2: 0x2,
	sdl.K_3: 0x3,
	sdl.K_4: 0xc,
	sdl.K_q: 0x4,
	sdl.K_w: 0x5,
	sdl.K_e: 0x6,
	sdl.K_r: 0xd,
	sdl.K_a: 0x7,
	sdl.K_s: 0x8,
	sdl.K_d: 0x9,
	sdl.K_f: 0xe,
	sdl.K_z: 0xa,
	sdl.K_y: 0xa,
	sdl.K_x: 0x0,
	sdl.K_c: 0xb,
	sdl.K_v: 0xf,
}

func (keypad *Keypad) HandleKeyPress(eventType *sdl.KeyboardEvent) bool {
	if eventType.Type == sdl.KEYDOWN || eventType.Type == sdl.KEYUP {
		code := eventType.Keysym.Sym
		pressed := eventType.Type == sdl.KEYDOWN
		key, mapped := mapping[code]

		if mapped {
			if pressed {
				keypad[key] = 0x1
			} else {
				keypad[key] = 0x0
			}

			return true
		}
	}

	return false
}
