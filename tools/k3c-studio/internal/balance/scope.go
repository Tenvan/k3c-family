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
	// World (BS11): jedes Ziel spielt die ganze erste Insel dieser Welt; ohne World entfallen bei einer Einschränkung
	// die Ziele mit world (Erkundung der Einzelstufe).
	World string
	// Helps: Dev-Hilfen der Insel-Läufe (helps.go), nur mit World.
	Helps string
}

// Restrict liefert eine Kopie der Ziele mit dem Szenario aus s; ein ungültiges Szenario oder ein unbekannter Grad ist
// ein Fehler.
func (t *Targets) Restrict(s Scope) (*Targets, error) {
	out := *t
	out.Targets = slices.Clone(t.Targets)
	if s.World == "" && (s.Depth != nil || s.Players != nil || s.Days != nil || s.Bot != "" || s.Grade != "") {
		// eine Einschränkung erkundet die Einzelstufe: die Ziele des Insel-Laufs (World, BS11) entfallen
		out.Targets = slices.DeleteFunc(out.Targets, func(g Target) bool { return g.World != "" })
	}
	for i := range out.Targets {
		g := &out.Targets[i]
		wasIsland := g.World != ""
		setIf(&g.Depth, s.Depth)
		setIf(&g.Players, s.Players)
		setIf(&g.Days, s.Days)
		if s.Bot != "" {
			g.Bot = s.Bot
		}
		if s.Grade != "" {
			g.Grade = s.Grade
		}
		if s.World != "" {
			g.World = s.World
			if !wasIsland && g.role() == RoleGoal {
				g.Role = RoleGuard // im Insel-Lauf ist nur das Ziel der Insel (Endboss) das Ziel (B-648)
			}
		}
		if s.Helps != "" {
			g.Helps = s.Helps
		}
		if err := g.validate(); err != nil {
			return nil, fmt.Errorf("szenario: %w", err)
		}
	}
	return &out, nil
}

func setIf(dst *int, v *int) {
	if v != nil {
		*dst = *v
	}
}
