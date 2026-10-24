package balance

import (
	"fmt"
	"slices"
)

// Scope legt das Szenario der Zielläufe fest (WZ28.4, CLI --depth, --players, --bot, --grade, --days): Jeder gesetzte
// Wert ersetzt den Wert jedes Ziels, nil bzw. leer = wie in data/balance-targets.json. Die Korridore bleiben; ein Ziel
// mit anderem Szenario ist eine Erkundung, keine Prüfung gegen den bestätigten Korridor.
type Scope struct {
	Depth, Players, Days *int
	Bot, Grade           string
}

// Restrict liefert eine Kopie der Ziele mit dem Szenario aus s; ein ungültiges Szenario oder ein unbekannter Grad ist
// ein Fehler.
func (t *Targets) Restrict(s Scope) (*Targets, error) {
	out := *t
	out.Targets = slices.Clone(t.Targets)
	for i := range out.Targets {
		g := &out.Targets[i]
		setIf(&g.Depth, s.Depth)
		setIf(&g.Players, s.Players)
		setIf(&g.Days, s.Days)
		if s.Bot != "" {
			g.Bot = s.Bot
		}
		if err := g.validate(); err != nil {
			return nil, fmt.Errorf("szenario: %w", err)
		}
	}
	if s.Grade != "" {
		grades, err := Grades()
		if err != nil {
			return nil, err
		}
		if !slices.Contains(grades, s.Grade) {
			return nil, fmt.Errorf("szenario: unbekannter Grad %q (bekannt: %v)", s.Grade, grades)
		}
		out.grade = s.Grade
	}
	return &out, nil
}

func setIf(dst *int, v *int) {
	if v != nil {
		*dst = *v
	}
}
