// Command rustpaket ergänzt den Paketbau des Rust-Spiels (`task rust:game:build`, RU57): `lizenzen` schreibt die
// Lizenzdatei LIZENZEN.txt nach stdout (B-1121), `pruefen` prüft einen Paketordner gegen die feste Liste (B-1137).
// Nur Go-Standardbibliothek; die Crate-Liste kommt aus `cargo metadata`.
package main

import (
	"fmt"
	"os"
)

const usage = `Aufruf: rustpaket lizenzen -exe <k3c-game.exe> [-rust rust] [-target x86_64-pc-windows-msvc]`

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, usage)
		os.Exit(2)
	}
	var err error
	switch os.Args[1] {
	case "lizenzen":
		err = cmdLizenzen(os.Args[2:], os.Stdout)
	default:
		fmt.Fprintln(os.Stderr, usage)
		os.Exit(2)
	}
	if err != nil {
		fmt.Fprintln(os.Stderr, "❌ rustpaket:", err)
		os.Exit(1)
	}
}
