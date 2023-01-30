package emulation_test

import (
	"github.com/henningstorck/chip8-interpreter/emulation"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMemoryReadWord(t *testing.T) {
	mem := emulation.Memory{}
	mem.Data[0x0] = 0x12
	mem.Data[0x1] = 0x34
	mem.Data[0x2] = 0x56
	mem.Data[0x3] = 0x78
	assert.Equal(t, uint16(0x1234), mem.ReadWord(0x0))
	assert.Equal(t, uint16(0x5678), mem.ReadWord(0x2))
}

func TestMemoryReset(t *testing.T) {
	mem := emulation.Memory{}
	mem.Init[0x200] = 0x1
	mem.Data[0x200] = 0x2
	mem.Reset()
	assert.Equal(t, uint8(0xf0), mem.Data[0x0])
	assert.Equal(t, uint8(0x80), mem.Data[0x49])
	assert.Equal(t, uint8(0x01), mem.Data[0x200])
}
