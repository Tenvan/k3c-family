package watch

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"testing"
	"time"
)

const quiet = 2 * time.Second

// write legt name unter dir an (Ordner dazu) und setzt die Änderungszeit auf mod, damit der Test nicht von der
// Auflösung der Dateizeiten abhängt.
func write(t *testing.T, dir, name, text string, mod time.Time) {
	t.Helper()
	path := filepath.Join(dir, filepath.FromSlash(name))
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.Chtimes(path, mod, mod); err != nil {
		t.Fatal(err)
	}
}

func newWatcher(t *testing.T) (*Watcher, string, string) {
	t.Helper()
	data, public := t.TempDir(), t.TempDir()
	write(t, data, "economy.json", `{}`, time.Unix(1000, 0))
	write(t, public, "grafik/well.png", "png", time.Unix(1000, 0))
	return New([]Root{{Name: "data", FS: os.DirFS(data)}, {Name: "public", FS: os.DirFS(public)}}, quiet, time.Time{}), data, public
}

// Eine Änderung löst nach der Entprellfrist genau einen Vorgang aus, vorher keinen.
func TestEineAenderungNachFrist(t *testing.T) {
	w, data, _ := newWatcher(t)
	t0 := time.Unix(5000, 0)
	if got := w.Poll(t0); got != nil {
		t.Fatalf("ohne Änderung: %v", got)
	}
	write(t, data, "economy.json", `{"a":1}`, time.Unix(2000, 0))
	if got := w.Poll(t0.Add(time.Second)); got != nil {
		t.Fatalf("sofort nach der Änderung: %v", got)
	}
	if got := w.Poll(t0.Add(time.Second + quiet - time.Millisecond)); got != nil {
		t.Fatalf("vor Ablauf der Frist: %v", got)
	}
	got := w.Poll(t0.Add(time.Second + quiet))
	if !slices.Equal(got, []string{"data/economy.json"}) {
		t.Fatalf("nach der Frist: %v", got)
	}
	if again := w.Poll(t0.Add(10 * quiet)); again != nil {
		t.Fatalf("zweiter Vorgang ohne Änderung: %v", again)
	}
}

// Mehrere schnelle Änderungen (auch neue, gelöschte Dateien, Grafik und Sound) ergeben einen Vorgang mit allen Pfaden.
func TestSchnelleAenderungenEinVorgang(t *testing.T) {
	w, data, public := newWatcher(t)
	t0 := time.Unix(5000, 0)
	write(t, data, "worlds/nord.json", `{}`, time.Unix(2000, 0))
	if got := w.Poll(t0); got != nil {
		t.Fatalf("erste Änderung: %v", got)
	}
	write(t, public, "grafik/well.png", "png2", time.Unix(2001, 0))
	write(t, public, "audio/coin.ogg", "ogg", time.Unix(2001, 0))
	if got := w.Poll(t0.Add(quiet - time.Millisecond)); got != nil {
		t.Fatalf("zweite Änderung: %v", got)
	}
	if err := os.Remove(filepath.Join(data, "economy.json")); err != nil {
		t.Fatal(err)
	}
	if got := w.Poll(t0.Add(quiet + time.Second)); got != nil { // Löschen startet die Frist neu
		t.Fatalf("nach dem Löschen: %v", got)
	}
	got := w.Poll(t0.Add(2*quiet + time.Second))
	want := []string{"data/economy.json", "data/worlds/nord.json", "public/audio/coin.ogg", "public/grafik/well.png"}
	if !slices.Equal(got, want) {
		t.Fatalf("Pfade: %v, erwartet %v", got, want)
	}
}

// Ein fehlender Ordner ist kein Fehler; entsteht er später, zählt sein Inhalt als Änderung.
func TestFehlenderOrdner(t *testing.T) {
	dir := filepath.Join(t.TempDir(), "public")
	w := New([]Root{{Name: "public", FS: os.DirFS(dir)}}, quiet, time.Time{})
	t0 := time.Unix(5000, 0)
	if got := w.Poll(t0); got != nil {
		t.Fatalf("fehlender Ordner: %v", got)
	}
	write(t, dir, "sprites/a.png", "a", time.Unix(2000, 0))
	w.Poll(t0.Add(time.Second))
	if got := w.Poll(t0.Add(time.Second + quiet)); !slices.Equal(got, []string{"public/sprites/a.png"}) {
		t.Fatalf("neuer Ordner: %v", got)
	}
}

