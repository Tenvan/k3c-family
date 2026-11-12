package planning

import "testing"

// B-783/AC-03: plan_next prüft wie plan_waves nach Dateimengen und Sperren: R1.1 (DEV, in Arbeit) sperrt A1.1 (DEV)
// nicht, und C1.2 (DEV, disjunkte Erlaubte Dateien) ist parallel möglich; B1.1 (Sperre exe) und C1.1 (Bereich unter
// tools/a/**) nicht.
func TestNextParallelGleicheDomaeneDisjunkteDateien(t *testing.T) {
	out := must(t)(Next(wavesRepo(t), NextFilter{}))
	checkText(t, out, []string{"Nächste Session: A1.1 · Titel A1.1", "── Parallel möglich ──\n- DEV · C1.2 · Titel C1.2 (Sprint C1, offline)"},
		[]string{"- DEV · B1.1", "- DEV · C1.1", "gesperrt"})
}

// Ohne Dateimengen bleibt die alte Sperre nach Domäne; mit Dateimengen nennt der Grund die Kollision.
func TestNextGrundBeiKollision(t *testing.T) {
	out := must(t)(Next(wavesRepo(t), NextFilter{Domain: "DEV"}))
	checkText(t, out, []string{"Nächste Session: A1.1"}, nil)
	running := []scoped{{session: Session{Nr: "R1.1", Domain: "DEV"}, area: []string{"tools/a/**"}}}
	x := scoped{session: Session{Nr: "C1.1", Domain: "DEV"}, area: []string{"tools/a/sub/file.go"}}
	if got := busy(running, x); got != "Bereich `tools/a/**` / `tools/a/sub/file.go` gesperrt (R1.1 in Arbeit)" {
		t.Errorf("busy: %q", got)
	}
	x.area = nil
	if got := busy(running, x); got != "Domäne DEV gesperrt (R1.1 in Arbeit)" {
		t.Errorf("busy ohne Dateimenge: %q", got)
	}
}
