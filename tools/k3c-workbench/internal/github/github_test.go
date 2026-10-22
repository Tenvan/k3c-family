package github

import (
	"context"
	"errors"
	"os"
	"os/exec"
	"strings"
	"testing"
	"time"
)

func read(t *testing.T, name string) []byte {
	t.Helper()
	b, err := os.ReadFile("testdata/" + name)
	if err != nil {
		t.Fatal(err)
	}
	return b
}

func TestParse(t *testing.T) {
	d, err := Parse(read(t, "prs.json"), read(t, "runs.json"), time.Date(2026, 10, 4, 12, 0, 0, 0, time.UTC))
	if err != nil {
		t.Fatal(err)
	}
	want := map[string]SprintPR{
		"M8":  {Number: 110, State: "offen", CI: "grün", Merge: "konfliktfrei"}, // neuer PR gewinnt, neuerer Check-Lauf gewinnt
		"M7":  {Number: 108, State: "Entwurf", CI: "läuft", Merge: "Konflikt"},  // CheckRun läuft, StatusContext grün
		"LT1": {Number: 107, State: "offen", CI: "rot", Merge: "unbekannt"},     // StatusContext ERROR
		"S5":  {Number: 95, State: "gemergt", CI: "–", Merge: "–"},              // ohne Checks, nicht offen
	}
	if len(d.Sprints) != len(want) {
		t.Fatalf("Sprints: %+v (feature/x zählt nicht)", d.Sprints)
	}
	for id, w := range want {
		g := d.Sprints[id]
		if g.Number != w.Number || g.State != w.State || g.CI != w.CI || g.Merge != w.Merge {
			t.Errorf("%s: %+v, erwartet %+v", id, g, w)
		}
	}
	if d.Develop == nil || d.Develop.CI != "rot" || d.Develop.Title != "M7 · Seiten (#102)" {
		t.Errorf("develop: %+v", d.Develop)
	}
	text := Text(d)
	for _, line := range []string{"M8 #110 offen · CI grün · konfliktfrei · https://", "S5 #95 gemergt · CI – · https://", "develop: CI rot"} {
		if !strings.Contains(text, line) {
			t.Errorf("Text ohne %q:\n%s", line, text)
		}
	}
}

func TestParseLeerUndKaputt(t *testing.T) {
	d, err := Parse([]byte("[]"), nil, time.Now())
	if err != nil || len(d.Sprints) != 0 || d.Develop != nil || Text(d) != "keine Sprint-PRs" {
		t.Fatalf("leer: %+v %v", d, err)
	}
	if _, err := Parse([]byte("kein json"), nil, time.Now()); err == nil {
		t.Fatal("kaputtes JSON ohne Fehler")
	}
}

func TestClientHinweisUndZwischenspeicher(t *testing.T) {
	c := New(t.TempDir())
	calls := 0
	c.run = func(context.Context, string, []string) ([]byte, error) {
		calls++
		return nil, &exec.Error{Name: "gh", Err: exec.ErrNotFound}
	}
	d := c.Status(context.Background(), false)
	if !strings.Contains(d.Error, "gh nicht gefunden") || d.Sprints == nil {
		t.Fatalf("gh fehlt: %+v", d)
	}
	prs := read(t, "prs.json")
	c.run = func(_ context.Context, _ string, args []string) ([]byte, error) {
		calls++
		if args[1] == "run" {
			return nil, errors.New("kein Netz") // develop-Lauf fehlt, PRs reichen
		}
		return prs, nil
	}
	if d := c.Status(context.Background(), false); d.Error != "" || d.Sprints["M8"].Number != 110 || d.Develop != nil {
		t.Fatalf("Abruf: %+v", d)
	}
	n := calls
	c.Status(context.Background(), false)
	if calls != n {
		t.Fatal("Zwischenspeicher umgangen")
	}
	c.run = func(context.Context, string, []string) ([]byte, error) { return nil, context.DeadlineExceeded }
	if d := c.Status(context.Background(), true); d.Sprints["M8"].Number != 110 || !strings.Contains(d.Error, "Stand von") {
		t.Fatalf("veraltet: %+v", d)
	}
}
