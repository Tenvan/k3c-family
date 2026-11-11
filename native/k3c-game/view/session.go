package view

import "time"

// NewGame sind die Angaben für ein neues Spiel (wie room.Options; leer = Standard).
type NewGame struct {
	Name    string     // Name des Spielstands (Slot); leer = local wählt einen freien
	Seed    string     // leer = Name als Seed
	Grade   string     // Schwierigkeitsgrad
	Goal    string     // Ziel der Insel
	Defeat  string     // Niederlage-Regel
	World   string     // Welt-ID (wie room.Options.World); leer = Standard-Welt
	Depth   int        // Tiefe der Stufe, in der die Spieler beginnen (0 = Oberfläche; Aufnahme-Modus, Tests)
	Devices []DeviceID // Geräte der Spieler 1 … MaxPlayers in dieser Reihenfolge, mindestens eins
}

// SaveInfo ist ein Eintrag der Spielstand-Liste.
type SaveInfo struct {
	Name     string    // Slot, Schlüssel für Load und Delete
	SavedAt  time.Time // Zeitpunkt des letzten Speicherns (Null-Wert, wenn unbekannt)
	Day      int       // Spieltag
	Depths   []int     // Tiefen der Spieler, sortiert, jede einmal
	Loadable bool      // false: kaputt oder alte Version; dann steht der Grund in Problem
	Problem  string    // deutsch, für die Anzeige
}

// Session ist das Spiel aus Sicht der Menüs; local erfüllt sie. Alle Methoden laufen in der Goroutine der Schleife
// und blockieren nur kurz (Dateizugriff). Fehler sind deutsch und anzeigbar.
type Session interface {
	New(g NewGame) error // neues Spiel anlegen und starten (ersetzt ein laufendes nach dessen Speichern)
	// Continue und Load starten mit den Geräten devices (Spieler 1 … in dieser Reihenfolge); leer = Geräte des letzten
	// Spiels, ohne letztes Spiel die linke Tastatur.
	Continue(devices []DeviceID) error          // zuletzt gespeicherten Spielstand laden und starten
	Load(name string, devices []DeviceID) error // Spielstand laden und starten
	List() ([]SaveInfo, error)
	Delete(name string) error // nicht den Spielstand des laufenden Spiels
	Save() error              // laufendes Spiel jetzt speichern (zusätzlich zum Autosave)
	Close() error             // speichern und Spiel beenden; danach Active() == false
	Active() bool             // es läuft ein Spiel (auch pausiert)
	Pause(on bool)            // Simulation anhalten bzw. weiterlaufen lassen
	Join(d DeviceID) (player int, err error)
	Leave(player int) error
}
