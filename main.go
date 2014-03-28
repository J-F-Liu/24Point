package main

import (
	"fmt"
	"github.com/lxn/walk"
	. "github.com/lxn/walk/declarative"
	"math/rand"
	"strconv"
	"time"
)

func main() {
	var aTE, bTE, cTE, dTE *walk.LineEdit
	var outTE *walk.TextEdit
	var mw *walk.MainWindow

	defer func() {
		if e := recover(); e != nil {
			walk.MsgBox(mw, "Error", fmt.Sprintf("%v", e), walk.MsgBoxIconInformation)
		}
	}()

	MainWindow{
		AssignTo: &mw,
		Title:    "24 Game",
		MinSize:  Size{600, 400},
		Layout:   VBox{},
		Children: []Widget{
			Composite{
				Layout: HBox{},
				Children: []Widget{
					Label{Text: "a"},
					LineEdit{AssignTo: &aTE},
					Label{Text: "b"},
					LineEdit{AssignTo: &bTE},
					Label{Text: "c"},
					LineEdit{AssignTo: &cTE},
					Label{Text: "d"},
					LineEdit{AssignTo: &dTE},
					PushButton{
						Text: "Go",
						OnClicked: func() {
							rand.Seed(time.Now().Unix())
							aTE.SetText(fmt.Sprint(rand.Intn(13) + 1))
							bTE.SetText(fmt.Sprint(rand.Intn(13) + 1))
							cTE.SetText(fmt.Sprint(rand.Intn(13) + 1))
							dTE.SetText(fmt.Sprint(rand.Intn(13) + 1))
						},
					},
				},
			},
			TextEdit{AssignTo: &outTE, ReadOnly: true},
			PushButton{
				Text: "Compute",
				OnClicked: func() {
					a, err := strconv.ParseInt(aTE.Text(), 10, 64)
					b, err := strconv.ParseInt(bTE.Text(), 10, 64)
					c, err := strconv.ParseInt(cTE.Text(), 10, 64)
					d, err := strconv.ParseInt(dTE.Text(), 10, 64)
					if err != nil {
						walk.MsgBox(mw, "Error", err.Error(), walk.MsgBoxIconInformation)
					}
					outTE.SetText("")
					for solution := range findSolution(a, b, c, d) {
						outTE.AppendText(fmt.Sprintf("%s = 24\r\n", solution))
					}
					if outTE.Text() == "" {
						outTE.SetText("No solution.")
					}
				},
			},
		},
	}.Run()
}
