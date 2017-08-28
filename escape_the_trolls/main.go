package main 

import (
	"github.com/nsf/termbox-go"
)

type Vec2 struct {
	X int
	Y int
}

func main() {
	err := termbox.Init();
	if (err != nil) {
		panic(err)
	}
	defer termbox.Close();

	running := true
	var pos Vec2
	playerRune := 'V'
	for running {
		event := termbox.PollEvent()
		if event.Key == termbox.KeyEsc {
			running = false
		} else {
			switch event.Ch {
				case 'd':
					playerRune = '>'
					pos.X++
				case 'a':
					playerRune = '<'
					pos.X--
				case 's':
					playerRune = 'V'
					pos.Y++
				case 'w':
					playerRune = '^'
					pos.Y--
			}
		}
		termbox.Clear(termbox.ColorBlack, termbox.ColorMagenta);
		termbox.SetCell(pos.X, pos.Y, playerRune, termbox.ColorBlack, termbox.ColorWhite)

		termbox.Flush();
	}
}