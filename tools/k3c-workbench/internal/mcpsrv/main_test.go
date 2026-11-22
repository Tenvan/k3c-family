package mcpsrv

import (
	"flag"
	"os"
	"testing"
)

func TestMain(m *testing.M) {
	// Höchstens sechs parallele Tests, wenn -parallel nicht gesetzt ist: Mit GOMAXPROCS warten git- und Task-Prozesse
	// aufeinander und ein einzelner Test zeigt über 3 s (B-830). Die Asset-Tests samt CLI-Attrappe liegen seit WZ63 in
	// k3c-studio.
	flag.Parse()
	set := false
	flag.Visit(func(f *flag.Flag) { set = set || f.Name == "test.parallel" })
	if !set {
		_ = flag.Set("test.parallel", "6")
	}
	os.Exit(m.Run())
}
