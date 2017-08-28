package main 

import (
	"github.com/nsf/termbox-go"
)

func main() {
	err := termbox.Init();
	if (err != nil) {
		panic(err)
	}
	defer termbox.Close();

	running := true
	xPos := 3;
	for running {
		e := termbox.PollEvent()
		if e.Key == termbox.KeyEsc {
			running = false
		} 
		if e.Key == termbox.KeyArrowRight {
			xPos++
		}
		termbox.Clear(termbox.ColorBlack, termbox.ColorBlack);
		buffer := termbox.CellBuffer()
		buffer[xPos].Fg = termbox.ColorBlue
		buffer[xPos].Ch = 'X'

		termbox.Flush();
	}
}