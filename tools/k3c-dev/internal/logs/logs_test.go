package logs

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"
	"time"
)

// sample ist testdata/k3c-dev.jsonl: sieben gültige Einträge (09:00 bis 09:06) und eine kaputte Zeile.
const sample = "testdata/k3c-dev.jsonl"

func msgs(entries []Entry) string {
	out := make([]string, len(entries))
	for i, e := range entries {
		out[i] = e.Msg
	}
	return strings.Join(out, "|")
}

func TestScanRueckwaertsUeberBlockgrenzen(t *testing.T) {
	for _, size := range []int64{7, 64, 1 << 20} {
		blockSize = size
		res, err := Scan(sample, Query{})
		if err != nil || len(res.Entries) != 7 || res.Skipped != 1 || res.BudgetHit {
			t.Fatalf("Block %d: %d Einträge, %d übersprungen, %v", size, len(res.Entries), res.Skipped, err)
		}
		if first := res.Entries[0]; first.Msg != "server gestoppt" || first.Time.Minute() != 6 {
			t.Errorf("Block %d: neuester zuerst erwartet, bekam %+v", size, first)
		}
	}
	blockSize = 1 << 20
}

func TestScanFilter(t *testing.T) {
	cases := []struct {
		q    Query
		want string
	}{
		{Query{MinLevel: "WARN"}, "server gestoppt|check_run: Ziel \"npm:x\" unbekannt|check_run: Ziel \"go:y\" unbekannt"},
		{Query{NS: "check"}, "lauf beendet"},
		{Query{Pattern: regexp.MustCompile(`^check_run`)}, "check_run: Ziel \"npm:x\" unbekannt|check_run: Ziel \"go:y\" unbekannt"},
		{Query{Limit: 2}, "server gestoppt|check_run: Ziel \"npm:x\" unbekannt"},
		{Query{Since: time.Date(2026, 9, 30, 9, 5, 0, 0, time.UTC)}, "server gestoppt|check_run: Ziel \"npm:x\" unbekannt"},
	}
	for _, c := range cases {
		res, err := Scan(sample, c.q)
		if got := msgs(res.Entries); err != nil || got != c.want {
			t.Errorf("%+v: %q, %v", c.q, got, err)
		}
	}
}

func TestScanBudgetUndFehlendeDatei(t *testing.T) {
	blockSize = 64
	defer func() { blockSize = 1 << 20 }()
	res, _ := Scan(sample, Query{Budget: 64})
	if !res.BudgetHit || res.BytesRead != 64 || len(res.Entries) == 7 {
		t.Errorf("Budget: %+v", res)
	}
	res, err := Scan(filepath.Join(t.TempDir(), "fehlt.jsonl"), Query{})
	if err != nil || len(res.Entries) != 0 || res.Entries == nil {
		t.Errorf("fehlende Datei: %+v, %v", res, err)
	}
}

func TestReadSinceMitCursor(t *testing.T) {
	path := filepath.Join(t.TempDir(), "a.jsonl")
	line := func(i int) string {
		return fmt.Sprintf(`{"time":"2026-09-30T10:00:%02dZ","level":"INFO","msg":"m%d"}`+"\n", i, i)
	}
	write := func(text string, flag int) {
		f, err := os.OpenFile(path, os.O_CREATE|os.O_WRONLY|flag, 0o644)
		if err != nil {
			t.Fatal(err)
		}
		_, _ = f.WriteString(text)
		_ = f.Close()
	}
	write(line(1)+line(2)+`{"time":"2026-09-30T10:00:03Z","lev`, os.O_TRUNC)
	res, _ := ReadSince(path, 0, 0, 0)
	if msgs(res.Entries) != "m1|m2" || res.Truncated {
		t.Fatalf("erster Lauf: %+v", res)
	}
	write(`el":"INFO","msg":"m3"}`+"\n"+line(4), os.O_APPEND)
	res, _ = ReadSince(path, res.Cursor, 1, 0)
	if msgs(res.Entries) != "m4" || !res.Truncated {
		t.Errorf("Limit: %+v", res)
	}
	write(line(9), os.O_TRUNC)
	res, _ = ReadSince(path, res.Cursor, 0, 0)
	if msgs(res.Entries) != "m9" || !res.Truncated {
		t.Errorf("Datei geschrumpft: %+v", res)
	}
	res, _ = ReadSince(path, 0, 0, 10)
	if len(res.Entries) != 0 || !res.Truncated || res.Cursor != 0 {
		t.Errorf("Budget kleiner als eine Zeile: %+v", res)
	}
}

func TestFingerprintReihenfolge(t *testing.T) {
	cases := map[string]string{
		`Anfrage 3f2a9c1e-0dfd-28b2-eb78-0c47ed6f37b9 nach 12837 ms`: "Anfrage <id> nach <n> ms",
		`Datei C:\WORKSPACE\k3c\saves\a1.json fehlt`:                 "Datei <path> fehlt",
		`Ziel "npm:x" unbekannt`:                                     "Ziel <s> unbekannt",
		`Versuch 4 von 5, 2,5 s`:                                     "Versuch <n> von <n>, <n> s",
		strings.Repeat("x", 300):                                     strings.Repeat("x", fingerprintMax),
	}
	for in, want := range cases {
		if got := Fingerprint(in); got != want {
			t.Errorf("Fingerprint(%q) = %q, erwartet %q", in, got, want)
		}
	}
}

func TestDigestGruppen(t *testing.T) {
	res, _ := Scan(sample, Query{MinLevel: "WARN"})
	groups := Digest(res.Entries)
	if len(groups) != 2 {
		t.Fatalf("Gruppen: %+v", groups)
	}
	g := groups[0]
	if g.Count != 2 || g.NS != "mcp" || g.Level != "ERROR" || g.Fingerprint != "check_run: Ziel <s> unbekannt" ||
		g.Example != `check_run: Ziel "npm:x" unbekannt` || g.First.Minute() != 4 || g.Last.Minute() != 5 {
		t.Errorf("Gruppe: %+v", g)
	}
	if LevelRank("TRACE") != -1 || LevelRank("warn") != 2 {
		t.Error("LevelRank")
	}
}
