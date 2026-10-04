package balance

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"

	"k3c/data"
	"k3c/engine/sim"
	"k3c/tools/k3c-dev/internal/enginetools"
)

// Replay-Format (B-159, BAL1.2): Ein Lauf als JSON-Datei aus Szenario, Datenstand-Hash und den Eingaben je Tick.
// Die Wiedergabe braucht keinen Bot; sie rechnet dieselben Befehle nach und liefert denselben Endzustand-Hash.
// Eigenes Format mit eigener Version, unabhängig von den Spielstand-Versionen (B-137).

// maxReplayTicks begrenzt Ticks aus einer Datei: Play läuft im MCP-Server, ein riesiger Wert würde ihn blockieren.
const maxReplayTicks = 1_000_000

// ReplayVersion ist die einzige Version, die Read kennt.
const ReplayVersion = 1

// Replay ist eine Datei. Inputs hat je Spieler (Index der Insel) die Befehle als Lauflängen: ein Span gilt N Ticks.
type Replay struct {
	Version int `json:"version"`
	Scenario
	DataHash       string   `json:"dataHash"` // Hash von data/ bei der Aufnahme
	Ticks          int      `json:"ticks"`
	EndHash        string   `json:"endHash"`
	CastleFallTick *int     `json:"castleFallTick"`
	Inputs         [][]Span `json:"inputs"`
}

// Span ist ein Befehl, der N Ticks hintereinander gilt.
type Span struct {
	N int `json:"n"`
	sim.PlayerCommand
}

// Playback ist das Ergebnis einer Wiedergabe; Warnings nennt z. B. einen abweichenden Datenstand.
type Playback struct {
	Ticks          int      `json:"ticks"`
	EndHash        string   `json:"endHash"`
	CastleFallTick *int     `json:"castleFallTick"`
	Warnings       []string `json:"warnings"`
}

// dataHash ist der Hash der eingebetteten Daten (data/embed.go), nach Pfad sortiert (fs.WalkDir) und mit
// LF-Zeilenenden, damit er auf jedem Rechner gleich ist.
var dataHash = hashData()

// DataHash liefert den Hash des eingebetteten Datenstands.
func DataHash() string { return dataHash }

func hashData() string {
	h := sha256.New()
	err := fs.WalkDir(data.Files, ".", func(path string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return err
		}
		raw, err := data.Files.ReadFile(path)
		_, _ = fmt.Fprintf(h,"%s\x00%s\x00", path, bytes.ReplaceAll(raw, []byte("\r\n"), []byte("\n")))
		return err
	})
	if err != nil {
		panic(fmt.Sprintf("balance: data/: %v", err))
	}
	return hex.EncodeToString(h.Sum(nil))
}

// endHash ist der Hash des Inselzustands als JSON (Structs; Maps sortiert encoding/json nach Schlüssel).
func endHash(isl *sim.Island) string {
	b, err := json.Marshal(isl)
	if err != nil {
		panic(fmt.Sprintf("balance: Endzustand: %v", err))
	}
	sum := sha256.Sum256(b)
	return hex.EncodeToString(sum[:])
}

// record hängt die Befehle eines Ticks an die Lauflängen an.
func (r *Replay) record(cmds []sim.PlayerCommand) {
	if r.Inputs == nil {
		r.Inputs = make([][]Span, len(cmds))
	}
	for i, c := range cmds {
		spans := r.Inputs[i]
		if n := len(spans); n > 0 && spans[n-1].PlayerCommand == c {
			spans[n-1].N++
			continue
		}
		r.Inputs[i] = append(spans, Span{N: 1, PlayerCommand: c})
	}
	r.Ticks++
}

// JSON schreibt die Datei eingerückt mit Zeilenende.
func (r Replay) JSON() ([]byte, error) {
	b, err := json.MarshalIndent(r, "", "  ")
	return append(b, '\n'), err
}

// ReadReplay liest eine Datei: kaputtes JSON → Fehler mit Zeilennummer, unbekannte Version → Fehler mit Version,
// Lauflängen, die nicht zu Spielern und Ticks passen → Fehler.
func ReadReplay(b []byte) (Replay, error) {
	var head struct {
		Version int `json:"version"`
	}
	if err := json.Unmarshal(b, &head); err != nil {
		return Replay{}, lineError(b, err)
	}
	if head.Version != ReplayVersion {
		return Replay{}, fmt.Errorf("replay: unbekannte Version %d (bekannt: %d)", head.Version, ReplayVersion)
	}
	var r Replay
	if err := json.Unmarshal(b, &r); err != nil {
		return Replay{}, lineError(b, err)
	}
	if r.Ticks < 0 || r.Ticks > maxReplayTicks {
		return Replay{}, fmt.Errorf("replay: %d Ticks, höchstens %d erlaubt", r.Ticks, maxReplayTicks)
	}
	if len(r.Inputs) != r.Players {
		return Replay{}, fmt.Errorf("replay: %d Eingabe-Listen für %d Spieler", len(r.Inputs), r.Players)
	}
	for i, spans := range r.Inputs {
		sum := 0
		for _, s := range spans {
			if s.N < 1 {
				return Replay{}, fmt.Errorf("replay: Spieler %d hat einen Span mit n = %d", i, s.N)
			}
			sum += s.N
		}
		if sum != r.Ticks {
			return Replay{}, fmt.Errorf("replay: Spieler %d hat %d Ticks Eingaben, erwartet %d", i, sum, r.Ticks)
		}
	}
	return r, nil
}

// lineError ergänzt einen JSON-Fehler um die Zeile, in der er liegt.
func lineError(b []byte, err error) error {
	var offset int64
	var syn *json.SyntaxError
	var typ *json.UnmarshalTypeError
	switch {
	case errors.As(err, &syn):
		offset = syn.Offset
	case errors.As(err, &typ):
		offset = typ.Offset
	default:
		return fmt.Errorf("replay: %w", err)
	}
	line := bytes.Count(b[:min(int(offset), len(b))], []byte("\n")) + 1
	return fmt.Errorf("replay: Zeile %d: %w", line, err)
}

// Play rechnet die Datei ohne Bot nach. Ein anderer Datenstand ist nur eine Warnung, die Wiedergabe läuft trotzdem.
func Play(r Replay) (Playback, error) {
	var pb Playback
	if r.DataHash != dataHash {
		pb.Warnings = append(pb.Warnings, fmt.Sprintf("Datenstand weicht ab: Datei %s, aktuell %s", r.DataHash, dataHash))
	}
	isl, err := newIsland(r.Scenario)
	if err != nil {
		return pb, err
	}
	c := newCollector(isl, 0)
	cmds := make([]sim.PlayerCommand, r.Players)
	pos, left := make([]int, r.Players), make([]int, r.Players)
	for tick := 1; tick <= r.Ticks; tick++ {
		for i, spans := range r.Inputs {
			if left[i] == 0 {
				cmds[i], left[i] = spans[pos[i]].PlayerCommand, spans[pos[i]].N
				pos[i]++
			}
			left[i]--
		}
		sim.StepIsland(isl, cmds, 1.0/enginetools.TickHz)
		c.observe(tick)
	}
	pb.Ticks, pb.EndHash, pb.CastleFallTick = r.Ticks, endHash(isl), c.result().CastleFallTick
	return pb, nil
}
