// Package logs liest die JSON-Logs unter logs/*.jsonl (B-046): eine Zeile je Eintrag im Format von log/slog mit
// time, level, msg, optional ns und weiteren Feldern als Daten. Gelesen wird von hinten und unter einem Byte-Budget,
// damit auch große Dateien schnell antworten.
package logs

import (
	"encoding/json"
	"strings"
	"time"
)

// Entry ist ein Log-Eintrag; Data hält alle Felder außer time, level, msg und ns.
type Entry struct {
	Time  time.Time      `json:"time"`
	Level string         `json:"level"`
	NS    string         `json:"ns"`
	Msg   string         `json:"msg"`
	Data  map[string]any `json:"data,omitempty"`
}

var levels = []string{"DEBUG", "INFO", "WARN", "ERROR"}

// Levels sind die bekannten Level in aufsteigender Reihenfolge.
func Levels() []string { return append([]string(nil), levels...) }

// LevelRank ordnet ein Level ein; ein unbekanntes zählt als niedrigstes, damit ein Filter es nie durchlässt.
func LevelRank(level string) int {
	for i, l := range levels {
		if strings.EqualFold(l, level) {
			return i
		}
	}
	return -1
}

// parse liest eine Zeile; false heißt: kein gültiges JSON oder keine Zeit.
func parse(line []byte) (Entry, bool) {
	var raw map[string]any
	if err := json.Unmarshal(line, &raw); err != nil {
		return Entry{}, false
	}
	stamp, _ := raw["time"].(string)
	t, err := time.Parse(time.RFC3339Nano, stamp)
	if err != nil {
		return Entry{}, false
	}
	e := Entry{Time: t}
	e.Level, _ = raw["level"].(string)
	e.Msg, _ = raw["msg"].(string)
	e.NS, _ = raw["ns"].(string)
	e.Level = strings.ToUpper(e.Level)
	for _, k := range []string{"time", "level", "msg", "ns"} {
		delete(raw, k)
	}
	if len(raw) > 0 {
		e.Data = raw
	}
	return e, true
}
