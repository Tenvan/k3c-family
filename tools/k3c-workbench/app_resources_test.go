package main

import (
	"testing"
	"time"

	"k3c/tools/k3c-workbench/internal/resmon"
)

func TestResourcesOhneMessgeraetIstAus(t *testing.T) {
	if v := (&App{}).Resources(60); v.Enabled {
		t.Fatal("ohne Messgerät darf die Ansicht nicht aktiv sein")
	}
}

func TestResourcesLiefertPunkteUndKlemmtDenZeitraum(t *testing.T) {
	m := resmon.New(resmon.Config{Interval: time.Second})
	m.Gauge("Konsole-Text", "B", func() int64 { return 42 })
	m.Track("tool:x")()
	m.Sample()
	m.Sample()
	v := (&App{res: m}).Resources(1_000_000)
	if !v.Enabled || v.WindowMin != resMaxWindow || len(v.Points) == 0 || len(v.Gauges) != 1 || !v.Gauges[0].Bytes {
		t.Fatalf("Ansicht: %+v", v)
	}
	if got := v.Points[len(v.Points)-1]; len(got.GaugeValues) != 1 || got.GaugeValues[0] != 42 || got.Goroutines < 1 {
		t.Fatalf("Punkt: %+v", got)
	}
	if len(v.Ops) != 1 || v.Ops[0].Name != "tool:x" || v.Profiles == nil || v.Children == nil {
		t.Fatalf("Vorgänge/Listen: %+v", v)
	}
}
