package mcpsrv

import (
	"context"
	"reflect"
	"regexp"
	"strings"
	"testing"
	"time"
)

var reRunID = regexp.MustCompile(`Lauf-ID (L\d+)`)

// B-372/AC-01: Ein Lauf, der länger als die Wartezeit dauert, liefert eine Lauf-ID; über sie kommen Exit-Code und
// Fehlerzeilen, sobald er endet.
func TestCheckRunLangerLaufLiefertLaufID(t *testing.T) {
	release := make(chan struct{})
	s, _ := fakeServer(t, func(spec runSpec) runResult {
		<-release
		spec.out("stderr", "--- FAIL: TestKaputt (0.00s)")
		return runResult{exit: 1, dur: 90 * time.Second}
	})
	s.checkWait = 20 * time.Millisecond
	ctx := context.Background()
	text, err := s.checkRun(ctx, checkIn{Target: "task:check:go"})
	m := reRunID.FindStringSubmatch(text)
	if err != nil || m == nil || !strings.Contains(text, "läuft noch") {
		t.Fatalf("erste Antwort: %q, %v", text, err)
	}
	id := m[1]
	if text, err := s.checkRun(ctx, checkIn{Run: id}); err != nil || !strings.Contains(text, "läuft noch") {
		t.Fatalf("laufend: %q, %v", text, err)
	}
	if _, err := s.checkRun(ctx, checkIn{Target: "task:check:go"}); err == nil || !strings.Contains(err.Error(), "läuft bereits") {
		t.Fatalf("zweiter Lauf desselben Ziels: %v", err)
	}
	close(release)
	<-s.checks.job(id).done // B-403/N6: auf das Ende des Laufs warten, nicht auf Zeit
	got, _ := s.checkRun(ctx, checkIn{Run: id})
	if !strings.Contains(got, "task:check:go · exit 1") || !strings.Contains(got, "FAIL: TestKaputt") {
		t.Fatalf("Ergebnis: %q", got)
	}
}

func TestCheckRunUnbekannteLaufID(t *testing.T) {
	s, _ := fakeServer(t, func(runSpec) runResult { return runResult{} })
	if _, err := s.checkRun(context.Background(), checkIn{Run: "L99"}); err == nil || !strings.Contains(err.Error(), "L99") {
		t.Fatalf("erwartet Fehler mit der ID, bekam %v", err)
	}
}

// Ein kurzer Lauf antwortet wie bisher direkt.
func TestCheckRunKurzerLaufAntwortetDirekt(t *testing.T) {
	s, _ := fakeServer(t, func(runSpec) runResult { return runResult{dur: 2 * time.Second} })
	text, err := s.checkRun(context.Background(), checkIn{Target: "task:test"})
	if err != nil || text != "task:test · exit 0 · 2,0 s" {
		t.Fatalf("Antwort: %q, %v", text, err)
	}
}

// B-372/AC-02: Die Server-Anweisungen nennen alle Parameter von console_tail.
func TestInstructionsNennenConsoleTailParameter(t *testing.T) {
	ty := reflect.TypeOf(tailIn{})
	for i := 0; i < ty.NumField(); i++ {
		name := strings.Split(ty.Field(i).Tag.Get("json"), ",")[0]
		if !strings.Contains(Instructions(), "console_tail {") || !strings.Contains(consoleTailLine(t), name) {
			t.Errorf("Parameter %q fehlt in der Zeile zu console_tail: %q", name, consoleTailLine(t))
		}
	}
}

func consoleTailLine(t *testing.T) string {
	t.Helper()
	for _, l := range strings.Split(Instructions(), "\n") {
		if strings.HasPrefix(strings.TrimSpace(l), "- `console_tail {") {
			return l
		}
	}
	t.Fatal("keine Zeile zu console_tail in den Anweisungen")
	return ""
}

// B-403/N5: Ein Job hält nur Fehlerzeilen und das Ende der Ausgabe; die Antwort bleibt dieselbe.
func TestCheckJobHaeltNurFehlerzeilen(t *testing.T) {
	j := &checkJob{t: checkTarget{kinds: []string{kindGoTest}}}
	for i := 0; i < 1000; i++ {
		j.add("ok zeile")
	}
	j.add("--- FAIL: TestX (0.00s)")
	if len(j.output) != 1 || len(j.tail) != fallbackLines {
		t.Fatalf("gehalten: %d Fehlerzeilen, %d Endzeilen", len(j.output), len(j.tail))
	}
	j.setResult(runResult{exit: 1})
	got, _ := j.answer()
	if !strings.Contains(got, "--- FAIL: TestX") {
		t.Fatalf("Fehlerzeile fehlt: %q", got)
	}
}

// Review WZ2.8: „läuft bereits“ nennt die Lauf-ID des laufenden Laufs.
func TestCheckRunLaeuftBereitNenntLaufID(t *testing.T) {
	release := make(chan struct{})
	s, started := fakeServer(t, func(runSpec) runResult { <-release; return runResult{} })
	s.checkWait = 10 * time.Millisecond
	ctx := context.Background()
	text, err := s.checkRun(ctx, checkIn{Target: "task:test"})
	m := reRunID.FindStringSubmatch(text)
	if err != nil || m == nil || started.Load() != 1 {
		t.Fatalf("erste Antwort: %q, %v", text, err)
	}
	_, err = s.checkRun(ctx, checkIn{Target: "task:test"})
	if err == nil || !strings.Contains(err.Error(), "läuft bereits") || !strings.Contains(err.Error(), m[1]) {
		t.Fatalf("erwartet läuft bereits mit %s, bekam %v", m[1], err)
	}
	close(release)
}
