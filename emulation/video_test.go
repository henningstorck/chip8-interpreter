package emulation_test

import (
	"testing"

	"github.com/henningstorck/chip8-interpreter/emulation"

	"github.com/stretchr/testify/assert"
)

func TestVideoRead(t *testing.T) {
	video := emulation.Video{}
	video.Data[72] = true
	assert.True(t, video.Read(8, 1))
	assert.False(t, video.Read(9, 1))
}

func TestVideoWrite(t *testing.T) {
	video := emulation.Video{}
	video.Write(8, 1, true)
	assert.True(t, video.Data[72])
	assert.False(t, video.Data[73])
}

func TestVideoInvert(t *testing.T) {
	video := emulation.Video{}
	video.Invert(8, 1)
	assert.True(t, video.Data[72])
	video.Invert(8, 1)
	assert.False(t, video.Data[72])
}

func TestVideoReset(t *testing.T) {
	video := emulation.Video{}
	video.Data[0] = true
	video.Reset()
	assert.False(t, video.Data[0])
}

func TestVideoInvalidAddress(t *testing.T) {
	video := emulation.Video{}
	assert.False(t, video.Read(100, 100))
	video.Invert(100, 100)
	assert.False(t, video.Read(100, 100))
}
