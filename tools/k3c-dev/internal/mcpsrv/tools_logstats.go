package mcpsrv

import (
	"context"
	"fmt"
	"slices"
	"sort"
	"strings"
	"time"

	"k3c/tools/k3c-dev/internal/logs"
)

const (
	defaultContextLines = 10
	// contextWindow ist die Spanne vor ts, die logs_context liest; genug für die Zeilen davor, ohne die ganze Datei.
	contextWindow = 6 * time.Hour
)

type statsIn struct {
	Source  string `json:"service,omitempty" jsonschema:"Dienst mit Logdatei aus logs_services; leer: alle"`
	Since   string `json:"since,omitempty" jsonschema:"Dauer wie 30m, 24h oder Zeitpunkt RFC 3339; Standard 1h"`
	GroupBy string `json:"groupBy,omitempty" jsonschema:"level (Standard) oder ns"`
}

type contextIn struct {
	Source string `json:"service" jsonschema:"Dienst mit Logdatei aus logs_services, z. B. k3c-server"`
	TS     string `json:"ts" jsonschema:"Zeitpunkt RFC 3339, z. B. aus logs_errors oder logs_query"`
	Before int    `json:"before,omitempty" jsonschema:"Zeilen davor, Standard 10"`
	After  int    `json:"after,omitempty" jsonschema:"Zeilen danach, Standard 10"`
	Limit  int    `json:"limit,omitempty" jsonschema:"höchstens so viele Zeilen insgesamt, Standard 100, höchstens 500"`
}

// logsStats ist das Tool logs_stats: Zahl der Einträge je Level oder Namensraum.
func (s *Server) logsStats(ctx context.Context, in statsIn) (string, error) {
	if in.GroupBy == "" {
		in.GroupBy = "level"
	}
	if in.GroupBy != "level" && in.GroupBy != "ns" {
		return "", fmt.Errorf("groupBy %q ungültig: erlaubt level oder ns", in.GroupBy)
	}
	if in.Since == "" {
		in.Since = "1h"
	}
	since, err := parseSince(in.Since, time.Now())
	if err != nil {
		return "", err
	}
	paths, err := s.logPaths(s.ws(ctx).root, in.Source)
	if err != nil {
		return "", err
	}
	res, err := scanAll(paths, logs.Query{Since: since, Limit: errorsScanLimit})
	if err != nil {
		return "", err
	}
	counts := map[string]int{}
	for _, e := range res.Entries {
		key := e.Level
		if in.GroupBy == "ns" {
			key = e.NS
		}
		counts[key]++
	}
	keys := make([]string, 0, len(counts))
	for k := range counts {
		keys = append(keys, k)
	}
	sort.Slice(keys, func(i, j int) bool { return counts[keys[i]] > counts[keys[j]] || counts[keys[i]] == counts[keys[j]] && keys[i] < keys[j] })
	parts := make([]string, len(keys))
	for i, k := range keys {
		parts[i] = fmt.Sprintf("%s %d", orDash(k), counts[k])
	}
	head := fmt.Sprintf("%d Einträge seit %s je %s", len(res.Entries), in.Since, in.GroupBy)
	return scanFooter(head, res) + "\n" + orNone(parts), nil
}

// logsContext ist das Tool logs_context: Zeilen um einen Zeitpunkt, älteste zuerst; ">" markiert die erste Zeile ab ts.
func (s *Server) logsContext(ctx context.Context, in contextIn) (string, error) {
	ts, err := time.Parse(time.RFC3339Nano, in.TS)
	if err != nil {
		return "", fmt.Errorf("ts %q ist kein Zeitpunkt nach RFC 3339", in.TS)
	}
	path, err := s.logPath(s.ws(ctx).root, in.Source)
	if err != nil {
		return "", err
	}
	res, err := logs.Scan(path, logs.Query{Since: ts.Add(-contextWindow)})
	if err != nil {
		return "", err
	}
	entries := slices.Clone(res.Entries)
	slices.Reverse(entries) // älteste zuerst
	at, _ := slices.BinarySearchFunc(entries, ts, func(e logs.Entry, t time.Time) int { return e.Time.Compare(t) })
	before, after := orDefault(in.Before, defaultContextLines), orDefault(in.After, defaultContextLines)
	from, to := max(0, at-before), min(len(entries), at+after+1)
	if to-from > clampLimit(in.Limit) {
		to = from + clampLimit(in.Limit)
	}
	out := make([]string, 0, to-from+1)
	for i := from; i < to; i++ {
		mark := "  "
		if i == at {
			mark = "> "
		}
		out = append(out, mark+entryLine(entries[i]))
	}
	return strings.Join(append(out, fmt.Sprintf("%d Zeilen um %s", to-from, in.TS)), "\n"), nil
}

func orDefault(v, def int) int {
	if v <= 0 {
		return def
	}
	return v
}

func orDash(s string) string {
	if s == "" {
		return "–"
	}
	return s
}
