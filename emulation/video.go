package emulation

const (
	Width  = 64
	Height = 32
)

type Drawable interface {
	ClearScreen()
	DrawPixel(x, y int32)
	Present()
}

type Video struct {
	Data [Width * Height]bool
}

func (video *Video) Read(x, y uint8) bool {
	if video.isValidAddress(x, y) {
		address := video.getAddress(x, y)
		return video.Data[address]
	}

	return false
}

func (video *Video) Write(x, y uint8, value bool) {
	if video.isValidAddress(x, y) {
		address := video.getAddress(x, y)
		video.Data[address] = value
	}
}

func (video *Video) Invert(x, y uint8) {
	if video.isValidAddress(x, y) {
		address := video.getAddress(x, y)
		video.Data[address] = !video.Data[address]
	}
}

func (video *Video) Reset() {
	video.Data = [Width * Height]bool{}
}

func (video *Video) Draw(drawable Drawable) {
	drawable.ClearScreen()

	for x := 0; x < Width; x++ {
		for y := 0; y < Height; y++ {
			if video.Read(uint8(x), uint8(y)) {
				drawable.DrawPixel(int32(x), int32(y))
			}
		}
	}

	drawable.Present()
}

func (video *Video) isValidAddress(x, y uint8) bool {
	return x < Width && y < Height
}

func (video *Video) getAddress(x, y uint8) uint16 {
	return uint16(x) + uint16(y)*Width
}
