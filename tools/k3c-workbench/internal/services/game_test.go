package services

import (
	"context"
	"log/slog"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"time"

	"k3c/tools/workkit/console"
)

// gameSvc ist ein Dienst wie das Spiel (B-1214): ohne Port, gesund, solange der Prozess läuft, nur von Hand gestartet.
func gameSvc(name string) Service {
	return Service{Name: name, Command: []string{"x"}, Health: "process", Manual: true,
		StopBefore: []string{"build", "rust:game:build"}}
}

// B-1214 AC-01: services.json führt die Spiel-Dienste game-dev (Task) und game-release (Paket-EXE).
func TestServicesJSONKenntDieSpielDienste(t *testing.T) {
	list, err := Load("../../services.json")
	if err != nil {
		t.Fatal(err)
	}
	byName := map[string]Service{}
	for _, s := range list {
		byName[s.Name] = s
	}
	dev, rel := byName["game-dev"], byName["game-release"]
	if strings.Join(dev.Command, " ") != "task rust:game:dev" || dev.Health != "process" || dev.Port != 0 || !dev.Manual {
		t.Errorf("game-dev: %+v", dev)
	}
	if strings.Join(rel.Command, " ") != "dist/k3c-game/k3c-game.exe" || rel.Health != "process" || !rel.Manual ||
		rel.Needs != "dist/k3c-game/k3c-game.exe" || !strings.Contains(rel.NeedsHint, "task build") ||
		!slices.Contains(rel.StopBefore, "build") || !slices.Contains(rel.StopBefore, "rust:game:build") {
		t.Errorf("game-release: %+v", rel)
	}
}

// Prozess-Dienste haben keinen Port; zwei davon sind kein „port doppelt“, ein Port bei health process ist ein Fehler.
func TestLoadProzessDienste(t *testing.T) {
	two := `[{"name":"A","command":["x"],"health":"process"},{"name":"B","command":["x"],"health":"process"}]`
	if _, err := Load(writeConfig(t, two)); err != nil {
		t.Errorf("zwei Prozess-Dienste: %v", err)
	}
	cases := map[string]string{
		`[{"name":"A","command":["x"],"port":5,"health":"process"}]`:             "ohne port",
		`[{"name":"A","command":["x"],"health":"http"}]`:                         "port 0 ungültig",
		`[{"name":"A","command":["x"],"health":"process","needs":"../x"}]`:       `needs "../x"`,
		`[{"name":"A","command":["x"],"health":"process","stopBefore":["a b"]}]`: `stopBefore "a b"`,
	}
	for content, want := range cases {
		if _, err := Load(writeConfig(t, content)); err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("%s: %v, erwartet %q", content, err, want)
		}
	}
	if got := Shift([]Service{gameSvc("g")}, 10); got[0].Port != 0 || got[0].HealthURL() != "" {
		t.Errorf("Shift verschiebt Port 0: %+v", got[0])
	}
}

// AC-01/AC-03: Start ohne Port-Prüfung, Stopp beendet den Prozess (Job-Objekt in Produktion), ein Ende ohne Fehler
// (Spiel geschlossen) ist „gestoppt“, kein Ausfall; StartAll und Adopt lassen das Spiel in Ruhe.
func TestProzessDienstStartStopUndEnde(t *testing.T) {
	f := &fake{} // die Prüfung schlüge fehl: ein Prozess-Dienst braucht sie nicht
	f.busy.Store(true)
	c := testController(f, console.New(0, nil), gameSvc("game-dev"))
	c.Adopt(context.Background())
	if st := c.Statuses()[0]; st.State != Stopped {
		t.Fatalf("Adopt übernimmt das Spiel: %+v", st)
	}
	if err := c.StartAll(context.Background()); err != nil || f.count() != 0 {
		t.Fatalf("StartAll startet das Spiel: %v, %d", err, f.count())
	}
	st, err := c.Start(context.Background(), "game-dev")
	if err != nil || st.State != Running {
		t.Fatalf("Start: %+v, %v", st, err)
	}
	st, _ = c.Stop(context.Background(), "game-dev", false)
	if st.State != Stopped || !f.last().killed.Load() {
		t.Errorf("Stop: %+v", st)
	}
	if _, err := c.Start(context.Background(), "game-dev"); err != nil {
		t.Fatal(err)
	}
	f.last().exit()
	waitFor(t, "gestoppt nach Ende", func() bool { s := c.Statuses()[0]; return s.State == Stopped && s.PID == 0 })
	if st := c.Statuses()[0]; st.LastError != "" {
		t.Errorf("Ende ohne Fehler als Ausfall: %+v", st)
	}
}

