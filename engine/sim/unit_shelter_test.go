package sim

import (
	"regexp"
	"slices"
	"strings"
	"testing"

	"k3c/engine/internal/golden"
)

// B-605/AC-01: Stirbt eine Truppe oder kommt eine hinzu, behalten die übrigen Truppen in der laufenden Gefahr ihren
// Schutzplatz. Nach der Gefahr legt die nächste den Platz neu fest.
func TestSchutzplatzStabilBeiTruppenwechsel(t *testing.T) {
	w := quietWorld(t)
	w.Cycle.Phase = "night"
	for range 5 {
		spawnVagrant(w, w.HubX, w.HubX).Kind = "peasant"
	}
	stepAll := func() {
		for _, tr := range slices.Clone(w.Troops) {
			stepPeasant(w, tr, 0.1)
		}
	}
	stepAll()
	survivors := slices.Delete(slices.Clone(w.Troops), 1, 2) // Bauer 2 stirbt
	before := make([]float64, len(survivors))
	for i, tr := range survivors {
		before[i] = shelterX(w, tr)
	}
	// Tod von Bauer 2, ein neuer Bauer vorn und einer hinten: alle Indizes der übrigen verschieben sich.
	w.Troops = slices.Delete(w.Troops, 1, 2)
	front := spawnVagrant(w, w.HubX, w.HubX)
	front.Kind = "peasant"
	w.Troops = append([]*Troop{front}, w.Troops[:len(w.Troops)-1]...)
	spawnVagrant(w, w.HubX, w.HubX).Kind = "peasant"
	stepAll()
	for i, tr := range survivors {
		if got := shelterX(w, tr); got != before[i] {
			t.Fatalf("Bauer %d: Schutzplatz %v statt %v nach Truppenwechsel", i, got, before[i])
		}
	}
	w.Cycle.Phase = "day"
	stepAll()
	w.Cycle.Phase = "night"
	if got, want := shelterX(w, survivors[0]), w.HubX+float64(slices.Index(w.Troops, survivors[0])%5-2); got != want {
		t.Fatalf("nächste Gefahr: Schutzplatz %v statt %v (neu nach Index)", got, want)
	}
}

// extraSite ist der zusätzliche unbezahlte Hub-Platz für B-605/AC-02: frei zwischen Werkstatt (-16) und Lager (-26),
// erst ab Hub-Stufe 5 bezahlbar; die Waffenkammer bleibt so ab Stufe 4 baubar.
var extraSite = HubSite{Kind: "armory", OffsetUnits: -20, HubLevel: 5}

// B-605/AC-02 (Nachtrag zu B-260/AC-01): Ein zusätzlicher unbezahlter Eintrag in hub.json › sites ändert in den
// Golden-Läufen nur IDs und den neuen Platz. Jeder Lauf rechnet mit und ohne Eintrag; verglichen wird jeder Snapshot ohne
// ID-Felder und ohne den neuen Platz.
func TestGoldenZusatzplatzNurIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("rechnet alle Golden-Läufe doppelt")
	}
	h := cloneHub(&hub)
	h.Sites = append(h.Sites, extraSite)
	for _, run := range goldenRuns(t) {
		t.Run(run.Name, func(t *testing.T) {
			goldenPurse(t)
			base, extra := goldenWorld(t, run, nil), goldenWorld(t, run, h)
			extraX := extra.HubX + extraSite.OffsetUnits
			tick := 0
			for _, seg := range run.Inputs {
				for range seg.Ticks {
					Step(base, seg.Commands, run.Dt)
					Step(extra, seg.Commands, run.Dt)
					if tick++; tick%run.SnapshotEvery != 0 {
						continue
					}
					want, got := idFreeTree(t, base), idFreeTree(t, extra)
					got.(map[string]any)["sites"] = withoutSite(t, got.(map[string]any)["sites"], extraX)
					if d := golden.Diff("world", want, got); d != "" {
						t.Fatalf("sim-%s Tick %d: Zusatzplatz ändert %s", run.Name, tick, d)
					}
				}
			}
		})
	}
}

// eventIDs sind die Felder von Ereignissen, die eine Entity-ID tragen (Absender, Ziel, Bauplatz, Boss).
var eventIDs = []string{"from", "to", "site", "boss"}

// idFreeTree ist der JSON-Baum der Welt ohne IDs (withoutIDs, eventIDs).
func idFreeTree(t *testing.T, w *World) any {
	t.Helper()
	tree := withoutIDs(golden.Tree(t, w)).(map[string]any)
	events, _ := tree["events"].([]any)
	for _, e := range events {
		for _, k := range eventIDs {
			delete(e.(map[string]any), k)
		}
	}
	return tree
}

// idRef ist ein Verweis mit ID als Text, z. B. payKey "site:55".
var idRef = regexp.MustCompile(`^([a-z]+):\d+$`)

// withoutIDs entfernt alle ID-Felder (id, …Id, …Ids) und die ID aus Verweisen wie "site:55" aus einem JSON-Baum.
func withoutIDs(v any) any {
	switch x := v.(type) {
	case string:
		return idRef.ReplaceAllString(x, "$1:")
	case map[string]any:
		for k, e := range x {
			if k == "id" || strings.HasSuffix(k, "Id") || strings.HasSuffix(k, "Ids") {
				delete(x, k)
				continue
			}
			x[k] = withoutIDs(e)
		}
	case []any:
		for i, e := range x {
			x[i] = withoutIDs(e)
		}
	}
	return v
}

// withoutSite nimmt den einen Zusatzplatz (Art und x) aus der Liste der Bauplätze; er muss unbezahlt bleiben.
func withoutSite(t *testing.T, sites any, x float64) any {
	t.Helper()
	list := sites.([]any)
	i := slices.IndexFunc(list, func(e any) bool {
		s := e.(map[string]any)
		return s["kind"] == extraSite.Kind && s["x"] == x
	})
	if i < 0 {
		t.Fatalf("Zusatzplatz bei x=%v fehlt", x)
	}
	if s := list[i].(map[string]any); s["state"] != "unpaid" {
		t.Fatalf("Zusatzplatz nicht unbezahlt: %v", s)
	}
	return slices.Delete(list, i, i+1)
}
