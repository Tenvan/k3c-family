// Package view ist der Vertrag zwischen Spielzustand und Darstellung des nativen Spiels (B-792). Er ist nach NG1
// eingefroren: Braucht ein Fachpaket mehr, gibt es ein Ticket an die NG1-Nachfolge, keinen Eingriff aus NG2–NG8.
//
// Rollen:
//   - game (Schleife) besitzt Fenster, Zellen, Menü-Stapel und die Simulation (local); nur game kennt alle Pakete.
//   - local erfüllt Session und liefert je Update einen Frame.
//   - Fachpakete (world, actors, props, input, hud, fx, menu, audio) liefern Drawer, Listener, Source oder Menus.
//
// Importregel (geprüft in game/imports_test.go): Fachpakete importieren nur view, art, k3c/engine/sim (lesend) und
// k3c/data, dazu Standardbibliothek und Ebitengine; nie einander, nie game oder local. view selbst kennt nur sim.
//
// Ablauf je Bild:
//  1. Update: game liest Source.Poll, gibt Befehle an local bzw. Input an den obersten Screen, baut den Frame und
//     ruft Listener.Listen genau einmal je Frame (Ereignisse und Cues erscheinen in genau einem Frame).
//  2. Draw: für jede Zelle die Drawer der Zellen-Ebenen (LayerFar … LayerHUD) in Ebenen-Reihenfolge, danach einmal die
//     Overlay-Drawer für das ganze Fenster, zuletzt der oberste Screen. Draw kann denselben Frame mehrmals zeichnen.
//
// Regeln für alle Drawer: *sim.World und alles im Frame nur lesen; keine Wanduhr (nur Frame.Clock), kein Zufall
// außerhalb der Simulation; Lage, Reihenfolge und Auswahl als reine Funktionen ohne ebiten.Image, mit Tests.
package view

// MaxPlayers ist die Höchstzahl lokaler Spieler (Zellen) im Split-Screen.
const MaxPlayers = 2
