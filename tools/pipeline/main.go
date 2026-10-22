// Command pipeline rechnet die Grafiken aus docs/assets/pipeline-liste.json deterministisch auf die Ziel-Palette DB32
// und die Pixeldichte 16 px um (B-331, GR8.2). Quelle und Ergebnis liegen getrennt: aus public/grafik/<pfad> wird
// public/grafik-db32/<pfad>. Regeln: docs/assets/stil-bibel.md.
package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
)

func main() {
	root := flag.String("root", ".", "Projektwurzel")
	list := flag.String("list", "docs/assets/pipeline-liste.json", "Liste der Grafiken (relativ zur Wurzel)")
	src := flag.String("src", "public/grafik", "Quellordner (relativ zur Wurzel)")
	out := flag.String("out", "public/grafik-db32", "Ergebnisordner (relativ zur Wurzel)")
	flag.Parse()
	n, err := run(filepath.Join(*root, *list), filepath.Join(*root, *src), filepath.Join(*root, *out))
	if err != nil {
		fmt.Fprintln(os.Stderr, "❌ pipeline:", err)
		os.Exit(1)
	}
	fmt.Printf("✅ pipeline: %d Grafiken nach %s umgerechnet\n", n, *out)
}
