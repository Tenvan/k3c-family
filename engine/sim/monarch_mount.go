package sim

// MountOf liefert das Reittier des Monarchen p (docs/rules/monarch.md § 7, B-152). Heute reitet jeder das
// Standard-Reittier aus den Daten (kein Feld in Player, Snapshot oder Spielstand); ein abweichendes Reittier käme hier hinzu.
func MountOf(_ *Player) MountData { return monarch.Mount }
