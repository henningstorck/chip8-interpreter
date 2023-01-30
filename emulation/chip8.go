package emulation

import (
	"log"
)

const (
	programOffset    = 0x200
	timerEveryNTicks = 8
)

type Chip8 struct {
	Memory Memory
	Video  Video
	Keypad Keypad

	vx    [16]uint8
	stack [16]uint16

	oc uint16
	pc uint16
	sp uint16
	iv uint16

	delayTimer uint8
	soundTimer uint8

	Paused     bool
	shouldDraw bool
	ticks      int
}

func (chip8 *Chip8) Reset() {
	chip8.Video.Reset()
	chip8.Memory.Reset()

	chip8.vx = [16]uint8{}
	chip8.stack = [16]uint16{}

	chip8.oc = 0
	chip8.pc = programOffset
	chip8.sp = 0
	chip8.iv = 0

	chip8.delayTimer = 0
	chip8.soundTimer = 0

	chip8.Paused = false
	chip8.shouldDraw = true
	chip8.ticks = 0
}

func (chip8 *Chip8) LoadProgram(fileName string) error {
	return chip8.Memory.LoadProgram(fileName, programOffset)
}

func (chip8 *Chip8) Cycle() error {
	if chip8.Paused {
		return nil
	}

	chip8.oc = chip8.Memory.ReadWord(chip8.pc)
	err := chip8.processOpcode()

	if err != nil {
		return err
	}

	if chip8.ticks%timerEveryNTicks == 0 {
		if chip8.delayTimer > 0 {
			chip8.delayTimer = chip8.delayTimer - 1
		}

		if chip8.soundTimer > 0 {
			if chip8.soundTimer == 1 {
				log.Println("Beep!")
			}

			chip8.soundTimer = chip8.soundTimer - 1
		}
	}

	chip8.ticks++
	return nil
}

func (chip8 *Chip8) ShouldDraw() bool {
	shouldDraw := chip8.shouldDraw
	chip8.shouldDraw = false
	return shouldDraw
}
