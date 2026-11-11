package e2e

// Bilder der Stationen (NG9.1): Die gebaute EXE (Umgebung K3C_GAME_EXE) nimmt den Ablauf im Aufnahme-Modus zweimal
// auf (--flow --at mit den Takten der Stationen aus dem Lauf ohne Fenster); beide Läufe müssen bildgleich sein. Die
// Bilder liegen unter K3C_E2E_OUT (sonst im Test-Ordner) in a/ und b/, dazu stationen.txt mit Bild, Station und Takt.

import (
	"bytes"
	"context"
	"fmt"
	"image"
	"image/png"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
	"testing"
	"time"
)

const (
	exeEnv   = "K3C_GAME_EXE" // Pfad der gebauten EXE (task native:game:e2e setzt ihn)
	rootEnv  = "K3C_ROOT"     // Arbeitsordner der EXE (public/grafik, native/k3c-rig/assets); Standard: Repo-Wurzel
	outEnv   = "K3C_E2E_OUT"  // Ordner der Bilder; Standard: Test-Ordner
	shotSize = "1280x720"
)

func TestFlowImages(t *testing.T) {
	if testing.Short() {
		t.Skip("Ende-zu-Ende mit Fenster: nur ohne -short")
	}
	exe := os.Getenv(exeEnv)
	if exe == "" {
		t.Skipf("%s nicht gesetzt (task native:game:e2e)", exeEnv)
	}
	probe := filepath.Join(exeRoot(), "public", "grafik", "objekte", "tanne", "manifest.json")
	if _, err := os.Stat(probe); err != nil {
		t.Skipf("gerenderte Grafik fehlt (%s; erst task gen:paper)", probe)
	}
	r := flow(t)
	at := make([]string, len(r.stations))
	var index strings.Builder
	for i, s := range r.stations {
		at[i] = strconv.Itoa(s.Tick)
		fmt.Fprintf(&index, "shot-%02d.png  %-12s Takt %d\n", i+1, s.Name, s.Tick)
	}
	out := os.Getenv(outEnv)
	if out == "" {
		out = t.TempDir()
	}
	for _, run := range []string{"a", "b"} {
		shoot(t, exe, filepath.Join(out, run), strings.Join(at, ","))
	}
	if err := os.WriteFile(filepath.Join(out, "stationen.txt"), []byte(index.String()), 0o644); err != nil {
		t.Fatal(err)
	}
	for i, s := range r.stations {
		name := fmt.Sprintf("shot-%02d.png", i+1)
		if !samePixels(t, filepath.Join(out, "a", name), filepath.Join(out, "b", name)) {
			t.Errorf("❌ %s (%s, Takt %d): Läufe a und b nicht bildgleich", name, s.Name, s.Tick)
		}
	}
	t.Logf("💾 %d Bilder je Lauf unter %s", len(r.stations), out)
}

// shoot startet die EXE im Aufnahme-Modus des Ablaufs und schreibt die Bilder nach dir.
func shoot(t *testing.T, exe, dir, at string) {
	t.Helper()
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Minute)
	defer cancel()
	cmd := exec.CommandContext(ctx, exe, "--flow", "--input", flowScript, "--shots", dir, "--at", at, "--size", shotSize)
	cmd.Dir = exeRoot()
	var log bytes.Buffer
	cmd.Stdout, cmd.Stderr = &log, &log
	if err := cmd.Run(); err != nil {
		t.Fatalf("❌ %s: %v\n%s", exe, err, lastLines(log.String(), 30))
	}
}

// exeRoot ist der Arbeitsordner der EXE: K3C_ROOT, sonst die Repo-Wurzel.
func exeRoot() string {
	if root := os.Getenv(rootEnv); root != "" {
		return root
	}
	return filepath.Join("..", "..", "..")
}

// samePixels vergleicht zwei PNG Bildpunkt für Bildpunkt.
func samePixels(t *testing.T, a, b string) bool {
	t.Helper()
	ia, ib := readPNG(t, a), readPNG(t, b)
	if ia.Bounds() != ib.Bounds() {
		return false
	}
	for y := ia.Bounds().Min.Y; y < ia.Bounds().Max.Y; y++ {
		for x := ia.Bounds().Min.X; x < ia.Bounds().Max.X; x++ {
			if ia.At(x, y) != ib.At(x, y) {
				return false
			}
		}
	}
	return true
}

func readPNG(t *testing.T, path string) image.Image {
	t.Helper()
	f, err := os.Open(path)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = f.Close() }()
	img, err := png.Decode(f)
	if err != nil {
		t.Fatalf("%s: %v", path, err)
	}
	return img
}

// lastLines liefert die letzten n Zeilen von s.
func lastLines(s string, n int) string {
	lines := strings.Split(strings.TrimRight(s, "\n"), "\n")
	return strings.Join(lines[max(0, len(lines)-n):], "\n")
}
