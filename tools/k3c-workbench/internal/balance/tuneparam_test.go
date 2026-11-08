package balance

import (
	"strings"
	"testing"
)

// locateNumber findet Zahlen in Listen und verschachtelten Objekten; der Diff nennt nur geänderte Zeilen.
func TestTuneDiffVerschachtelt(t *testing.T) {
	raw := []byte("{\n  \"a\": [1, {\"b\": 2.5}],\n  \"c\": {\"d\": 3}\n}\n")
	for path, want := range map[string]string{"a.0": "1", "a.1.b": "2.5", "c.d": "3"} {
		s, e, err := locateNumber(raw, strings.Split(path, "."))
		if err != nil || string(raw[s:e]) != want {
			t.Errorf("%s: %q %v", path, raw[s:e], err)
		}
	}
	if _, _, err := locateNumber(raw, []string{"c"}); err == nil {
		t.Error("objekt als Zahl gefunden")
	}
	p, err := ParseTuneParam("economy.json:dawnGoldPerPlayer=1..9")
	if err != nil {
		t.Fatal(err)
	}
	if d, _ := proposalDiff([]TuneParam{p}, []float64{p.Base}); d != "" {
		t.Errorf("diff ohne Änderung: %q", d)
	}
}
