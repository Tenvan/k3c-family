package playtest

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func fiStore(t *testing.T) *Store {
	t.Helper()
	_, _, dir := trRig(t, true)
	return &Store{Root: filepath.Dir(dir)}
}

const fiSession = "2026-11-22-1"

func TestSetFindingsSchreibtUndBehaeltHaken(t *testing.T) {
	st := fiStore(t)
	in := []FindingIn{{ID: "f1", Quote: "Ich finde den Bauarbeiter nicht", Ts: 1_005_000, Type: "Problem", Project: "WZG"},
		{ID: "f2", Quote: "Zu dunkel", Ts: 1_020_500, DuplicateOf: "B-1100"}}
	if a, u, err := st.SetFindings(fiSession, in); err != nil || a != 2 || u != 0 {
		t.Fatalf("neu: %d %d %v", a, u, err)
	}
	on := true
	if _, err := st.UpdateFinding(fiSession, "f1", FindingPatch{Selected: &on}); err != nil {
		t.Fatal(err)
	}
	in[0].Quote = "Bauarbeiter fehlt"
	if a, u, err := st.SetFindings(fiSession, in[:1]); err != nil || a != 0 || u != 1 {
		t.Fatalf("Update: %d %d %v", a, u, err)
	}
	list, _ := st.Findings(fiSession)
	if len(list) != 2 || list[0].Quote != "Bauarbeiter fehlt" || !list[0].Selected {
		t.Errorf("Haken muss bleiben: %+v", list)
	}
	ses, _ := st.Get(fiSession)
	if ses.Stage != StageEvaluated {
		t.Errorf("Stufe: %s", ses.Stage)
	}
}

func TestSetFindingsFehler(t *testing.T) {
	st := fiStore(t)
	cases := map[string][]FindingIn{
		"id":        {{ID: "a b", Quote: "x"}},
		"quote":     {{ID: "a"}},
		"type":      {{ID: "a", Quote: "x", Type: "Bug"}},
		"prio":      {{ID: "a", Quote: "x", Prio: "dringend"}},
		"duplicate": {{ID: "a", Quote: "x", DuplicateOf: "Ticket 5"}},
		"doppelt":   {{ID: "a", Quote: "x"}, {ID: "a", Quote: "y"}},
		"leer":      {},
	}
	for name, in := range cases {
		if _, _, err := st.SetFindings(fiSession, in); !errors.Is(err, ErrBadFinding) {
			t.Errorf("%s: %v", name, err)
		}
	}
	if _, _, err := st.SetFindings("2026-01-01-9", []FindingIn{{ID: "a", Quote: "x"}}); !errors.Is(err, ErrNotFound) {
		t.Errorf("unbekannte Session: %v", err)
	}
	if _, _, err := st.SetFindings(fiSession, []FindingIn{{ID: "ok", Quote: "x"}, {ID: "b", Quote: ""}}); err == nil {
		t.Error("ein ungültiger Befund muss alles ablehnen")
	}
	if l, _ := st.Findings(fiSession); len(l) != 0 {
		t.Errorf("nichts darf geschrieben sein: %v", l)
	}
}

// fakePlan hält Tickets im Speicher.
type fakePlan struct {
	n       int
	created []map[string]string
	notes   map[string]string
	failFor string
}

func (f *fakePlan) planner() Planner {
	return Planner{
		Create: func(slug, title string, fields map[string]string) (string, error) {
			if f.failFor != "" && strings.Contains(title, f.failFor) {
				return "", errors.New("Sprint X gibt es nicht")
			}
			f.n++
			f.created = append(f.created, fields)
			return fmt.Sprintf("B-%d angelegt: docs/backlog/B-%d-%s.md\nTitel %s", 1300+f.n, 1300+f.n, slug, title), nil
		},
		Get: func(id string) (string, error) {
			if id != "B-1100" {
				return "", errors.New("unbekannte ID " + id)
			}
			return "# B-1100 · x\n\n## Notizen\n\n" + f.notes[id] + "\n\n## Ende\n", nil
		},
		Sections: func(id string, b map[string]string) error {
			if f.notes == nil {
				f.notes = map[string]string{}
			}
			if n, ok := b["Notizen"]; ok {
				f.notes[id] = n
			} else {
				f.notes[id+"/neu"] = b["Ausgangslage"]
			}
			return nil
		},
	}
}

// ticketsErsterLauf legt die Befunde an, hakt vier davon an und erzeugt die Tickets einmal.
func ticketsErsterLauf(t *testing.T) (*Store, *fakePlan, TicketResult) {
	t.Helper()
	st := fiStore(t)
	_, _, err := st.SetFindings(fiSession, []FindingIn{
		{ID: "f1", Quote: "Ich finde den Bauarbeiter nicht", Ts: 1_005_000, Type: "Problem", Frame: "frames/marker-1.jpg", Context: "Insel 2 Stufe 3"},
		{ID: "f2", Quote: "Zu dunkel", Ts: 1_020_500, DuplicateOf: "B-1100", Context: "Nacht"},
		{ID: "f3", Quote: "nicht angehakt", Type: "Idee"},
		{ID: "f4", Quote: "scheitert", Type: "Idee"},
		{ID: "f5", Quote: "Dup unbekannt", DuplicateOf: "B-9"},
	})
	if err != nil {
		t.Fatal(err)
	}
	on := true
	for _, id := range []string{"f1", "f2", "f4", "f5"} {
		if _, err := st.UpdateFinding(fiSession, id, FindingPatch{Selected: &on}); err != nil {
			t.Fatal(err)
		}
	}
	fp := &fakePlan{failFor: "scheitert"}
	res, err := st.CreateTickets(fiSession, TicketDefaults{Domain: "REG", Project: "WZG", Sprint: "WZ65", Prio: "mittel"}, fp.planner())
	if err != nil {
		t.Fatal(err)
	}
	return st, fp, res
}

