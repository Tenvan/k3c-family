package logs

import (
	"path/filepath"
	"testing"

	"k3c/tools/workkit/applog"
	"k3c/tools/workkit/console"
)

// Das eigene Log (workkit/applog) schreibt im Format, das Scan liest (B-046; vorher Test in internal/applog).
func TestScanLiestApplog(t *testing.T) {
	dir := filepath.Join(t.TempDir(), "logs")
	l, err := applog.Open(dir, console.New(0, nil))
	if err != nil {
		t.Fatal(err)
	}
	l.With("ns", "check").Warn("lauf beendet", "target", "task:test", "exit", 1)
	if err := l.Close(); err != nil {
		t.Fatal(err)
	}
	res, err := Scan(filepath.Join(dir, applog.Source+".jsonl"), Query{})
	if err != nil || len(res.Entries) != 1 {
		t.Fatalf("Scan: %+v, %v", res, err)
	}
	e := res.Entries[0]
	if e.Level != "WARN" || e.NS != "check" || e.Msg != "lauf beendet" || e.Data["target"] != "task:test" || e.Data["exit"] != 1.0 {
		t.Errorf("Eintrag: %+v", e)
	}
}
