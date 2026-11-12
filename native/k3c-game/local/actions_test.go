package local

import (
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

var pad7 = view.DeviceID{Kind: view.DeviceGamepad, Index: 7}

// Rebind gibt dem Spieler ein anderes Gerät, meldet CueRebound und merkt es für Weiter; ein Gerät steuert nur einen.
func TestRebind(t *testing.T) {
	s := testSession(t, t.TempDir())
	if err := s.Rebind(0, pad7); err == nil {
		t.Fatal("Rebind ohne Spiel")
	}
	if err := s.New(view.NewGame{Name: "pad", Devices: twoDevices}); err != nil {
		t.Fatal(err)
	}
	if err := s.Rebind(0, twoDevices[1]); err == nil {
		t.Error("Gerät von Spieler 2 an Spieler 1 vergeben")
	}
	if err := s.Rebind(1, pad7); err != nil {
		t.Fatal(err)
	}
	f := s.Step()
	if seat, _ := f.Seat(1); seat.Device != pad7 || !hasCue(f, view.CueRebound) || f.Cues[0].Device != pad7 {
		t.Errorf("Platz %+v, Cues %+v", seat, f.Cues)
	}
	if err := s.Rebind(1, pad7); err != nil || len(s.Step().Cues) != 0 {
		t.Errorf("gleiches Gerät: %v", err)
	}
	if !slices.Equal(s.last, []view.DeviceID{twoDevices[0], pad7}) {
		t.Errorf("Geräte für Weiter %v", s.last)
	}
	if err := s.Rebind(3, pad7); err == nil {
		t.Error("Rebind für fehlenden Spieler")
	}
}

// Learn und Respec gehen an den Raum: unbekannte Skills und fremde Spieler lehnt local ab; ein lernbarer Skill steht
// im nächsten Frame beim Monarchen, Respec am Tag an der Burg nimmt ihn wieder.
func TestLearnRespec(t *testing.T) {
	s := testSession(t, t.TempDir())
	if err := s.Learn(0, "x"); err == nil {
		t.Fatal("Learn ohne Spiel")
	}
	if err := s.New(view.NewGame{Name: "skills", Devices: twoDevices[:1]}); err != nil {
		t.Fatal(err)
	}
	f := s.Step()
	if err := s.Learn(0, "gibt-es-nicht"); err == nil {
		t.Error("unbekannter Skill gelernt")
	}
	if err := s.Respec(1); err == nil {
		t.Error("Respec für fehlenden Spieler")
	}
	// B-803/AC-02: Die Ablehnung der Simulation nennt den Grund.
	if err := s.Learn(0, "fireball"); err == nil || !strings.HasSuffix(err.Error(), `: skill "fireball": keine Punkte frei`) {
		t.Errorf("Learn ohne Punkt: %v", err)
	}
	// Zu Beginn gibt es keine Punkte: ein Punkt über SkillBase (der Takt läuft nur in Step, kein Wettlauf).
	f.Monarch(0).SkillBase = 1
	skills := sim.LearnableSkills(f.World(view.Cell{Player: 0}), f.Monarch(0))
	if len(skills) == 0 {
		t.Fatal("kein Skill lernbar trotz freiem Punkt")
	}
	if err := s.Learn(0, skills[0]); err != nil {
		t.Fatal(err)
	}
	if f = s.Step(); !slices.Contains(f.Monarch(0).Skills, skills[0]) {
		t.Fatalf("Skills nach Learn %v", f.Monarch(0).Skills)
	}
	if err := s.Respec(0); err != nil {
		t.Fatal(err)
	}
	if f = s.Step(); len(f.Monarch(0).Skills) != 0 {
		t.Errorf("Skills nach Respec %v", f.Monarch(0).Skills)
	}
}

// Einstellungen: Standard ohne Datei, gespeichert und wieder gelesen, kaputte Datei = Standard; die Datei steht nicht
// in der Spielstand-Liste.
func TestSettingsPersist(t *testing.T) {
	dir := t.TempDir()
	s := testSession(t, dir)
	if got := s.Step().Settings; got != view.DefaultSettings() {
		t.Fatalf("ohne Datei %+v", got)
	}
	want := view.Settings{Music: 0.3, Effects: 0.6, Fullscreen: true, Language: "en"}
	if err := s.SetSettings(view.Settings{Music: 0.3, Effects: 0.6, Fullscreen: true, Language: "en"}); err != nil {
		t.Fatal(err)
	}
	if got := s.Step().Settings; got != want {
		t.Errorf("nach SetSettings %+v", got)
	}
	if got := testSession(t, dir).Step().Settings; got != want {
		t.Errorf("neu geöffnet %+v", got)
	}
	if list, _ := s.List(); len(list) != 0 {
		t.Errorf("Einstellungen in der Spielstand-Liste: %+v", list)
	}
	if err := os.WriteFile(filepath.Join(dir, SettingsFile), []byte("{kaputt"), 0o644); err != nil {
		t.Fatal(err)
	}
	if got := testSession(t, dir).Step().Settings; got != view.DefaultSettings() {
		t.Errorf("kaputte Datei %+v", got)
	}
	if err := os.WriteFile(filepath.Join(dir, SettingsFile), []byte(`{"music":7,"language":"xx"}`), 0o644); err != nil {
		t.Fatal(err)
	}
	if got := testSession(t, dir).Step().Settings; got != view.DefaultSettings() {
		t.Errorf("Werte außerhalb %+v", got)
	}
}

// Ist der Ordner nicht beschreibbar, gelten die Einstellungen trotzdem und SetSettings meldet den Fehler.
func TestSettingsNotWritable(t *testing.T) {
	file := filepath.Join(t.TempDir(), "datei")
	if err := os.WriteFile(file, nil, 0o644); err != nil {
		t.Fatal(err)
	}
	s := testSession(t, filepath.Join(file, "saves"))
	if err := s.SetSettings(view.Settings{Music: 0.5}); err == nil {
		t.Error("kein Fehler beim Speichern")
	}
	if got := s.Step().Settings; got.Music != 0.5 || got.Language != view.DefaultLanguage {
		t.Errorf("Einstellungen %+v", got)
	}
}
