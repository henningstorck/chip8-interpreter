package emulation_test

import (
	"testing"

	"github.com/henningstorck/chip8-interpreter/emulation"

	"github.com/stretchr/testify/assert"
	"github.com/veandco/go-sdl2/sdl"
)

func TestKeypadKeyDown(t *testing.T) {
	pad := emulation.Keypad{}
	pad.HandleKeyPress(createKeyboardEvent(true, sdl.K_1))
	assert.Equal(t, uint8(0x1), pad[0x1])
}

func TestKeypadKeyUp(t *testing.T) {
	pad := emulation.Keypad{}
	pad.HandleKeyPress(createKeyboardEvent(false, sdl.K_1))
	assert.Equal(t, uint8(0x0), pad[0x1])
}

func TestKeypadKeyDownAndUp(t *testing.T) {
	pad := emulation.Keypad{}
	pad.HandleKeyPress(createKeyboardEvent(true, sdl.K_1))
	pad.HandleKeyPress(createKeyboardEvent(false, sdl.K_1))
	assert.Equal(t, uint8(0x0), pad[0x1])
}

func TestKeypadKeyUpAndDown(t *testing.T) {
	pad := emulation.Keypad{}
	pad.HandleKeyPress(createKeyboardEvent(false, sdl.K_1))
	pad.HandleKeyPress(createKeyboardEvent(true, sdl.K_1))
	assert.Equal(t, uint8(0x1), pad[0x1])
}

func createKeyboardEvent(keyDown bool, keycode sdl.Keycode) *sdl.KeyboardEvent {
	keyboardEvent := &sdl.KeyboardEvent{
		Keysym: sdl.Keysym{
			Sym: keycode,
		},
	}

	if keyDown {
		keyboardEvent.Type = sdl.KEYDOWN
	} else {
		keyboardEvent.Type = sdl.KEYUP
	}

	return keyboardEvent
}
