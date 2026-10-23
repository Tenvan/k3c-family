// Command soundatlas packt die erste Präferenz jedes Effekt-Slots aus data/assets.json (Bus sfx, ohne Schleifen) zu einem
// Sound-Atlas public/audio/atlas.ogg und atlas.mp3 mit atlas.json (Start und Dauer je Slot, SO6.2, B-494).
// ffmpeg dekodiert und kodiert; die Reihenfolge folgt der Slot-ID, die Ausgabe hängt nur von den Eingaben ab.
package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	root := flag.String("root", ".", "Projektwurzel")
	ffmpeg := flag.String("ffmpeg", "ffmpeg", "Pfad zu ffmpeg")
	flag.Parse()
	n, err := run(*root, *ffmpeg)
	if err != nil {
		fmt.Fprintln(os.Stderr, "soundatlas:", err)
		os.Exit(1)
	}
	fmt.Printf("soundatlas: %d Effekte nach public/audio/atlas.{ogg,mp3,json}\n", n)
}
