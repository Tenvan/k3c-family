package local

import (
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"os"
	"path/filepath"
	"testing"

	"k3c/engine/room"
	"k3c/engine/sim"
	"k3c/engine/store"
	"k3c/native/k3c-game/view"
)

// stepN taktet n Mal mit Bewegung nach rechts und liefert den letzten Frame.
func stepN(t *testing.T, s *Session, n int) view.Frame {
	t.Helper()
	var f view.Frame
	for range n {
		if err := s.Command(0, view.Command{Move: 1}); err != nil {
			t.Fatal(err)
		}
		f = s.Step()
	}
	return f
}

// fileHash ist der SHA-256 der Spielstand-Datei name.
func fileHash(t *testing.T, dir, name string) string {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join(dir, name+".json"))
	if err != nil {
		t.Fatal(err)
	}
	sum := sha256.Sum256(raw)
	return hex.EncodeToString(sum[:])
}

// roomLoadHash lädt den Spielstand direkt über engine/room (ohne local) und liefert den Welt-Hash.
func roomLoadHash(t *testing.T, dir, name string) string {
	t.Helper()
	m := room.NewManager(&store.Saves{Dir: dir, Now: fixedNow})
	m.Now = fixedNow
	m.NewCode = func() string { return "REFR" }
	p := &refPeer{worlds: map[int]*sim.World{}}
	if _, err := m.Create("ref", p, name, false, 0, []int{0, 1}, room.Options{}); err != nil {
		t.Fatal(err)
	}
	return hashWorlds(t, p.worlds)
}

// AC-04 (NG1.2), B-793/AC-02: Speichern, Liste, Laden und Löschen gegen einen Temp-Ordner. Nach Laden ist der
// Welt-Hash gleich dem eines direkt über engine/room geladenen Raums, und Speichern ohne Takt schreibt denselben
// Spielstand (Hash der Datei). Gegen die Welt vor dem Schließen lässt sich nicht vergleichen: Das Spielstand-Format
// rundet Positionen und setzt Abklingzeiten und IDs neu (belegt, unverändert wie im Server).
func TestSaveListLoadDelete(t *testing.T) {
	dir := t.TempDir()
	s := testSession(t, dir)
	if err := s.New(view.NewGame{Devices: twoDevices}); err != nil {
		t.Fatal(err)
	}
	if s.name != "spiel-1" {
		t.Fatalf("freier Name = %q", s.name)
	}
	stepN(t, s, 60)
	if err := s.Save(); err != nil {
		t.Fatal(err)
	}
	if f := s.Step(); !hasCue(f, view.CueSaved) {
		t.Errorf("kein CueSaved nach Save: %+v", f.Cues)
	}
	if err := s.Close(); err != nil || s.Active() {
		t.Fatalf("Close: %v, aktiv %v", err, s.Active())
	}
	want := fileHash(t, dir, "spiel-1")

	list, err := s.List()
	if err != nil || len(list) != 1 || list[0].Name != "spiel-1" || !list[0].Loadable || list[0].SavedAt.IsZero() {
		t.Fatalf("Liste = %+v, %v", list, err)
	}
	if err := s.Load("spiel-1"); err != nil {
		t.Fatal(err)
	}
	if got, ref := hashWorlds(t, s.p.worlds), roomLoadHash(t, dir, "spiel-1"); got != ref {
		t.Errorf("Welt-Hash nach Laden %s, Raum direkt %s", got, ref)
	}
	if err := s.Save(); err != nil {
		t.Fatal(err)
	}
	if got := fileHash(t, dir, "spiel-1"); got != want {
		t.Errorf("Spielstand-Hash nach Laden und Speichern %s, vorher %s", got, want)
	}
	if f := s.Step(); len(f.Seats) != 2 {
		t.Errorf("nach Laden %d Plätze, erwartet die 2 Geräte des letzten Spiels", len(f.Seats))
	}

	if err := s.Delete("spiel-1"); err == nil {
		t.Error("Spielstand des laufenden Spiels gelöscht")
	}
	if err := s.Close(); err != nil {
		t.Fatal(err)
	}
	if err := s.Delete("spiel-1"); err != nil {
		t.Fatal(err)
	}
	if list, _ := s.List(); len(list) != 0 {
		t.Errorf("Liste nach Löschen = %+v", list)
	}
	if err := s.Delete("spiel-1"); !errors.Is(err, store.ErrNotFound) {
		t.Error("Löschen eines fehlenden Stands ohne Fehler")
	}
	if err := s.Load("spiel-1"); err == nil {
		t.Error("gelöschter Stand geladen")
	}
}

