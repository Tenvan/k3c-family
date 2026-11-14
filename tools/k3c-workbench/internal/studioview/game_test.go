package studioview

import (
	"context"
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"time"
)

// B-847: „Im Spiel ansehen“ legt den Testspielstand voll-<seed> außerhalb des Repos an und startet das native Spiel mit
// festem Pfad, das ihn lädt; ohne EXE nennt die Meldung den Build-Task. Kein Bezug mehr auf dist/index.html oder game.html.

// repoRoot ist die Wurzel des Checkouts (tools/k3c-workbench/internal/studioview → ../../../..).
func repoRoot(t *testing.T) string {
	t.Helper()
	root, err := filepath.Abs(filepath.Join("..", "..", "..", ".."))
	if err != nil {
		t.Fatal(err)
	}
	return root
}

// gameRoot ist ein Checkout mit der Standard-Welt; exe legt eine native EXE (relativ zur Wurzel) an.
func gameRoot(t *testing.T, exe string) string {
	t.Helper()
	root := t.TempDir()
	raw, err := os.ReadFile(filepath.Join(repoRoot(t), "data", "worlds", "standard.json"))
	if err != nil {
		t.Fatal(err)
	}
	write(t, root, "data/worlds/standard.json", string(raw))
	if exe != "" {
		write(t, root, exe, "MZ")
	}
	return root
}

// newGame: Game mit Testspielständen in einem Temp-Ordner; gestartete Befehle landen in started statt im System.
func newGame(t *testing.T, root string) (g *Game, saves string, started *[]*exec.Cmd) {
	t.Helper()
	saves = t.TempDir()
	ctx, cancel := context.WithCancel(context.Background())
	t.Cleanup(cancel)
	g = NewGame(ctx, root, saves, nil)
	var cmds []*exec.Cmd
	g.launch = func(cmd *exec.Cmd) error { cmds = append(cmds, cmd); return nil }
	return g, saves, &cmds
}

// AC-01: Die Befehlszeile zeigt auf die native EXE mit festem Pfad und lädt voll-<seed>; kein game.html.
func TestImSpielAnsehenStartetNativesSpiel(t *testing.T) {
	root := gameRoot(t, "native/k3c-game/bin/k3c-game.exe")
	g, saves, started := newGame(t, root)
	msg, err := g.Open("abc")
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(msg, "voll-abc") {
		t.Errorf("Antwort %q", msg)
	}
	if len(*started) != 1 {
		t.Fatalf("%d Starts", len(*started))
	}
	cmd := (*started)[0]
	if want := filepath.Join(root, "native", "k3c-game", "bin", "k3c-game.exe"); cmd.Path != want || cmd.Dir != filepath.Clean(root) {
		t.Errorf("EXE %q in %q, erwartet %q in %q", cmd.Path, cmd.Dir, want, root)
	}
	if want := []string{"--saves", saves, "--load", "voll-abc", "--players", "1"}; !slices.Equal(cmd.Args[1:], want) {
		t.Errorf("Schalter %v, erwartet %v", cmd.Args[1:], want)
	}
	if line := strings.Join(cmd.Args, " "); strings.Contains(line, "game.html") || strings.Contains(line, "index.html") {
		t.Errorf("Befehlszeile mit Browser-Client: %s", line)
	}
	if _, err := os.Stat(filepath.Join(saves, "voll-abc.json")); err != nil {
		t.Errorf("Testspielstand fehlt: %v", err)
	}
	if _, err := os.Stat(filepath.Join(root, "saves")); !os.IsNotExist(err) {
		t.Errorf("saves/ im Checkout angelegt: %v", err)
	}
	// EXE läuft schon: ein zweiter Start ist erlaubt.
	if _, err := g.Open("abc"); err != nil || len(*started) != 2 {
		t.Errorf("zweiter Start: %v (%d Starts)", err, len(*started))
	}
}

// Ohne Build im Checkout gilt das Spiel-Paket dist/k3c-game mit seinem Ordner als Arbeitsordner.
func TestImSpielAnsehenSpielPaket(t *testing.T) {
	root := gameRoot(t, "dist/k3c-game/k3c-game.exe")
	g, _, started := newGame(t, root)
	if _, err := g.Open("abc"); err != nil {
		t.Fatal(err)
	}
	if cmd := (*started)[0]; cmd.Dir != filepath.Join(root, "dist", "k3c-game") || filepath.Base(cmd.Path) != "k3c-game.exe" {
		t.Errorf("EXE %q in %q", cmd.Path, cmd.Dir)
	}
}

// AC-02: Fehlt die EXE, nennt die Meldung task native:game:build und startet nichts; ungültige Seeds lehnt Open ab.
func TestImSpielAnsehenAblehnungen(t *testing.T) {
	g, saves, started := newGame(t, gameRoot(t, ""))
	if _, err := g.Open("abc"); err == nil || !strings.Contains(err.Error(), "task native:game:build") {
		t.Errorf("ohne EXE: %v", err)
	}
	if _, err := os.Stat(filepath.Join(saves, "voll-abc.json")); !os.IsNotExist(err) {
		t.Errorf("Spielstand trotz fehlender EXE angelegt: %v", err)
	}
	g2, _, _ := newGame(t, gameRoot(t, "native/k3c-game/bin/k3c-game.exe"))
	for _, seed := range []string{"", "Abc", "a b", "../x", strings.Repeat("a", 28)} {
		if _, err := g2.Open(seed); err == nil {
			t.Errorf("Seed %q angenommen", seed)
		}
	}
	g2.launch = func(*exec.Cmd) error { return errors.New("kaputt") }
	if _, err := g2.Open("abc"); err == nil || !strings.Contains(err.Error(), "kaputt") {
		t.Errorf("Startfehler: %v", err)
	}
	if len(*started) != 0 {
		t.Errorf("ohne EXE gestartet: %d", len(*started))
	}
}

// Nach Close startet Open nichts mehr; doppeltes Close ist harmlos.
func TestOpenNachCloseAbgelehnt(t *testing.T) {
	g, _, _ := newGame(t, gameRoot(t, "native/k3c-game/bin/k3c-game.exe"))
	if _, err := g.Open("abc"); err != nil {
		t.Fatal(err)
	}
	g.Close(time.Second)
	g.Close(time.Second)
	if _, err := g.Open("abc"); err == nil || !strings.Contains(err.Error(), "beendet") {
		t.Errorf("Open nach Close: %v", err)
	}
}
