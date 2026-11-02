package sim

// MountOf liefert das Reittier des Monarchen p in der Stufe w (docs/rules/monarch.md § 7, B-152). Heute reitet jeder das
// Standard-Reittier aus den Daten seiner Welt (kein Feld in Player, Snapshot oder Spielstand); ein abweichendes Reittier käme hier hinzu.
func MountOf(w *World, _ *Player) MountData { return w.values().Monarch.Mount }