// Run prüft je Takt und ruft fire einmal je Vorgang; der Watcher schreibt nichts in die beobachteten Ordner.
func TestRunTaktUndNurLesen(t *testing.T) {
	w, data, public := newWatcher(t)
	tick := make(chan time.Time)
	fired := make(chan []string, 4)
	ctx, cancel := context.WithCancel(context.Background())
	done := make(chan struct{})
	go func() { Run(ctx, w, tick, func(p []string) { fired <- p }); close(done) }()
	t0 := time.Unix(5000, 0)
	for i := range 5 {
		tick <- t0.Add(time.Duration(i) * time.Second)
	}
	write(t, data, "economy.json", `{"b":2}`, time.Unix(3000, 0))
	before := tree(t, data, public)
	for i := 5; i < 10; i++ {
		tick <- t0.Add(time.Duration(i) * time.Second)
	}
	cancel()
	<-done
	close(fired)
	var all [][]string
	for p := range fired {
		all = append(all, p)
	}
	if len(all) != 1 || !slices.Equal(all[0], []string{"data/economy.json"}) {
		t.Fatalf("Vorgänge: %v", all)
	}
	if after := tree(t, data, public); !slices.Equal(after, before) {
		t.Fatalf("Ordner verändert:\n%v\n%v", before, after)
	}
}

// tree listet alle Dateien mit Größe und Änderungszeit.
func tree(t *testing.T, dirs ...string) []string {
	t.Helper()
	var out []string
	for _, dir := range dirs {
		err := filepath.WalkDir(dir, func(p string, d os.DirEntry, err error) error {
			if err != nil {
				return err
			}
			info, err := d.Info()
			if err != nil {
				return err
			}
			out = append(out, fmt.Sprint(p, info.Size(), info.ModTime().UnixNano()))
			return nil
		})
		if err != nil {
			t.Fatal(err)
		}
	}
	return out
}

// Grundstand nach dem Laden: Eine Datei, die seit since geändert wurde, löst nach der Frist (ab since) einen Vorgang aus.
func TestAenderungZwischenLadenUndGrundstand(t *testing.T) {
	dir := t.TempDir()
	write(t, dir, "alt.json", `{}`, time.Unix(1000, 0))
	write(t, dir, "neu.json", `{}`, time.Unix(3000, 0))
	write(t, dir, "zukunft.json", `{}`, time.Now().Add(24*time.Hour)) // falsche Uhr: zählt nicht als Änderung
	w := New([]Root{{Name: "data", FS: os.DirFS(dir)}}, quiet, time.Unix(2000, 0))
	if got := w.Poll(time.Unix(2000, 0).Add(quiet - time.Millisecond)); got != nil {
		t.Fatalf("vor der Frist: %v", got)
	}
	if got := w.Poll(time.Unix(2000, 0).Add(quiet)); !slices.Equal(got, []string{"data/neu.json"}) {
		t.Fatalf("nach der Frist: %v", got)
	}
}

// B-511: Temp-Dateien des atomaren Schreibens sind keine Änderung; erst die fertige Datei zählt.
func TestTempDateienSindKeineAenderung(t *testing.T) {
	w, _, public := newWatcher(t)
	t0 := time.Unix(5000, 0)
	_ = w.Poll(t0)
	write(t, public, "audio/.import-123", "halb", t0)
	if got := w.Poll(t0.Add(quiet + time.Second)); got != nil {
		t.Fatalf("Temp-Datei löste einen Vorgang aus: %v", got)
	}
	write(t, public, "audio/klang.ogg", "fertig", t0)
	_ = w.Poll(t0.Add(2 * (quiet + time.Second))) // erkennt die Datei, die Entprellfrist läuft
	got := w.Poll(t0.Add(4 * (quiet + time.Second)))
	if !slices.Equal(got, []string{"public/audio/klang.ogg"}) {
		t.Errorf("fertige Datei: %v", got)
	}
}
