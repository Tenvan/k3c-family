package serverapi

import (
	"fmt"
	"slices"
	"strings"
)

// FormatStatus ist die Antwort von server_status: Version, Laufzeit, Zähler, Räume, letzte Abstürze.
func FormatStatus(st Status) string {
	lines := []string{
		fmt.Sprintf("Server %s · läuft seit %s · %d Räume · %d Spielstände · %d Berichte",
			st.Version, uptime(st.UptimeS), len(st.Rooms), st.Saves, st.Reports),
	}
	for _, f := range st.Failures {
		lines = append(lines, fmt.Sprintf("Absturz %s (%s) %s: %s", f.Code, f.Name, f.At, f.Error))
	}
	if len(st.Failures) == 0 {
		lines = append(lines, "Abstürze: keine")
	}
	return strings.Join(lines, "\n")
}

// FormatRooms ist die Antwort von rooms_list: eine Zeile je Raum.
func FormatRooms(rooms []Room) string {
	if len(rooms) == 0 {
		return "Keine Räume"
	}
	lines := make([]string, 0, len(rooms))
	for _, r := range rooms {
		state := "pausiert"
		if r.Running {
			state = "läuft"
		}
		lines = append(lines, fmt.Sprintf("%s · %s · Tiefe %d · %d Geräte · Monarchen %s · Tick %d · Tick-Dauer %.2f ms (p99 %.2f ms)",
			r.Code, state, r.Depth, r.Devices, strings.Join(r.Monarchs, "/"), r.Tick, r.TickMs.Last, r.TickMs.P99))
	}
	return strings.Join(lines, "\n")
}

// FormatSummary ist die Antwort von room_snapshot.
func FormatSummary(s Summary) string {
	troops := make([]string, 0, len(s.Troops))
	for kind, n := range s.Troops {
		troops = append(troops, fmt.Sprintf("%s %d", kind, n))
	}
	slices.Sort(troops) // Map-Reihenfolge ist zufällig
	if len(troops) == 0 {
		troops = append(troops, "keine")
	}
	gold := make([]string, len(s.Gold))
	for i, g := range s.Gold {
		gold[i] = fmt.Sprint(g)
	}
	return fmt.Sprintf("Raum %s · Tiefe %d · Tick %d · Tag %d (%s) · Welle %d\nGold je Monarch: %s\nTruppen: %s · Gegner %d · Burg %.0f HP",
		s.Code, s.Depth, s.Tick, s.Day, s.Phase, s.Wave, strings.Join(gold, ", "), strings.Join(troops, ", "), s.Enemies, s.Castle)
}

func uptime(seconds int64) string {
	switch {
	case seconds < 60:
		return "< 1 min"
	case seconds < 3600:
		return fmt.Sprintf("%d min", seconds/60)
	default:
		return fmt.Sprintf("%d h %d min", seconds/3600, seconds%3600/60)
	}
}