// Weiter lädt den zuletzt gespeicherten ladbaren Stand; ein Name gibt es nur einmal.
func TestContinueAndDuplicate(t *testing.T) {
	s := testSession(t, t.TempDir())
	if err := s.Continue(); err == nil {
		t.Fatal("Weiter ohne Spielstand")
	}
	if err := s.New(view.NewGame{Name: "alt", Devices: twoDevices[:1]}); err != nil {
		t.Fatal(err)
	}
	stepN(t, s, 5)
	if err := s.Close(); err != nil {
		t.Fatal(err)
	}
	if err := s.New(view.NewGame{Name: "alt", Devices: twoDevices[:1]}); err == nil {
		t.Error("zweites neues Spiel mit demselben Namen")
	}
	if err := s.New(view.NewGame{Name: "Ungültig!", Devices: twoDevices[:1]}); err == nil {
		t.Error("ungültiger Name angenommen")
	}
	if err := s.Continue(); err != nil || s.name != "alt" {
		t.Errorf("Weiter: %v, Spiel %q", err, s.name)
	}
}

// Kaputter Stand und unbekannte Version stehen als nicht ladbar in der Liste; Laden meldet einen Fehler, kein Absturz.
func TestBrokenSaves(t *testing.T) {
	dir := t.TempDir()
	files := map[string]string{
		"kaputt.json":  "{nicht json",
		"zukunft.json": `{"campaignId":"x","version":99,"savedAt":"2026-01-01T00:00:00.000Z"}`,
	}
	for name, body := range files {
		if err := os.WriteFile(filepath.Join(dir, name), []byte(body), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	s := testSession(t, dir)
	list, err := s.List()
	if err != nil || len(list) != 2 {
		t.Fatalf("Liste = %+v, %v", list, err)
	}
	for _, e := range list {
		if e.Loadable || e.Problem == "" {
			t.Errorf("%s ladbar: %+v", e.Name, e)
		}
		if err := s.Load(e.Name); err == nil || s.Active() {
			t.Errorf("%s geladen", e.Name)
		}
	}
	if err := s.Continue(); err == nil {
		t.Error("Weiter mit nur kaputten Ständen")
	}
}

// Ist der Ordner nicht beschreibbar, meldet Speichern CueSaveFailed und das Spiel läuft weiter.
func TestSaveDirNotWritable(t *testing.T) {
	file := filepath.Join(t.TempDir(), "datei")
	if err := os.WriteFile(file, nil, 0o644); err != nil {
		t.Fatal(err)
	}
	s := testSession(t, filepath.Join(file, "saves")) // Ordner unter einer Datei: MkdirAll scheitert
	if err := s.New(view.NewGame{Name: "fest", Devices: twoDevices[:1]}); err != nil {
		t.Fatal(err)
	}
	stepN(t, s, 3)
	if err := s.Save(); err == nil {
		t.Error("Save ohne Fehler")
	}
	f := stepN(t, s, 1)
	if !hasCue(f, view.CueSaveFailed) || !s.Active() {
		t.Errorf("Cues %+v, aktiv %v", f.Cues, s.Active())
	}
	if f2 := stepN(t, s, 1); f2.Tick != f.Tick+1 {
		t.Errorf("Spiel steht: Takt %d nach %d", f2.Tick, f.Tick)
	}
}
