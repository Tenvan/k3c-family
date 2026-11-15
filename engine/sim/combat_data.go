package sim

import (
	"errors"
	"fmt"
)

// Kampfvariante je Welt (docs/rules/kampf.md § 3, B-932, B-933): Welt-Datei › `combat` {mode, groupSize}, Werte in
// data/economy.json › combat.

const (
	// CombatGroups: Gruppen mit Schild und Auslöser am Tor (Standard).
	CombatGroups = "groups"
	// CombatGeneral: Banner je Monarch, ihm folgt ein Trupp.
	CombatGeneral = "general"
	// DefaultGroupSize: Soldaten je Gruppe neben dem Führer (kampf.md § 3.1).
	DefaultGroupSize = 5
)

// CombatSettings ist `combat` einer Welt; leer = Standard (groups, 5).
type CombatSettings struct {
	Mode      string `json:"mode,omitempty"`
	GroupSize int    `json:"groupSize,omitempty"`
}

// combatData sind die Werte aus data/economy.json › combat (Startwerte, kampf.md § 8).
type combatData struct {
	ShieldCost, TriggerCost, BannerCost int
	TroopSoldiers, TroopArchers         int
	EngageUnits                         float64 // Gegner und Portale in diesem Abstand voraus greift eine Gruppe oder ein Trupp an
	ShieldOffsetUnits                   float64 // Schild-Zahlziel: so weit rechts der Burg
	GateInsetUnits                      float64 // Auslöser und Banner: so weit innen vor dem äußersten Tor
	BehindUnits                         float64 // Ziele bis so weit hinter dem Anker zählen noch als voraus
	StandRangeFactor                    float64 // Angreifer stehen so viele Reichweiten (Anteil) vor dem Ziel
	FollowUnits                         float64 // Trupp ohne Ziel: Abstand zum Monarchen (Soldaten voraus, Schützen dahinter)
	ShieldArriveUnits                   float64 // so nah an der Burg gilt das Schild als geholt
}

// rawCombat ist die Rohform von `combat` (Zeiger erkennen fehlende Felder).
type rawCombat struct {
	Mode      *string `json:"mode"`
	GroupSize *int    `json:"groupSize"`
}

// settings prüft `combat` einer Welt-Datei; nil gibt die Standardwerte (leer).
func (r *rawCombat) settings() (CombatSettings, error) {
	var c CombatSettings
	if r == nil {
		return c, nil
	}
	if r.Mode != nil {
		if *r.Mode != CombatGroups && *r.Mode != CombatGeneral {
			return c, fmt.Errorf("combat.mode %q unbekannt (groups oder general)", *r.Mode)
		}
		c.Mode = *r.Mode
	}
	if r.GroupSize != nil {
		if *r.GroupSize < 1 {
			return c, errors.New("combat.groupSize muss eine ganze Zahl ≥ 1 sein")
		}
		c.GroupSize = *r.GroupSize
	}
	return c, nil
}

// chainCombat setzt die Kampfvariante aus der Kette (älteste Welt zuerst) zusammen; die Kind-Welt gewinnt feldweise.
func chainCombat(chain []WorldDef) CombatSettings {
	var out CombatSettings
	for _, w := range chain {
		if w.Combat.Mode != "" {
			out.Mode = w.Combat.Mode
		}
		if w.Combat.GroupSize > 0 {
			out.GroupSize = w.Combat.GroupSize
		}
	}
	return out
}

// combatMode ist die Kampfvariante der Stufe (fehlt sie, groups).
func combatMode(w *World) string {
	if w.values().Combat.Mode == CombatGeneral {
		return CombatGeneral
	}
	return CombatGroups
}

// groupSize sind die Soldaten je Gruppe neben dem Führer.
func groupSize(w *World) int {
	if n := w.values().Combat.GroupSize; n > 0 {
		return n
	}
	return DefaultGroupSize
}
