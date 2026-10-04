package conlog

import (
	"bytes"
	"errors"
	"log/slog"
	"strings"
	"testing"
)

func TestZeileOhneFarben(t *testing.T) {
	var buf bytes.Buffer
	log := slog.New(New(&buf, slog.LevelDebug, false)).With("ns", "room").WithGroup("req")
	log.Error("Raum zu", "code", "FAMILIE", "err", errors.New("weg da"), slog.Group("p", "x", 1), "leer", "")
	got := buf.String()[len("15:04:05.000 "):]
	want := `ERROR [room] Raum zu req.code=FAMILIE req.err="weg da" req.p.x=1 req.leer=""` + "\n"
	if got != want {
		t.Errorf("\n got %q\nwant %q", got, want)
	}
}

func TestFarbenUndLevel(t *testing.T) {
	var buf bytes.Buffer
	log := slog.New(New(&buf, slog.LevelInfo, true))
	log.Debug("versteckt")
	log.Warn("achtung", "ns", "svc")
	out := buf.String()
	if strings.Contains(out, "versteckt") || !strings.Contains(out, yellow+"WARN ") || !strings.Contains(out, cyan+"[svc]") {
		t.Errorf("%q", out)
	}
}
