package view

import "github.com/hajimehoshi/ebiten/v2"

// StepKind sagt game, was nach Screen.Update geschieht.
type StepKind int

const (
	StepStay    StepKind = iota // Screen bleibt oben
	StepPush                    // Step.Next oben auflegen (Untermenü)
	StepReplace                 // obersten Screen durch Step.Next ersetzen
	StepPop                     // obersten Screen schließen (zurück); leerer Stapel mit aktivem Spiel = weiterspielen
	StepPlay                    // alle Screens schließen, Spiel läuft (nach Session.New, Continue oder Load)
	StepQuit                    // Programm beenden (game ruft vorher Session.Close)
)

// Step ist das Ergebnis eines Screen.Update.
type Step struct {
	Kind StepKind
	Next Screen // für StepPush und StepReplace
	Cues []Cue  // z. B. CueMove, CueConfirm; game legt sie in den nächsten Frame
}

// Screen ist ein Menü-Bildschirm (menu). Er bekommt die Eingabe aller Geräte, solange er oben liegt, und bedient die
// Session selbst (Neues Spiel, Laden, Löschen …). Gezeichnet wird er nach LayerOverlay über das ganze Fenster; im
// Pause-Menü liegt darunter das stehende Spiel des Frames.
type Screen interface {
	Update(in Input, s Session) Step
	Draw(dst *ebiten.Image, f *Frame, c Cell) // c = WindowCell
}

// Menus liefert die Einstiege des Menüs: Main beim Start und nach Spielende, Pause bei ActionMenu im Spiel. game ruft
// Session.Pause(true) beim Auflegen des Pause-Screens und Pause(false), sobald der Stapel leer ist (StepPop, StepPlay).
type Menus interface {
	Main() Screen
	Pause() Screen
}

// MenuPages ist eine optionale Rolle von Menus für den Aufnahme-Modus (--menu): Page liefert das Untermenü name
// ("saves", "options", "lobby"; sonst nil), inGame = über dem Pause-Menü, by = bedienendes Gerät. game legt es über
// Main bzw. Pause auf; im Spiel führt der Weg dorthin über die Menü-Einträge.
type MenuPages interface {
	Page(name string, inGame bool, by DeviceID) Screen
}
