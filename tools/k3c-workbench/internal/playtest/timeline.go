package playtest

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
)

// Zeitleiste (B-1223): Sätze des Transkripts, Marker aus markers.jsonl und deren Bilder in zeitlicher Folge.
// Zeiten: ms in Unix-Zeit (Ms) und in Videozeit ab Aufnahmestart (VideoMs, für den Sprung im Player).

// Marker ist eine Zeile von markers.jsonl (Schreiber: das Spiel, WZ66; Feldnamen wie rust/…/playtest/marker.rs).
type Marker struct {
	Ts           int64    `json:"ts"`
	Kind         string   `json:"kind"` // pad oder ruckler
	Player       int      `json:"player"`
	World        string   `json:"world"`
	Island       int      `json:"island"`
	Stage        int      `json:"stage"`
	Night        bool     `json:"night"`
	Phase        string   `json:"phase"`
	WorstFrameMs float64  `json:"worst_frame_ms"`
	FrameMs      *float64 `json:"frame_ms,omitempty"`
	Save         string   `json:"save,omitempty"`
}

// Entry ist ein Eintrag der Zeitleiste.
type Entry struct {
	Kind    string  `json:"kind"` // sprache, marker
	Ms      int64   `json:"ms"`
	VideoMs int64   `json:"videoMs"`
	EndMs   int64   `json:"endMs,omitempty"` // nur Sprache
	Text    string  `json:"text,omitempty"`  // Sprache: der Satz; Marker: Kurzbeschreibung
	Marker  *Marker `json:"marker,omitempty"`
	Nr      int     `json:"nr,omitempty"`    // Marker: Zeilennummer ab 1
	Frame   string  `json:"frame,omitempty"` // Marker: Bild relativ zur Session, wenn es existiert
}

// Timeline ist die Zeitleiste einer Session.
type Timeline struct {
	ID            string  `json:"id"`
	CaptureMs     int64   `json:"captureMs"` // Unix-ms bei 0 s Videozeit
	HasTranscript bool    `json:"hasTranscript"`
	Entries       []Entry `json:"entries"`
}

// ReadMarkers liest markers.jsonl; fehlt die Datei, ist die Liste leer, kaputte Zeilen werden übergangen.
func ReadMarkers(dir string) ([]Marker, error) {
	raw, err := os.ReadFile(filepath.Join(dir, fileMarkers))
	if errors.Is(err, os.ErrNotExist) {
		return nil, nil
	}
	if err != nil {
		return nil, err
	}
	var out []Marker
	for _, l := range strings.Split(string(raw), "\n") {
		var m Marker
		if strings.TrimSpace(l) == "" || json.Unmarshal([]byte(l), &m) != nil {
			continue
		}
		out = append(out, m)
	}
	return out, nil
}

// describe ist die Kurzbeschreibung eines Markers.
func describe(m Marker) string {
	who := "Spieler " + itoa(m.Player)
	if m.Player == 0 {
		who = "Spiel"
	}
	what := "Marker"
	if m.Kind == "ruckler" {
		what = "Ruckler"
	}
	s := what + " · " + who
	if m.Island > 0 {
		s += " · Insel " + itoa(m.Island) + " Stufe " + itoa(m.Stage)
	}
	return s
}

func itoa(n int) string { return strconv.Itoa(n) }

// Timeline baut die Zeitleiste; ohne transcript.json zeigt sie nur Marker (und deren Bilder, falls schon da).
func (s *Store) Timeline(id string) (Timeline, error) {
	ses, err := s.Get(id)
	if err != nil {
		return Timeline{}, err
	}
	dir := filepath.Join(s.Root, ses.ID)
	capture := CaptureStart(readMeta(dir))
	tl := Timeline{ID: id, CaptureMs: capture, Entries: []Entry{}}
	var tr Transcript
	if raw, err := os.ReadFile(filepath.Join(dir, fileTranscript)); err == nil && json.Unmarshal(raw, &tr) == nil {
		tl.HasTranscript = true
		for _, seg := range tr.Segments {
			tl.Entries = append(tl.Entries, Entry{Kind: "sprache", Ms: seg.StartMs, EndMs: seg.EndMs, VideoMs: seg.StartMs - capture, Text: seg.Text})
		}
	}
	marks, err := ReadMarkers(dir)
	if err != nil {
		return tl, err
	}
	for i := range marks {
		m := marks[i]
		e := Entry{Kind: "marker", Ms: m.Ts, VideoMs: m.Ts - capture, Text: describe(m), Marker: &m, Nr: i + 1}
		if st, err := os.Stat(filepath.Join(dir, filepath.FromSlash(FrameName(i+1)))); err == nil && !st.IsDir() {
			e.Frame = FrameName(i + 1)
		}
		tl.Entries = append(tl.Entries, e)
	}
	sort.SliceStable(tl.Entries, func(i, j int) bool { return tl.Entries[i].Ms < tl.Entries[j].Ms })
	return tl, nil
}

// FramePath ist der Pfad eines Marker-Bilds (nur frames/marker-<nr>.jpg).
func (s *Store) FramePath(id string, nr int) (string, error) {
	if !idRe.MatchString(id) || nr < 1 {
		return "", ErrBadID
	}
	p := filepath.Join(s.Root, id, filepath.FromSlash(FrameName(nr)))
	if st, err := os.Stat(p); err != nil || st.IsDir() {
		return "", ErrNotFound
	}
	return p, nil
}
