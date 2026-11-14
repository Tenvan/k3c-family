package devtools

// Einträge des Cheat-Menüs (F9, B-888) in den Gruppen des alten Dialogs (src/scenes/debug/debugActions.ts): Vorrat,
// Ausbau, Zeit, Grad, Kampf. Jeder Eintrag ist eine Dev-Aktion des Raums (engine/room/dev.go); Bosse und Events kommen
// aus data/bosses.json und data/events.json. Reine Daten, ohne Ebitengine.

import (
	"encoding/json"
	"fmt"
	"log/slog"

	"k3c/data"
	"k3c/native/k3c-game/view"
)

// cheatEntry ist ein Eintrag: Beschriftung und Aktion; toggle != "" schaltet einen Schalter (On wird beim Auslösen
// gesetzt, die Beschriftung zeigt den Stand).
type cheatEntry struct {
	label  string
	act    view.DevAction
	toggle string
}

// cheatGroup ist eine Gruppe (Reiter) des Menüs.
type cheatGroup struct {
	name    string
	entries []cheatEntry
}

// Werte der Einträge (wie der alte Dialog).
var (
	goldAmounts    = []int{10, 100, 500}
	materialAmount = 50
	upgradeLevels  = []int{1, 2, 3, 4, 5}
	resources      = [][2]string{{"wood", "Holz"}, {"stone", "Stein"}, {"copper", "Kupfer"}, {"iron", "Eisen"}, {"crystal", "Kristall"}}
	phases         = [][2]string{{"day", "Tag"}, {"dusk", "Abend"}, {"night", "Nacht"}, {"dawn", "Morgen"}}
	// grades ohne dev: der Grad dev geht nur mit Dev-Mode des Raums, den das native Spiel nicht setzt (local/dev.go).
	grades = [][2]string{{"easy", "Leicht"}, {"normal", "Normal"}, {"hard", "Hart"}, {"ultra", "Ultra"}}
	events = [][2]string{{"fullMoon", "Vollmond"}, {"bloodMoon", "Blutmond"}, {"merchantRaid", "Händler-Überfall"}}
)

// cheatGroups ist das Menü (einmal gebaut).
var cheatGroups = buildGroups(loadBosses())

func act(action string) view.DevAction { return view.DevAction{Action: action} }

// buildGroups baut die Gruppen; bosses sind Id und Name der Bosse.
func buildGroups(bosses [][2]string) []cheatGroup {
	var stock, build, tm, grade, fight []cheatEntry
	for _, n := range goldAmounts {
		stock = append(stock, cheatEntry{label: fmt.Sprintf("Gold +%d", n), act: view.DevAction{Action: "gold", Amount: n}})
	}
	for _, r := range resources {
		stock = append(stock, cheatEntry{label: fmt.Sprintf("%s +%d", r[1], materialAmount),
			act: view.DevAction{Action: "material", Resource: r[0], Amount: materialAmount}})
	}
	stock = append(stock, cheatEntry{label: "Lager-Maximum +100", act: act("maxStock")},
		cheatEntry{label: "Alle Ressourcen füllen", act: act("fillStock")},
		cheatEntry{label: "Baukosten 1", act: act("cheapBuild"), toggle: "cheapBuild"})
	for _, n := range upgradeLevels {
		build = append(build, cheatEntry{label: fmt.Sprintf("Alles ausbauen: Stufe %d", n), act: view.DevAction{Action: "buildAll", Level: n}})
	}
	build = append(build, cheatEntry{label: "Burgfall", act: act("castleFall")})
	tm = append(tm, cheatEntry{label: "Nächste Tageszeit", act: act("nextPhase")})
	for _, p := range phases {
		tm = append(tm, cheatEntry{label: "Zu: " + p[1], act: view.DevAction{Action: "phase", Phase: p[0]}})
	}
	for _, f := range view.Timescales {
		tm = append(tm, cheatEntry{label: fmt.Sprintf("Zeit %d×", f), act: view.DevAction{Action: "timescale", Factor: f}})
	}
	tm = append(tm, cheatEntry{label: "Zeitraffer endet mit der Tageszeit", act: act("timescaleAuto"), toggle: "timescaleAuto"})
	for _, g := range grades {
		grade = append(grade, cheatEntry{label: "Grad " + g[1], act: view.DevAction{Action: "grade", Grade: g[0]}})
	}
	fight = append(fight, cheatEntry{label: "Nacht sofort", act: act("night")}, cheatEntry{label: "Welle jetzt", act: act("wave")})
	for _, b := range bosses {
		fight = append(fight, cheatEntry{label: "Boss " + b[1], act: view.DevAction{Action: "boss", Boss: b[0]}})
	}
	fight = append(fight, cheatEntry{label: "Boss: nächste Phase", act: act("bossPhase")})
	for _, e := range events {
		fight = append(fight, cheatEntry{label: "Event " + e[1], act: view.DevAction{Action: "event", Event: e[0]}})
	}
	return []cheatGroup{{"Vorrat", stock}, {"Ausbau", build}, {"Zeit", tm}, {"Grad", grade}, {"Kampf", fight}}
}

// loadBosses liest Id und Name der Mini- und Endbosse aus data/bosses.json (Fehler: keine Bosse, Log).
func loadBosses() [][2]string {
	var v struct {
		Bosses, EndBosses []struct{ ID, Name string }
	}
	raw, err := data.Files.ReadFile("bosses.json")
	if err == nil {
		err = json.Unmarshal(raw, &v)
	}
	if err != nil {
		slog.Warn("💥 Bosse für das Cheat-Menü nicht lesbar", "err", err)
		return nil
	}
	out := make([][2]string, 0, len(v.Bosses)+len(v.EndBosses))
	for _, b := range append(v.Bosses, v.EndBosses...) {
		out = append(out, [2]string{b.ID, b.Name})
	}
	return out
}
