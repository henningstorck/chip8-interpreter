package ui

import (
	"github.com/henningstorck/chip8-interpreter/emulation"
	"github.com/veandco/go-sdl2/sdl"
)

const (
	title = "CHIP-8"
	scale = 16
)

var (
	colorOff = []uint8{0x10, 0x1D, 0x2B, 0xFF}
	colorOn  = []uint8{0x90, 0x91, 0x85, 0xFF}
)

type Window struct {
	sdlWindow   *sdl.Window
	sdlRenderer *sdl.Renderer
}

func CreateWindow() (Window, error) {
	sdlWin, err := createSdlWindow()

	if err != nil {
		return Window{}, err
	}

	sdlRend, err := createSdlRenderer(sdlWin)

	if err != nil {
		return Window{}, err
	}

	return Window{
		sdlWindow:   sdlWin,
		sdlRenderer: sdlRend,
	}, nil
}

func createSdlWindow() (*sdl.Window, error) {
	if err := sdl.Init(uint32(sdl.INIT_EVERYTHING)); err != nil {
		return nil, err
	}

	width := int32(emulation.Width * scale)
	height := int32(emulation.Height * scale)

	return sdl.CreateWindow(title, int32(sdl.WINDOWPOS_UNDEFINED), int32(sdl.WINDOWPOS_UNDEFINED), width, height, uint32(sdl.WINDOW_SHOWN))
}

func createSdlRenderer(sdlWin *sdl.Window) (*sdl.Renderer, error) {
	return sdl.CreateRenderer(sdlWin, -1, sdl.RENDERER_ACCELERATED)
}

func (window Window) ClearScreen() {
	window.sdlRenderer.SetDrawColorArray(colorOff...)
	window.sdlRenderer.FillRect(nil)
}

func (window Window) DrawPixel(x, y int32) {
	window.sdlRenderer.SetDrawColorArray(colorOn...)
	window.sdlRenderer.FillRect(&sdl.Rect{
		X: x * scale,
		Y: y * scale,
		W: scale,
		H: scale,
	})
}

func (window Window) Present() {
	window.sdlRenderer.Present()
}

func (win Window) Destroy() {
	sdl.Quit()
	win.sdlWindow.Destroy()
	win.sdlRenderer.Destroy()
}
