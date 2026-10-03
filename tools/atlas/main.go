// Command atlas packt die im Spiel genutzten Figuren-Frames (data/sprites.json) zu Atlas-PNGs und einer
// Phaser-Multiatlas-Beschreibung (public/atlas/atlas.json). Die Ausgabe ist deterministisch (B-163).
package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	root := flag.String("root", ".", "Projektwurzel")
	out := flag.String("out", "public/atlas", "Ausgabeverzeichnis (relativ zur Wurzel oder absolut)")
	maxSize := flag.Int("max", 4096, "maximale Kantenlänge eines Atlas in Pixeln")
	flag.Parse()
	n, err := run(*root, *out, *maxSize)
	if err != nil {
		fmt.Fprintln(os.Stderr, "atlas:", err)
		os.Exit(1)
	}
	fmt.Printf("atlas: %d Atlas/Atlanten nach %s\n", n, *out)
}
