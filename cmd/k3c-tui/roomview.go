package main

import (
	"fmt"
	"maps"
	"slices"
	"strings"
)

// roomView hält, was renderRoom braucht.
type roomView struct {
	d    *roomDetail
	code string
	sel  int
	// confirmID ist die Kennung des Geräts, das getrennt werden soll (leer: keine Rückfrage offen).
	confirmID string
	err       string
	width     int
}

// renderRoom ist die Raumansicht: Zusammenfassung und Liste der Geräte mit Auswahl.
func renderRoom(v roomView) string {
	var lines []string
	if v.err != "" {
		lines = append(lines, styleError.Render(v.err)+styleDim.Render(" · neuer Versuch in 2 s"))
	}
	if v.d == nil {
		lines = append(lines, styleDim.Render("Lade Raum "+v.code+" …"))
		return clip(lines, v.width)
	}
	d := v.d
	lines = append(lines,
		styleHead.Render("Raum "+d.Code)+fmt.Sprintf(" · Tiefe %d · Tag %d (%s) · Welle %d · Tick %d", d.Depth, d.Day, d.Phase, d.Wave, d.Tick),
		"Gold je Monarch: "+goldList(d.Gold),
		"Stufen: "+stageList(d.Stages),
		fmt.Sprintf("Truppen: %s · Gegner %d · Burg %.0f HP", troopList(d.Troops), d.Enemies, d.Castle),
		"")
	lines = append(lines, deviceLines(v)...)
	return clip(lines, v.width)
}

func deviceLines(v roomView) []string {
	if len(v.d.Devices) == 0 {
		return []string{styleDim.Render("Keine Geräte")}
	}
	lines := []string{styleHead.Render(fmt.Sprintf("  %-8s %-11s %s", "Gerät", "Zustand", "Slots"))}
	for i, dev := range v.d.Devices {
		state, mark := "getrennt", "  "
		if dev.Connected {
			state = "verbunden"
		}
		if i == v.sel {
			mark = "▶ "
		}
		lines = append(lines, fmt.Sprintf("%s%-8s %-11s %v", mark, dev.ID, state, dev.Slots))
	}
	if v.confirmID != "" {
		lines = append(lines, "", styleError.Render(fmt.Sprintf("Gerät %s trennen? (y/n)", v.confirmID)))
	}
	return lines
}

// stageList schreibt je Stufe Tiefe, Tag, Phase und Spieler („T0 Tag 2 day: 0,1 · T1 Tag 1 night: –“).
func stageList(stages []stage) string {
	if len(stages) == 0 {
		return "–"
	}
	parts := make([]string, len(stages))
	for i, s := range stages {
		players := make([]string, len(s.Players))
		for j, p := range s.Players {
			players[j] = fmt.Sprint(p)
		}
		list := strings.Join(players, ",")
		if list == "" {
			list = "–"
		}
		parts[i] = fmt.Sprintf("T%d Tag %d %s: %s", s.Depth, s.Day, s.Phase, list)
	}
	return strings.Join(parts, " · ")
}

func goldList(gold []int) string {
	if len(gold) == 0 {
		return "–"
	}
	parts := make([]string, len(gold))
	for i, g := range gold {
		parts[i] = fmt.Sprint(g)
	}
	return strings.Join(parts, ", ")
}

// troopList schreibt die Truppen sortiert („archer 1, peasant 3“).
func troopList(troops map[string]int) string {
	if len(troops) == 0 {
		return "keine"
	}
	parts := make([]string, 0, len(troops))
	for _, kind := range slices.Sorted(maps.Keys(troops)) {
		parts = append(parts, fmt.Sprintf("%s %d", kind, troops[kind]))
	}
	return strings.Join(parts, ", ")
}