// Fehlt die EXE (needs), meldet der Start den Hinweis statt eines Prozessfehlers.
func TestProzessDienstOhneExe(t *testing.T) {
	root := t.TempDir()
	f := &fake{}
	s := gameSvc("game-release")
	s.Needs, s.NeedsHint = "dist/k3c-game/k3c-game.exe", "erst task build"
	c := New([]Service{s}, Options{Root: root, Start: f.start, Check: f.check, Listen: f.listen, Sample: f.sample,
		KillPID: f.killPID, StartPoll: 2 * time.Millisecond})
	st, err := c.Start(context.Background(), "game-release")
	if err == nil || st.State != Failed || !strings.Contains(st.LastError, "erst task build") || f.count() != 0 {
		t.Fatalf("ohne EXE: %+v, %v", st, err)
	}
	exe := filepath.Join(root, "dist", "k3c-game", "k3c-game.exe")
	_ = os.MkdirAll(filepath.Dir(exe), 0o755)
	_ = os.WriteFile(exe, []byte("x"), 0o644)
	if st, err := c.Start(context.Background(), "game-release"); err != nil || st.State != Running {
		t.Fatalf("mit EXE: %+v, %v", st, err)
	}
	_, _ = c.Stop(context.Background(), "game-release", false)
}

// AC-02: Die Ausgabe des Spiels landet in logs/<dienst>.jsonl (Quelle je Dienst), die Emojis unverändert.
func TestProzessDienstAusgabeImLog(t *testing.T) {
	root := t.TempDir()
	f := &fake{}
	c := New([]Service{gameSvc("game-dev")}, Options{Root: root, Console: console.New(0, nil), Log: slog.New(slog.DiscardHandler),
		Start: f.start, Check: f.check, Listen: f.listen, Sample: f.sample, KillPID: f.killPID, StartPoll: 2 * time.Millisecond})
	if _, err := c.Start(context.Background(), "game-dev"); err != nil {
		t.Fatal(err)
	}
	_, _ = c.Stop(context.Background(), "game-dev", false)
	waitFor(t, "Ausgabe-Datei", func() bool { return len(readOut(t, root, "game-dev")) == 1 })
	if es := readOut(t, root, "game-dev"); es[0].Msg != "hallo von game-dev" || es[0].Data["stream"] != "stdout" {
		t.Errorf("Log: %+v", es)
	}
}

// AC-03: Vor einem Neubau-Task beendet die Workbench den laufenden Release-Dienst (Hinweis im Log statt Kopierfehler).
func TestStopForTask(t *testing.T) {
	f := &fake{}
	c := testController(f, nil, gameSvc("game-release"), svc("Vite", false))
	if got := c.StopForTask(context.Background(), "build"); len(got) != 0 {
		t.Errorf("gestoppter Dienst gemeldet: %v", got)
	}
	if _, err := c.Start(context.Background(), "game-release"); err != nil {
		t.Fatal(err)
	}
	if got := c.StopForTask(context.Background(), "test"); len(got) != 0 {
		t.Errorf("fremder Task stoppt: %v", got)
	}
	if got := c.StopForTask(context.Background(), "rust:game:build"); len(got) != 1 || got[0] != "game-release" {
		t.Fatalf("Neubau: %v", got)
	}
	if st := c.Statuses()[0]; st.State != Stopped || !f.last().killed.Load() {
		t.Errorf("nach Neubau-Stopp: %+v", st)
	}
}

// Ein Programm mit Pfad liegt relativ zur Repo-Wurzel, ein Name ohne Pfad (task) kommt aus PATH.
func TestProgramRelativZurWurzel(t *testing.T) {
	root := filepath.Join("C:", "repo")
	if got := program("dist/k3c-game/k3c-game.exe", root); got != filepath.Join(root, "dist", "k3c-game", "k3c-game.exe") {
		t.Errorf("Pfad: %s", got)
	}
	if got := program("task", root); got != "task" {
		t.Errorf("Name: %s", got)
	}
}
