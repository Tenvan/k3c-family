package balance

import (
	"slices"
	"strings"
	"testing"
)

// B-640/AC-03: Delta-Bericht je Werte-Gruppe, eingefrorene nur beim Kippen, Hinweise nur für freie Gruppen.

// deltaPair liefert Baseline und Lauf auf Basis von run7: erste Hälfte der Ziele Gruppe basis, zweite holz; im Lauf
// kippt ein bisher erfülltes Ziel der Gruppe basis auf verletzt. Zurück: Baseline, Lauf, Name des gekippten Ziels.
func deltaPair(t *testing.T) (Summary, Summary, string) {
	t.Helper()
	base := fixture(t)
	for i := range base.Targets {
		base.Targets[i].Group = GroupBasis
		if i >= len(base.Targets)/2 {
			base.Targets[i].Group = GroupHolz
		}
	}
	cur := base
	cur.Targets = slices.Clone(base.Targets)
	for i, e := range cur.Targets {
		if i < len(cur.Targets)/2 && e.Status == StatusOK && e.Kind == KindShare && e.Lower != nil {
			cur.Targets[i].Value = *e.Lower - 40
			cur.Targets[i].Status = StatusViolated
			cur.Targets[i].Pass = false
			return base, cur, e.Name
		}
	}
	t.Fatal("kein erfülltes Ziel mit Untergrenze in der ersten Hälfte der Fixture")
	return base, cur, ""
}

func groupOf(in Insights, group string) *GroupDelta {
	for i := range in.Delta {
		if in.Delta[i].Group == group {
			return &in.Delta[i]
		}
	}
	return nil
}

func TestDeltaFreieGruppenImmerMitTrend(t *testing.T) {
	base, cur, name := deltaPair(t)
	in := BuildInsights(cur, &base, testHints(t), Meta{Run: "r", Baseline: "M1"})
	basis, holz := groupOf(in, GroupBasis), groupOf(in, GroupHolz)
	if basis == nil || holz == nil {
		t.Fatalf("freie Gruppen fehlen: %+v", in.Delta)
	}
	if basis.Light != LightRed || basis.Trend != TrendWorse || !slices.Contains(basis.Tipped, name) || basis.Warning != "" {
		t.Errorf("basis: %+v", *basis)
	}
	if holz.Trend != TrendSame || len(holz.Tipped) != 0 {
		t.Errorf("holz unverändert: %+v", *holz)
	}
	if groupOf(in, GroupSchiff) != nil {
		t.Error("schiff hat keine Ziele in der Fixture und fehlt im Delta")
	}
}

func TestDeltaEingefrorenNurBeimKippen(t *testing.T) {
	base, cur, name := deltaPair(t)
	meta := Meta{Run: "r", Baseline: "M1", Frozen: []string{GroupBasis, GroupHolz}}
	in := BuildInsights(cur, &base, testHints(t), meta)
	if len(in.Delta) != 1 {
		t.Fatalf("nur die gekippte eingefrorene Gruppe erscheint: %+v", in.Delta)
	}
	d := in.Delta[0]
	if d.Group != GroupBasis || !d.Frozen || !strings.Contains(d.Warning, "eingefrorene Gruppe basis kippt") || !strings.Contains(d.Warning, name) {
		t.Errorf("Warnung: %+v", d)
	}
	same := BuildInsights(base, &base, testHints(t), meta)
	if len(same.Delta) != 0 {
		t.Errorf("unveränderter Lauf: eingefrorene Gruppen schweigen: %+v", same.Delta)
	}
}

func TestDeltaHinweiseNurFuerFreieGruppen(t *testing.T) {
	cur := fixture(t)
	free := BuildInsights(cur, nil, testHints(t), Meta{Run: "r"})
	frozen := BuildInsights(cur, nil, testHints(t), Meta{Run: "r", Frozen: []string{GroupBasis}})
	count := func(in Insights, group string) (n int) {
		for _, t := range in.Targets {
			for _, l := range t.Levers {
				if l.Group == group {
					n++
				}
			}
		}
		return n
	}
	if count(free, GroupBasis) == 0 {
		t.Fatal("ohne Einfrieren nennt die Fixture Hebel der Gruppe basis")
	}
	if n := count(frozen, GroupBasis); n != 0 {
		t.Errorf("eingefrorene Gruppe basis nennt noch %d Hebel", n)
	}
	for _, h := range frozen.Hints {
		for _, l := range h.Levers {
			if l.Group == GroupBasis {
				t.Errorf("Hinweis %q nennt den Hebel %s einer eingefrorenen Gruppe", h.Target, l.Path)
			}
		}
	}
}

func TestDeltaOhneBaselineLeer(t *testing.T) {
	in := BuildInsights(fixture(t), nil, testHints(t), Meta{Run: "r"})
	if in.Delta == nil || len(in.Delta) != 0 {
		t.Errorf("ohne Baseline kein Delta, aber nie null: %#v", in.Delta)
	}
}
