// Package e2e ist der Ende-zu-Ende-Test des nativen Spiels (NG9.1, B-802/AC-01): Er treibt die echte Schleife
// (game.Game mit allen Fachpaketen und local) ohne Fenster mit einer festen Eingabe-Folge vom Hauptmenü über die Lobby
// mit zwei Geräten, das Spiel (laufen, Münzen geben, bauen, Skill lernen, Tag und Nacht mit Gegnern), Pause, Spiel
// verlassen (speichert) und Weiter bis zur Niederlage, prüft jede Station an Zustandswerten und den Welt-Hash nach
// dem Laden und vergleicht zwei Läufe. Mit gebauter EXE (Umgebung K3C_GAME_EXE, `task native:game:e2e`) nimmt er
// dieselben Stationen im Aufnahme-Modus (--flow --at) zweimal auf und vergleicht die Bilder.
//
// Nur Tests; der Ordner fällt nicht unter die Importregel der Fachpakete (game/imports_test.go).
package e2e
