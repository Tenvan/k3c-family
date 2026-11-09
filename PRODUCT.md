# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

Familien am Fernseher: Couch-Koop mit einem bis vier Spielern, gemischtes Alter (auch Kinder), Xbox-Controller, Sofa-Abstand zum TV (Edge auf der Xbox). Zweite Gruppe ist der Entwickler am PC, der Werkzeug- und Testseiten nutzt. Der Server läuft im Heimnetz (Raspberry Pi 3).

## Product Purpose

K3C (Family Three Crowns) ist ein Couch-Koop-Side-Scroller im Stil von Kingdom Two Crowns: Monarch zu Pferd, Münzen sammeln, Gebäude bauen, Truppen anwerben, nachts Wellen abwehren. Erfolg heißt: Die Familie startet ohne Anleitung per Pad ein Spiel und bleibt flüssig und stabil im Vollbild.

## Positioning

Ein Heimnetz-Spiel für die ganze Familie, das auf der Xbox im Browser läuft (Papierschnitt-Grafik, Split-Screen, Go-Server), ohne Store und ohne Konto.

## Operating Context

Alle Bedienung per Gamepad (Standard-Mapping). B ist nie belegt (Edge-Zurück), View + Menu gemeinsam gehen zur Startseite. Die Landingpage bleibt dauerhaft offen und zeigt Seiten im Vollflächen-iframe, damit der Vollbildmodus über Seitenwechsel erhalten bleibt. Nach B-768 ist das Pausenmenü der Einstiegspunkt: Neues Spiel, Fortsetzen, Raum beitreten, Optionen und der Weg zu allen anderen Seiten.

## Capabilities and Constraints

Phaser 4, TypeScript, Vite; Spiel-Logik nur im Go-Server. Texte kommen aus `src/core/texts.de.ts` / `texts.en.ts` (Deutsch und Englisch). Logging mit Emojis, keine Sprünge im Spiel, jede Mechanik mit zwei Spielern gleichzeitig. Tickets, Sprints und Sessions nur über die Workbench-MCP-Tools.

## Brand Commitments

Name „Family Three Crowns“ (Kurzform K3C). Verbindlich für neue Oberflächen: das Nachtblau/Gold der Landingpage (`index.html`: Nacht-Verlauf #0b1026 bis #3b3a6b, Gold #ffd166, Glas-Kacheln, grüner A-Knopf). Weitere Farbgruppen nach Bereich sind erlaubt.

## Evidence on Hand

Landingpage `index.html` und `src/landing/`, Pausen- und Optionsmenü `src/scenes/OptionsScene.ts`, Lobby `src/scenes/LobbyScene.ts`, Papierschnitt-Grafik unter `public/grafik/` (nicht eingecheckt, `task gen:paper`). Keine Nutzerzahlen oder Rezensionen; nichts davon erfinden.

## Product Principles

- Pad zuerst: jede Funktion ist ohne Maus und Tastatur erreichbar.
- Aus dem Sofa lesbar: große Schrift, hoher Kontrast, nichts Wichtiges nur klein.
- Farbe ist nie die einzige Information (Farbenblind-Symbole).
- Ein Einstieg, wenige Ebenen: Fortsetzen ist immer mit einem Druck erreichbar.
- Der Vollbildmodus geht nicht verloren.

## Accessibility & Inclusion

Lesbar aus Sofa-Abstand und vollständig mit dem Pad bedienbar sind Pflicht. Farbenblind-Symbole sind vorhanden; Farbe allein trägt keine Information.
