package main

import (
	"flag"
	"log"

	"github.com/henningstorck/chip8-interpreter/emulation"

	"github.com/henningstorck/chip8-interpreter/ui"
	"github.com/veandco/go-sdl2/sdl"
)

const hz = 500

func main() {
	var fileName string
	flag.StringVar(&fileName, "rom", "", "The ROM file to load")
	flag.Parse()

	chip8 := emulation.Chip8{
		Memory: emulation.Memory{},
		Video:  emulation.Video{},
		Keypad: emulation.Keypad{},
	}

	chip8.Reset()

	if fileName != "" {
		if err := chip8.LoadProgram(fileName); err != nil {
			log.Fatalln("Cannot load program.", err)
		}
	}

	window, err := ui.CreateWindow()
	defer window.Destroy()

	if err != nil {
		log.Fatalln("Cannot initialize ui.", err)
	}

	for running := true; running; {
		if err := chip8.Cycle(); err != nil {
			log.Fatalln("Cannot process CPU cycle.", err)
		}

		if chip8.ShouldDraw() {
			chip8.Video.Draw(window)
		}

		for event := sdl.PollEvent(); event != nil; event = sdl.PollEvent() {
			switch eventType := event.(type) {
			case *sdl.QuitEvent:
				running = false
			case *sdl.KeyboardEvent:
				handledByKeypad := chip8.Keypad.HandleKeyPress(eventType)

				if !handledByKeypad && eventType.Type == sdl.KEYDOWN {
					switch eventType.Keysym.Sym {
					case sdl.K_i:
						chip8.Reset()
					case sdl.K_p:
						chip8.Paused = !chip8.Paused
					}
				}
			}
		}

		sdl.Delay(1000 / hz)
	}
}
