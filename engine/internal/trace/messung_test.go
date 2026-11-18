package trace

import (
	"os"
	"path/filepath"
	"testing"
	"time"

	"k3c/engine/room"
	"k3c/engine/sim"
)

// Szene messung-nacht (RU39.1): dieselbe Szene wie `nacht` im Messskript des Spiels (native/measure.ps1:
// --seed k3c --players 2 --tick 15600 --input 0@0=-,1@0=-). Aufbau wie das Spiel bei --seed (native/k3c-game/local/
// session.go: Raum mit Seed, ohne Dev-Mode und ohne Schwierigkeit), 15 600 Takte Vorlauf ohne Aufzeichnung, danach
// 9 060 Takte (60 Aufwärmen der Wiedergabe + 9 000 = 300 s) mit beiden Spielern stehend. Nur mit -write; ein Sim-Lauf
// als Test mit der Ausnahme aus B-1014 (RU38.1, RU39.1).
const (
	messungName  = "messung-nacht"
	messungVor   = 15600
	messungTakte = 9060
	messungPath  = "../../../.work/trace/messung-nacht.jsonl.gz"
)

// tail zeichnet erst ab dem Zustand Nummer from auf (0 = Startzustand) und zählt die Takte ab dort neu.
type tail struct {
	*recorder
	n, from int
}

func (t *tail) State(_, stage int, state any) {
	if t.n++; t.n <= t.from {
		return
	}
	t.recorder.State(t.n-1-t.from, stage, state)
}

// recordMessung spielt die Szene messung-nacht und liefert die Aufzeichnung (Kopf plus messungTakte+1 Zustände).
func recordMessung(t *testing.T) *Schreiber {
	t.Helper()
	m := room.NewManager(memStore{})
	m.Now = func() time.Time { return time.Unix(1_700_000_000, 0) }
	m.NewCode = func() string { return "TRCE" }
	rec := &tail{recorder: &recorder{w: &Schreiber{}, scene: messungName}, from: messungVor}
	r, err := m.Create("trace", rec, "trace-"+messungName, true, 0, []int{0, 1}, room.Options{Seed: "k3c"})
	if err != nil {
		t.Fatal(err)
	}
	stand := map[int]sim.PlayerCommand{0: {}, 1: {}}
	for range messungVor + messungTakte {
		if err := r.Input("trace", rec, stand); err != nil {
			t.Fatal(err)
		}
		r.Tick()
	}
	if rec.err != nil {
		t.Fatal(rec.err)
	}
	return rec.w
}

// TestMessungNacht schreibt mit -write die Aufzeichnung der Mess-Szene nach .work/trace/ (nicht eingecheckt).
func TestMessungNacht(t *testing.T) {
	if !*write {
		t.Skip("nur mit -write (lang: 24 660 Takte)")
	}
	rec := recordMessung(t)
	if got := len(rec.Lines()); got != messungTakte+2 {
		t.Fatalf("%d Zeilen, erwartet %d (Kopf plus Startzustand plus Takte)", got, messungTakte+2)
	}
	if err := os.MkdirAll(filepath.Dir(messungPath), 0o755); err != nil {
		t.Fatal(err)
	}
	put(t, messungPath, rec, -1)
}
