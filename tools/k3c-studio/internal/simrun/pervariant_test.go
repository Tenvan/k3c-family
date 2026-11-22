package simrun

import (
	"fmt"
	"slices"
	"strings"
	"testing"
	"unicode/utf8"
)

// line: Label, Varianten, Seeds und Bot eines Laufs in einer Zeile.
func line(i Info) string {
	return fmt.Sprintf("%s|%v|%v|%s|%v", i.Label, i.Spec.Vary, i.Spec.SeedList, i.Spec.Bot, i.Spec.PerVariant)
}

// B-662/AC-05: „je Variante“ legt n + 1 Läufe an: Basis und je Variante einer, gemeinsames Label-Präfix, gleiche Seeds.
func TestJeVarianteLegtLaeufeAn(t *testing.T) {
	root := t.TempDir()
	g := writingRegistry(t, `{"version":1,"seeds":1,"pass":true}`)
	vary := []string{"economy.json:purse.startGold=120", "hub.json:tree.wood=2", "troops.json:archer.cost=5"}
	seeds := []string{"k3c-dev", "wald-1"}
	base, err := g.StartOffline(root, "Repo-Wurzel", Spec{SeedList: seeds, Bot: "coop2", Vary: vary, Label: "Delta M2", PerVariant: true})
	if err != nil {
		t.Fatal(err)
	}
	want := []string{fmt.Sprintf("Delta M2 · Basis|[]|%v|coop2|false", seeds)}
	for _, v := range vary {
		want = append(want, fmt.Sprintf("Delta M2 · %s|[%s]|%v|coop2|false", v, v, seeds))
	}
	runs := g.List(root)
	slices.Reverse(runs) // List: neueste zuerst
	var got []string
	for _, r := range runs {
		got = append(got, line(r))
	}
	if !slices.Equal(got, want) || base.ID != runs[0].ID {
		t.Errorf("Läufe:\n%s\nerwartet:\n%s (Basis %s)", strings.Join(got, "\n"), strings.Join(want, "\n"), base.ID)
	}
	for _, r := range runs {
		waitFor(t, g, root, r.ID, func(i Info) bool { return i.State == Done })
	}
}

// B-662/AC-05: „je Variante“ lehnt Bereiche (Abstimmung) und einen Auftrag ohne feste Variante ab, ohne Lauf.
func TestJeVarianteLehntAb(t *testing.T) {
	root := t.TempDir()
	g := writingRegistry(t, `{"version":1,"seeds":1,"pass":true}`)
	tune := &Tune{Mode: "sweep", Params: []string{"economy.json:purse.startGold=100..200"}}
	for spec, why := range map[*Spec]string{
		{Vary: []string{"hub.json:tree.wood=2"}, PerVariant: true, Tune: tune}: "je Variante geht nicht mit Abstimmung",
		{PerVariant: true}: "mindestens eine feste Variante",
	} {
		if _, err := g.StartOffline(root, "Repo-Wurzel", *spec); err == nil || !strings.Contains(err.Error(), why) {
			t.Errorf("%+v: %v, erwartet „%s“", spec, err, why)
		}
	}
	if n := len(g.List(root)); n != 0 {
		t.Errorf("abgelehnte Aufträge legten %d Läufe an", n)
	}
}

// Labels bleiben in der Grenze: zu lang kürzt zuerst das Präfix, die Variante bleibt lesbar.
func TestJeVarianteLabelGrenze(t *testing.T) {
	long := strings.Repeat("Langes Label ", 5)[:maxLabel]
	v := "buildings.json:castle.levels.2.cost=12"
	specs := Spec{Label: long, Vary: []string{v}, PerVariant: true}.perVariant()
	for _, s := range specs {
		if utf8.RuneCountInString(s.Label) > maxLabel || s.Check() != nil {
			t.Errorf("Label %q (%d Zeichen): %v", s.Label, utf8.RuneCountInString(s.Label), s.Check())
		}
	}
	if !strings.HasSuffix(specs[1].Label, " · "+v) {
		t.Errorf("Variante abgeschnitten: %q", specs[1].Label)
	}
	if got := (Spec{Vary: []string{v}, PerVariant: true}).perVariant()[0].Label; got != "je Variante · Basis" {
		t.Errorf("ohne Label: %q", got)
	}
}
