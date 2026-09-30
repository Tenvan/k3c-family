package usage

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

func TestNeustartBehaeltGesamtzeitUndZeitreihe(t *testing.T) {
	path := filepath.Join(t.TempDir(), "k3c", "mcp-usage.json")
	c := &clock{t: time.Date(2026, 9, 30, 12, 0, 0, 0, time.UTC)}
	tr := Open(path, c.now, nil)
	repeat(tr, c.event("q", `{"a":1}`, 100), 20)
	tr.Record(c.event("q", `{"a":1}`, 1500))
	if err := tr.Flush(); err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(path + ".tmp"); !errors.Is(err, os.ErrNotExist) {
		t.Errorf("temporäre Datei liegt noch da: %v", err)
	}
	c.t = c.t.Add(time.Hour)
	again := Open(path, c.now, func(err error) { t.Errorf("unerwartete Warnung: %v", err) })
	snap := again.Snapshot()
	if snap.Session.Calls != 0 || snap.AllTime.Calls != 21 || snap.AllTime.Outliers != 1 || len(snap.Minutes) != 1 {
		t.Errorf("nach Neustart: Sitzung %d, Gesamt %d, Ausreißer %d, Minuten %d",
			snap.Session.Calls, snap.AllTime.Calls, snap.AllTime.Outliers, len(snap.Minutes))
	}
	if !strings.HasPrefix(snap.AllTime.Since, "2026-09-30T12:00") || !strings.HasPrefix(snap.Session.Since, "2026-09-30T13:00") {
		t.Errorf("seit: Gesamt %s, Sitzung %s", snap.AllTime.Since, snap.Session.Since)
	}
	again.Record(c.event("q", `{"a":1}`, 100)) // Vergleichsgruppe aus der Datei
	if u := again.Snapshot().AllTime.Tools[0]; u.Calls != 22 || u.Args[0].Count != 22 {
		t.Errorf("weiterzählen: %+v", u)
	}
}

func TestKaputteOderFremdeDateiWirdBak(t *testing.T) {
	for name, content := range map[string]string{"kaputt": "{nicht", "fremd": `{"version":99,"allTime":{"tools":{}}}`} {
		path := filepath.Join(t.TempDir(), "mcp-usage.json")
		if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
			t.Fatal(err)
		}
		var warned []error
		tr := Open(path, time.Now, func(err error) { warned = append(warned, err) })
		if len(warned) != 1 || !strings.Contains(warned[0].Error(), ".bak") || tr.Snapshot().AllTime.Calls != 0 {
			t.Errorf("%s: Warnungen %v", name, warned)
		}
		if bak, err := os.ReadFile(path + ".bak"); err != nil || string(bak) != content {
			t.Errorf("%s: .bak fehlt oder verändert: %v", name, err)
		}
	}
}

func TestNichtSchreibbarWarntEinmal(t *testing.T) {
	blocker := filepath.Join(t.TempDir(), "datei")
	if err := os.WriteFile(blocker, nil, 0o644); err != nil {
		t.Fatal(err)
	}
	var warned int
	tr := Open(filepath.Join(blocker, "mcp-usage.json"), time.Now, func(error) { warned++ })
	tr.Record(Event{At: time.Now(), Tool: "q", DurationMs: 5, OK: true})
	for range 3 {
		if err := tr.Flush(); err == nil {
			t.Fatal("Schreiben in eine Datei als Ordner ohne Fehler")
		}
	}
	if warned != 1 || tr.Snapshot().Session.Calls != 1 {
		t.Errorf("%d Warnungen, Statistik %d", warned, tr.Snapshot().Session.Calls)
	}
}

func TestRecordPlantSpeichernGebuendelt(t *testing.T) {
	path := filepath.Join(t.TempDir(), "mcp-usage.json")
	tr := Open(path, time.Now, nil)
	tr.Record(Event{At: time.Now(), Tool: "q", DurationMs: 5, OK: true})
	tr.Record(Event{At: time.Now(), Tool: "q", DurationMs: 5, OK: true})
	if _, err := os.Stat(path); !errors.Is(err, os.ErrNotExist) {
		t.Error("sofort geschrieben statt gebündelt")
	}
	deadline := time.Now().Add(5 * time.Second)
	for time.Now().Before(deadline) {
		if _, err := os.Stat(path); err == nil {
			return
		}
		time.Sleep(50 * time.Millisecond)
	}
	t.Error("nach saveDelay nicht geschrieben")
}
