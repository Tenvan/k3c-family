package logs

import (
	"regexp"
	"sort"
	"time"
)

// fingerprintMax begrenzt den Schlüssel einer Gruppe.
const fingerprintMax = 200

// masks ersetzen veränderliche Teile einer Meldung, in dieser Reihenfolge: Die Zahlen-Regel zerlegte sonst UUIDs und
// Pfade, bevor diese als Ganzes erkannt sind.
var masks = []struct {
	re   *regexp.Regexp
	with string
}{
	{regexp.MustCompile(`[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}`), "<id>"},
	{regexp.MustCompile(`(?:[A-Za-z]:)?[\\/]?(?:[\w.\-]+[\\/])+[\w.\-]+`), "<path>"},
	{regexp.MustCompile(`"[^"]*"|'[^']*'|„[^“]*“`), "<s>"},
	{regexp.MustCompile(`\d+(?:[.,]\d+)?`), "<n>"},
}

// Fingerprint macht gleichartige Meldungen gleich.
func Fingerprint(msg string) string {
	for _, m := range masks {
		msg = m.re.ReplaceAllString(msg, m.with)
	}
	if r := []rune(msg); len(r) > fingerprintMax {
		msg = string(r[:fingerprintMax])
	}
	return msg
}

// Group sind gleichartige Einträge: gleicher Namespace und gleicher Fingerabdruck.
type Group struct {
	NS          string    `json:"ns"`
	Fingerprint string    `json:"fingerprint"`
	Level       string    `json:"level"` // höchstes Level der Gruppe
	Count       int       `json:"count"`
	First       time.Time `json:"first"`
	Last        time.Time `json:"last"`
	Example     string    `json:"example"` // neueste Meldung
}

// Digest fasst Einträge zu Gruppen zusammen, häufigste zuerst, bei Gleichstand die jüngste.
func Digest(entries []Entry) []Group {
	byKey := map[string]*Group{}
	var order []*Group
	for _, e := range entries {
		fp := Fingerprint(e.Msg)
		key := e.NS + "\x00" + fp
		g, ok := byKey[key]
		if !ok {
			g = &Group{NS: e.NS, Fingerprint: fp, Level: e.Level, First: e.Time, Last: e.Time, Example: e.Msg}
			byKey[key] = g
			order = append(order, g)
		}
		g.Count++
		if LevelRank(e.Level) > LevelRank(g.Level) {
			g.Level = e.Level
		}
		if e.Time.Before(g.First) {
			g.First = e.Time
		}
		if e.Time.After(g.Last) {
			g.Last, g.Example = e.Time, e.Msg
		}
	}
	out := make([]Group, len(order))
	for i, g := range order {
		out[i] = *g
	}
	sort.SliceStable(out, func(i, j int) bool {
		if out[i].Count != out[j].Count {
			return out[i].Count > out[j].Count
		}
		return out[i].Last.After(out[j].Last)
	})
	return out
}