func TestTicketsNurAngehakteOhneDuplikat(t *testing.T) {
	_, fp, res := ticketsErsterLauf(t)
	if len(res.Created) != 1 || len(res.Noted) != 1 || len(res.Failed) != 2 || res.Skipped != 1 {
		t.Fatalf("Ergebnis: %+v", res)
	}
	if len(fp.created) != 1 || fp.created[0]["Projekt"] != "WZG" || fp.created[0]["Sprint"] != "WZ65" || fp.created[0]["Typ"] != "Problem" || fp.created[0]["Domäne"] != "REG" {
		t.Errorf("Felder: %v", fp.created)
	}
	text := fp.notes["B-1301/neu"]
	for _, want := range []string{"> Ich finde den Bauarbeiter nicht", "Insel 2 Stufe 3", "playtests/" + fiSession + "/frames/marker-1.jpg", fiSession} {
		if !strings.Contains(text, want) {
			t.Errorf("Ticket ohne %q: %s", want, text)
		}
	}
	if n := fp.notes["B-1100"]; !strings.Contains(n, "Befund f2") || !strings.Contains(n, "Zu dunkel") {
		t.Errorf("Hinweis im Duplikat: %q", n)
	}
}

// Befunde nach dem Lauf: Ticket nur bei angehakten Befunden mit Erfolg, Fehler stehen am Befund.
func TestTicketsBefundStatus(t *testing.T) {
	st, _, _ := ticketsErsterLauf(t)
	list, _ := st.Findings(fiSession)
	by := map[string]Finding{}
	for _, f := range list {
		by[f.ID] = f
	}
	if by["f1"].Ticket != "B-1301" || by["f3"].Ticket != "" || by["f4"].Error == "" || by["f5"].Error == "" || by["f2"].Ticket != "" {
		t.Errorf("findings.json: %+v", by)
	}
}

// Zweiter Lauf: kein zweites Ticket, kein zweiter Hinweis; ein Befund mit Ticket bleibt gesperrt.
func TestTicketsZweiterLauf(t *testing.T) {
	st, fp, _ := ticketsErsterLauf(t)
	res, err := st.CreateTickets(fiSession, TicketDefaults{Project: "WZG", Sprint: "WZ65", Domain: "REG"}, fp.planner())
	if err != nil || len(res.Created) != 0 || len(fp.created) != 1 || strings.Count(fp.notes["B-1100"], "Befund f2") != 1 {
		t.Errorf("zweiter Lauf: %+v %v %d", res, err, len(fp.created))
	}
	if _, err := st.UpdateFinding(fiSession, "f1", FindingPatch{Selected: new(bool)}); err == nil {
		t.Error("Befund mit Ticket darf nicht mehr geändert werden")
	}
}

func TestTicketsOhnePflichtfelder(t *testing.T) {
	st := fiStore(t)
	_, _, _ = st.SetFindings(fiSession, []FindingIn{{ID: "f1", Quote: "x", Type: "Idee"}})
	on := true
	_, _ = st.UpdateFinding(fiSession, "f1", FindingPatch{Selected: &on})
	fp := &fakePlan{}
	res, err := st.CreateTickets(fiSession, TicketDefaults{Domain: "REG"}, fp.planner())
	if err != nil || len(res.Failed) != 1 || !strings.Contains(res.Failed[0], "Projekt, Sprint fehlt") || len(fp.created) != 0 {
		t.Errorf("%+v %v", res, err)
	}
}

func TestFrameAtZieltBildUndLehntAb(t *testing.T) {
	tr, j, dir := trRig(t, true)
	p, err := tr.FrameAt(fiSession, 1_012_000)
	if err != nil || filepath.Base(p) != "t-1012000.jpg" {
		t.Fatalf("%q %v", p, err)
	}
	if !strings.Contains(strings.Join(j.calls[0], " "), "-ss 12.000") {
		t.Errorf("Aufruf: %v", j.calls[0])
	}
	if _, err := tr.FrameAt(fiSession, 1_012_000); err != nil || len(j.calls) != 1 {
		t.Errorf("vorhandenes Bild wiederverwenden: %v %d", err, len(j.calls))
	}
	if err := os.Remove(filepath.Join(dir, fileVideo)); err != nil {
		t.Fatal(err)
	}
	if _, err := tr.FrameAt(fiSession, 5); !errors.Is(err, ErrNoVideo) {
		t.Errorf("ohne Video: %v", err)
	}
	if _, err := tr.FrameAt("2026-05-05-5", 5); !errors.Is(err, ErrNotFound) {
		t.Errorf("unbekannte Session: %v", err)
	}
}
