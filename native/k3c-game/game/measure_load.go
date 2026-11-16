package game

// Last-Szene des Messmodus (RU0.1, B-1019): --measure-load <ausbau>,<truppen>,<gegner> baut vor der Messung die Welt
// aus und füllt sie über die Dev-Aktionen des Raums (wie der Stresstest, stress.go): Ausbaustufe für alle Bauplätze,
// eigene Truppen als Mischung aus Kämpfern, Schützen und Bürgern, Gegner als Mischung zweier Sorten. Die Spieler reiten
// über --input durch die Landschaft. Die Simulation selbst bleibt unverändert.

import (
	"fmt"
	"log/slog"
	"strconv"
	"strings"

	"k3c/native/k3c-game/view"
)

// MeasureLoad sind die Mengen der Last-Szene; alle 0 = keine.
type MeasureLoad struct {
	Build   int // Ausbaustufe aller Bauplätze (Dev-Aktion buildAll)
	Troops  int // eigene Einheiten insgesamt
	Enemies int // Gegner insgesamt
}

// loadMix verteilt die Truppen und die Gegner auf Sorten (Anteile in Prozent).
var (
	loadTroops  = []loadShare{{"warrior", 35}, {"archer", 25}, {"peasant", 25}, {"vagrant", 15}}
	loadEnemies = []loadShare{{"goblin", 60}, {"skeleton", 40}}
)

type loadShare struct {
	unit    string
	percent int
}

// parseMeasureLoad liest „ausbau,truppen,gegner“ (drei Zahlen ab 0); leer = keine Last-Szene.
func parseMeasureLoad(s string) (MeasureLoad, error) {
	var l MeasureLoad
	if s == "" {
		return l, nil
	}
	parts := strings.Split(s, ",")
	dst := []*int{&l.Build, &l.Troops, &l.Enemies}
	if len(parts) != len(dst) {
		return l, fmt.Errorf("🚫 --measure-load %q: drei Zahlen ausbau,truppen,gegner", s)
	}
	for i, p := range parts {
		n, err := strconv.Atoi(p)
		if err != nil || n < 0 {
			return l, fmt.Errorf("🚫 --measure-load %q: drei Zahlen ab 0", s)
		}
		*dst[i] = n
	}
	return l, nil
}

// loadRefillTicks ist der Abstand, in dem die Last-Szene die Gegner prüft und auffüllt (5 Sekunden).
const loadRefillTicks = 150

// refillMeasureLoad hält die Last: Lebt weniger als die Hälfte der Gegner, kommt die Hälfte neu dazu, an der Burg
// (beim Spieler entstanden, rauben sie sofort die Krone und das Spiel endet).
func (g *Game) refillMeasureLoad() {
	l := g.load
	if l.Enemies == 0 || g.frame.Tick%loadRefillTicks != 0 || len(g.frame.Seats) == 0 {
		return
	}
	ds, ok := g.sim.(view.DevSession)
	if _, alive := worldState(&g.frame); !ok || alive >= l.Enemies/2 {
		return
	}
	for _, s := range loadEnemies {
		if n := l.Enemies / 2 * s.percent / 100; n > 0 {
			if err := ds.Dev(g.frame.Seats[0].Player, view.DevAction{Action: "spawnCastle", Kind: "enemy", Unit: s.unit, Amount: n}); err != nil {
				slog.Warn("🚫 Last-Szene: Gegner nicht aufgefüllt", "err", err)
			}
		}
	}
}

// measureLoad liest die Angabe s und führt die Last-Szene aus (braucht --tick ab 1).
func (g *Game) measureLoad(s string) error {
	l, err := parseMeasureLoad(s)
	if err != nil {
		return err
	}
	return g.applyMeasureLoad(l)
}

// applyMeasureLoad führt die Last-Szene aus (nach dem ersten Takt, wenn die Plätze der Spieler stehen).
func (g *Game) applyMeasureLoad(l MeasureLoad) error {
	if l == (MeasureLoad{}) {
		return nil
	}
	ds, ok := g.sim.(view.DevSession)
	if !ok || len(g.frame.Seats) == 0 {
		return errNoDev
	}
	p := g.frame.Seats[0].Player
	dev := func(a view.DevAction) {
		if err := ds.Dev(p, a); err != nil {
			slog.Warn("🚫 Last-Szene: Dev-Aktion abgelehnt", "aktion", a.Action, "einheit", a.Unit, "err", err)
		}
	}
	if l.Build > 0 {
		dev(view.DevAction{Action: "buildAll", Level: l.Build})
	}
	if l.Enemies > 0 {
		dev(view.DevAction{Action: "night"}) // Gegner halten sich nur nachts
	}
	g.load = l
	for _, s := range loadTroops {
		if n := l.Troops * s.percent / 100; n > 0 {
			dev(view.DevAction{Action: "spawn", Kind: "troop", Unit: s.unit, Amount: n})
		}
	}
	for _, s := range loadEnemies {
		if n := l.Enemies * s.percent / 100; n > 0 {
			dev(view.DevAction{Action: "spawnCastle", Kind: "enemy", Unit: s.unit, Amount: n})
		}
	}
	slog.Info("🐛 Last-Szene", "ausbau", l.Build, "truppen", l.Troops, "gegner", l.Enemies)
	return nil
}
