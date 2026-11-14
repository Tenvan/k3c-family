package view

import "k3c/engine/sim"

// Optionale Rollen eines Fachpakets. game prüft jeden Beitrag eines Pakets (Drawer, Listener, Source, Menus) per
// Typ-Zusicherung und meldet ihn für jede Rolle an, die er erfüllt; ein Paket braucht dafür keinen Eingriff in game.
// CameraRule gibt es höchstens einmal (world), AssetSource und Overlay beliebig oft.

// CameraRule führt die Kamera einer Zelle (world, B-795: Totzone, Grenzen der Stufe). game ruft Follow im Update
// genau einmal je Zelle, nachdem der Frame gebaut ist, nie in Draw; ein Zustand in der Regel ist damit erlaubt.
// c ist die Zelle mit Lage und Maßstab dieses Updates, ihre Kamera steht noch auf prev.X. prev ist die Kamera desselben
// Spielers (Seat.Player) aus dem vorigen Update; prev.Scale == 0 heißt: keine (erstes Bild, Spieler neu), die Regel
// springt dann direkt ans Ziel. game übernimmt das Ergebnis als c.Camera (ein NaN- oder unendliches X verwirft es).
// Der Szenen-Schalter --cam setzt X danach fest. Ohne Regel folgt die Kamera dem Monarchen, ohne Monarchen der Burg.
type CameraRule interface {
	Follow(f *Frame, c Cell, prev Camera) Camera
}

// AssetSource meldet die Papier-Assets (`gruppe/id`, wie art.PlanLevel), die ein Fachpaket in einem Level braucht.
// game fragt alle Quellen, sobald ein Level in Frame.Stages neu auftaucht (w ist dessen Welt, nur lesen), lädt die
// Vereinigung über art.Library.SetLevels und gibt frei, was kein Level in Frame.Stages mehr braucht. Die Antwort
// muss alles nennen, was das Paket im Level zeichnen kann (nicht nur das gerade Sichtbare); fehlt eine Datei, beendet
// sich das Spiel mit dem Pfad (art.ErrMissing).
type AssetSource interface {
	Assets(level string, w *sim.World) []string
}

// Overlay ist ein Menü im laufenden Spiel je Spieler (hud: Skill-Menü, B-799). game ruft Update in jedem Update im
// Spiel (kein Screen offen) vor Pause, Beitritt und Befehlen, mit der Eingabe aller Geräte und dem Frame des vorigen
// Updates (Seats ordnet Geräte den Spielern zu). Das Overlay bedient die Session selbst (Learn, Respec).
type Overlay interface {
	Update(in Input, f *Frame, s Session) OverlayStep
}

// OverlayStep ist das Ergebnis eines Overlay.Update. Für einen verbrauchten Spieler gibt game in diesem Update einen
// leeren Befehl (Monarch steht) und wertet die Aktionen seines Geräts nicht aus (kein Pause-Menü, kein ActionHome).
type OverlayStep struct {
	Consumed [MaxPlayers]bool // Index = Seat.Player
	Cues     []Cue            // z. B. CueMove, CueConfirm, CueDenied; game legt sie in den nächsten Frame
}
