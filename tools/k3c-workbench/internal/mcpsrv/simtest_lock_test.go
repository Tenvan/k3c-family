package mcpsrv

import (
	"context"
	"os"
	"strings"
	"testing"
)

// Die übrigen Tests prüfen sim_test und sim_run selbst; für sie ist die Sim-Sperre aufgehoben. Die Sperre prüfen
// die Tests unten mit t.Setenv (B-788).
func init() { _ = os.Setenv(simUnlockEnv, "1") }

// B-788/AC-02: Während der Sperre lehnen sim_test start und sim_run ab, ohne einen Lauf zu starten.
func TestSimSperreLehntAb(t *testing.T) {
	t.Setenv(simUnlockEnv, "")
	s, started := fakeServer(t, fakeBalance(t, 1))
	root := s.ws(context.Background()).root
	installFakeSim(t, root)
	_, err := s.simTest(context.Background(), simTestIn{Action: "start", Seeds: 3})
	if err == nil || !strings.Contains(err.Error(), "Beschluss 🧑 2026-11-11") || !strings.Contains(err.Error(), simUnlockEnv+"=1") {
		t.Errorf("sim_test start: %v", err)
	}
	if _, err := s.simRun(context.Background(), simIn{Seed: "abc", Ticks: 600}); err == nil ||
		!strings.Contains(err.Error(), "sim_run abgelehnt") {
		t.Errorf("sim_run: %v", err)
	}
	if n := started.Load(); n != 0 {
		t.Errorf("%d Läufe trotz Sperre gestartet", n)
	}
	if list := s.simList(root); strings.Contains(list, "run-1") {
		t.Errorf("Lauf im Register: %s", list)
	}
}

// B-788/AC-02: Mit Schalter K3C_SIM_ENTSPERRT=1 laufen sim_test start und sim_run wie bisher.
func TestSimSperreMitSchalter(t *testing.T) {
	t.Setenv(simUnlockEnv, "1")
	s, _ := fakeServer(t, fakeBalance(t, 1))
	root := s.ws(context.Background()).root
	installFakeSim(t, root)
	out, err := s.simTest(context.Background(), simTestIn{Action: "start", Seeds: 3, Wait: true})
	if err != nil || !strings.Contains(out, "fertig") {
		t.Errorf("sim_test start: %v\n%s", err, out)
	}
	text, err := s.simRun(context.Background(), simIn{Seed: "abc", Ticks: 600})
	if err != nil || !strings.Contains(text, "600 Ticks") {
		t.Errorf("sim_run: %v %q", err, text)
	}
}
