// Package trace schreibt Aufzeichnungen eines Sim-Laufs als gzip-komprimierte JSON-Zeilen (.jsonl.gz) für den Rust-Port
// (Projekt RSU, RU38): Zeile 1 ist das Level der Szene, danach je Takt eine Zeile mit Zustand und Ereignissen.
//
//	{"art":"level","szene":"nacht","seed":"…","spieler":2,"level":{…}}
//	{"art":"takt","takt":1,"zustand":{…},"ereignisse":[…]}
//
// Zahlen schreibt encoding/json; das Paket ist reine Logik ohne Abhängigkeit vom Raum (die Szenen stehen im Test).
package trace

import (
	"bytes"
	"compress/gzip"
	"encoding/json"
	"fmt"
	"io"
)

// Kopf ist die erste Zeile einer Aufzeichnung.
type Kopf struct {
	Art     string `json:"art"` // immer "level"
	Szene   string `json:"szene"`
	Seed    string `json:"seed"`
	Spieler int    `json:"spieler"`
	Level   any    `json:"level"`
}

// Takt ist die Zeile eines Takts.
type Takt struct {
	Art        string `json:"art"` // immer "takt"
	Takt       int    `json:"takt"`
	Zustand    any    `json:"zustand"`
	Ereignisse any    `json:"ereignisse"`
}

// Schreiber sammelt die Zeilen einer Aufzeichnung; Lines liefert sie unkomprimiert, Gzip komprimiert sie.
type Schreiber struct {
	lines [][]byte
}

// Level schreibt die Kopfzeile; sie muss die erste Zeile sein.
func (s *Schreiber) Level(szene, seed string, spieler int, level any) error {
	if len(s.lines) != 0 {
		return fmt.Errorf("trace: Kopfzeile muss die erste Zeile sein")
	}
	return s.add(Kopf{Art: "level", Szene: szene, Seed: seed, Spieler: spieler, Level: level})
}

// Takt schreibt die Zeile eines Takts; zustand und ereignisse werden sofort serialisiert (die Welt ändert sich weiter).
func (s *Schreiber) Takt(takt int, zustand, ereignisse any) error {
	if len(s.lines) == 0 {
		return fmt.Errorf("trace: Takt vor der Kopfzeile")
	}
	return s.add(Takt{Art: "takt", Takt: takt, Zustand: zustand, Ereignisse: ereignisse})
}

func (s *Schreiber) add(v any) error {
	b, err := json.Marshal(v)
	if err != nil {
		return err
	}
	s.lines = append(s.lines, b)
	return nil
}

// Lines sind die Zeilen ohne Zeilenende.
func (s *Schreiber) Lines() [][]byte { return s.lines }

// Gzip liefert die ersten n Zeilen-Takte (n < 0: alle) als .jsonl.gz: die Kopfzeile plus n Taktzeilen.
func (s *Schreiber) Gzip(n int) ([]byte, error) {
	lines := s.lines
	if n >= 0 && n+1 < len(lines) {
		lines = lines[:n+1]
	}
	var buf bytes.Buffer
	zw, err := gzip.NewWriterLevel(&buf, gzip.BestCompression)
	if err != nil {
		return nil, err
	}
	for _, l := range lines {
		if _, err := zw.Write(append(append([]byte{}, l...), '\n')); err != nil {
			return nil, err
		}
	}
	if err := zw.Close(); err != nil {
		return nil, err
	}
	return buf.Bytes(), nil
}

// Entpacken liest eine .jsonl.gz und liefert die Zeilen ohne Zeilenende.
func Entpacken(raw []byte) ([][]byte, error) {
	zr, err := gzip.NewReader(bytes.NewReader(raw))
	if err != nil {
		return nil, err
	}
	text, err := io.ReadAll(zr)
	if err != nil {
		return nil, err
	}
	text = bytes.TrimSuffix(text, []byte("\n"))
	return bytes.Split(text, []byte("\n")), nil
}
