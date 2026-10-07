package mcpsrv

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"time"
)

// sim_test (B-348, TR1.1): Register, Aktionen und Modus offline mit Fake-Prozessen.

// fakeBalance schreibt beim Schritt „rechnen“ einen Bericht mit n Zielen in --report-dir.
func fakeBalance(t *testing.T, n int) func(runSpec) runResult {
	return func(spec runSpec) runResult {
		i := slices.Index(spec.args, "--report-dir")
		if i < 0 {
			return runResult{} // bauen
		}
		var targets []string
		targets = append(targets, `{"name":"Median","kind":"median","value":0,"lower":null,"upper":1,"pass":true}`)
		for k := range n {
			targets = append(targets, fmt.Sprintf(`{"name":"Ziel %d","kind":"share","value":%d,"lower":75,"upper":90,"status":"ok","pass":%v}`, k, 80+k, k == 0))
		}
		dir := filepath.Join(spec.dir, spec.args[i+1])
		if err := os.MkdirAll(dir, 0o755); err != nil {
			t.Error(err)
		}
		raw := `{"version":1,"seeds":3,"pass":false,"targets":[` + strings.Join(targets, ",") + `]}`
		if err := os.WriteFile(filepath.Join(dir, "balance-1.json"), []byte(raw), 0o644); err != nil {
			t.Error(err)
		}
		return runResult{}
	}
}

func start(t *testing.T, s *Server, in simTestIn) string {
	t.Helper()
	in.Action = "start"
	out, err := s.simTest(context.Background(), in)
	if err != nil {
		t.Fatal(err)
	}
	return strings.Fields(out)[1]
}

// waitDone wartet, bis der Lauf nicht mehr läuft, und liefert seinen Status.
func waitDone(t *testing.T, s *Server, id string) string {
	t.Helper()
	for range 500 {
		out, err := s.simTest(context.Background(), simTestIn{Action: "status", ID: id})
		if err != nil {
			t.Fatal(err)
		}
		if strings.Contains(out, "Bericht:") { // fertig, nach stop erst, wenn der Runner aufgeräumt hat
			return out
		}
		time.Sleep(10 * time.Millisecond)
	}
	t.Fatalf("%s endet nicht", id)
	return ""
}

func TestSimTestOfflineBewertetZiele(t *testing.T) {
	s, _ := fakeServer(t, fakeBalance(t, 2))
	id := start(t, s, simTestIn{Seeds: 3})
	out := waitDone(t, s, id)
	for _, want := range []string{"✅ " + id, "offline·headless·balance", "fertig", "Fail", "Pass Ziel 0: 80 % (75–90 %)",
		"Fail Ziel 1: 81 %", "Pass Median: 0 (≤ 1)", "Bericht: reports/simtest-" + id + "/simtest.md"} {
		if !strings.Contains(out, want) {
			t.Errorf("status ohne %q:\n%s", want, out)
		}
	}
	if _, err := os.Stat(filepath.Join(s.cfg.Root, "reports", "simtest-"+id, "simtest.md")); err != nil {
		t.Error(err)
	}
	if list := s.simList(s.cfg.Root); !strings.HasPrefix(list, "✅ "+id) {
		t.Errorf("list: %s", list)
	}
}

func TestSimTestStartSofortUndStop(t *testing.T) {
	blocked := make(chan struct{})
	s, _ := fakeServer(t, func(runSpec) runResult { <-blocked; return runResult{} })
	s.run = func(ctx context.Context, _ runSpec) runResult { <-ctx.Done(); return runResult{exit: -1} }
	begin := time.Now()
	id := start(t, s, simTestIn{})
	if time.Since(begin) > time.Second {
		t.Fatal("start wartet auf den Lauf")
	}
	out, err := s.simTest(context.Background(), simTestIn{Action: "stop", ID: id})
	if err != nil || !strings.Contains(out, "abgebrochen") {
		t.Fatalf("stop: %q, %v", out, err)
	}
	status := waitDone(t, s, id)
	if !strings.Contains(status, "■ "+id) || !strings.Contains(status, "abgebrochen") || !strings.Contains(status, "Bericht:") {
		t.Errorf("status nach stop:\n%s", status)
	}
	close(blocked)
}

func TestSimTestStatusHoechstensZehnZeilen(t *testing.T) {
	s, _ := fakeServer(t, fakeBalance(t, 20))
	out := waitDone(t, s, start(t, s, simTestIn{}))
	if n := len(strings.Split(out, "\n")); n > maxStatusLines {
		t.Errorf("%d Zeilen:\n%s", n, out)
	}
	if !strings.Contains(out, "weitere im Bericht") {
		t.Errorf("ohne Hinweis auf gekürzte Ziele:\n%s", out)
	}
}

func TestSimTestHoechstensZweiLaeufe(t *testing.T) {
	s, _ := fakeServer(t, nil)
	s.run = func(ctx context.Context, _ runSpec) runResult { <-ctx.Done(); return runResult{} }
	start(t, s, simTestIn{})
	start(t, s, simTestIn{})
	if _, err := s.simTest(context.Background(), simTestIn{Action: "start"}); err == nil {
		t.Error("dritter Lauf gestartet")
	}
	s.sims.cancelAll()
}

func TestSimTestLehntAb(t *testing.T) {
	s, started := fakeServer(t, func(runSpec) runResult { return runResult{} })
	cases := []simTestIn{
		{Action: "starten"},
		{Action: "start", Mode: "mock"},
		{Action: "start", Players: 9},
		{Action: "start", Clients: 5},
		{Action: "start", Seeds: 500},
		{Action: "start", Bots: "zauberer"},
		{Action: "start", Focus: []string{"spass"}},
		{Action: "start", Clients: 2},
		{Action: "start", Focus: []string{"perf"}},
		{Action: "start", Players: 2},
		{Action: "start", Mode: "online", Duration: "3h"},
		{Action: "start", Mode: "online"},
		{Action: "status", ID: "run-99"},
		{Action: "stop", ID: "run-99"},
	}
	for _, in := range cases {
		if out, err := s.simTest(context.Background(), in); err == nil {
			t.Errorf("%+v angenommen: %s", in, out)
		} else if strings.Contains(err.Error(), "\n") {
			t.Errorf("%+v: Grund nicht in einer Zeile: %v", in, err)
		}
	}
	if started.Load() != 0 {
		t.Errorf("%d Prozesse gestartet", started.Load())
	}
}
