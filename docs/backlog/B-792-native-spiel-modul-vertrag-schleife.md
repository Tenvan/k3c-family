# B-792 · Das native Spiel hat ein eigenes Go-Modul mit Spielschleife, Grafik-Bibliothek und festem Zeichen-Vertrag

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG1
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Es gibt zwei Prototypen: `native/k3c-ebiten` (Level aus Sheets, Fluss, feste Schicht `fern`, Rig-Figuren per `--art rig|sheet`, alles flach in `package main`) und `native/k3c-rig` (Rig-Format `RIG.md`, Pakete `rig`, `paint`, `export`, Teile unter `assets/`). Ein Spiel mit Simulation, Menüs und Spielständen gibt es nativ noch nicht. Mehrere Sprints sollen parallel daran bauen; dafür braucht es getrennte Paket-Ordner und einen festen Vertrag zwischen Spielzustand und Zeichnen.

## Ziel

Ein eigenes Go-Modul `native/k3c-game` mit Spielschleife, Grafik-Bibliothek und einem Vertrag (`view`), an den die Fach-Sprints (Welt, Figuren, Bauten, Eingabe, HUD, Menüs, Ton) ihre Pakete hängen, ohne einander oder die Schleife zu ändern.

## Beteiligte und Zielgruppen

Claude (Umsetzung, die Fach-Sprints NG2–NG8 bauen darauf), 🧑 Ralf (Freigabe, Entscheidung über Ort und Abhängigkeiten).

## Anforderungen

- **Ort:** eigenes Go-Modul `k3c/native/k3c-game` unter `native/k3c-game/`, `replace k3c => ../..` und `replace k3c/native/k3c-rig => ../k3c-rig`; Ebitengine in derselben Version wie `native/k3c-ebiten/go.mod` (v2.10.6), ohne cgo.
- **Pakete:** `game/` (Schleife, Zustände Menü ↔ Spiel, Zusammenbau), `view/` (Vertrag), `local/` (Simulation im Prozess, B-793), `art/` (Grafik, Schrift), dazu je ein Paket mit Platzhalter für `world/`, `actors/`, `props/`, `input/`, `hud/`, `fx/`, `menu/`, `audio/`.
- **Vertrag `view`:** `Frame` (Takt, Stufe, `*sim.World` nur lesend, Plätze Spieler ↔ Monarch ↔ Zelle, Pause, Ereignisse des Takts), `Cell` (Rechteck, Spieler, Kamera; Welt in den oberen zwei Dritteln, Fluss im unteren Drittel wie NV3), `Camera`, `Drawer` mit fester Ebene (fern, mitte, nah, Boden, Bauten, Figuren, Fluss, Effekte, HUD je Zelle, Overlay je Fenster), Eingabe-Typen (Befehle je Spieler, Menü-Aktionen), `Screen` für Menüs und `Session` (Neues Spiel, Weiter, Laden, Liste, Löschen) als Schnittstelle, die `local` erfüllt.
- **Importregel:** Fachpakete importieren nur `view`, `art`, `k3c/engine/sim` (lesend) und `k3c/data`, nie einander; nur `game` kennt alle. Ein Test prüft das.
- **Rechnen getrennt vom Zeichnen:** Lage, Reihenfolge und Auswahl sind reine Funktionen ohne `ebiten.Image`, damit sie offline testbar sind.
- **Grafik `art`:** Sheets je Level aus `public/grafik` über `data/paper.json`, Rig-Bibliothek über `native/k3c-rig/paint`, eine Schrift für HUD und Menüs, Texturbudget im Log.
- **Schleife:** 1 oder 2 Zellen im Split-Screen, Platzhalter für alles aus dem Frame, große FPS-Anzeige.

## Nicht-Ziele

Fertige Darstellung einzelner Bereiche (NG2–NG8); Online-Spiel, Browser, Werkzeug-Seiten, Diagnose-Anzeige außer FPS; Änderungen unter `engine/` oder an den Prototypen.

## Regeln und Einschränkungen

Entscheidung 005 und 008, Rig-Format `native/k3c-rig/RIG.md`, Komplexitäts-Budget (Datei ≤ 400, Funktion ≤ 60 Zeilen), Logging mit Emojis (`CLAUDE.md`). Kein `math/rand`; Zufall nur aus der Simulation. Neue Module nur mit Freigabe 🧑 (Sperre `deps`): Alles, was Ebitengine für Text und Ton braucht, legt dieser Sprint fest, damit NG2–NG8 `go.mod` nicht ändern.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `native/k3c-game/main.go`, `native/k3c-game/go.mod`, `native/k3c-game/go.sum`, `native/k3c-game/game/**`, `native/k3c-game/view/**`, `native/k3c-game/art/**`, `native/k3c-game/world/**`, `native/k3c-game/actors/**`, `native/k3c-game/props/**`, `native/k3c-game/input/**`, `native/k3c-game/hud/**`, `native/k3c-game/fx/**`, `native/k3c-game/menu/**`, `native/k3c-game/audio/**`, `native/Taskfile.yml`
- **Abhängig von:** NV3
- **Braucht 🧑:** Freigabe, Entscheidung
- **Größe:** M
- **Sperre:** deps, taskfile

## Beispiele

`task native:game` öffnet ein Fenster, zwei Gamepads verbunden → zwei Zellen übereinander, je Zelle farbige Platzhalter für Monarch, Gebäude und Gegner an den Positionen der Simulation, der Fluss als Fläche im unteren Drittel, FPS groß oben.

## Ausnahme- und Fehlerfälle

Grafik fehlt (`gen:paper` nicht gelaufen, Rig-Teil fehlt) → Meldung mit Pfad beim Start und Abbruch, kein leeres Bild. Ein Fachpaket braucht mehr im Vertrag → Ticket an NG1-Nachfolge, kein Eingriff in `view` aus NG2–NG8.

## Akzeptanzkriterien

- **AC-01** `native/k3c-game` baut ohne cgo mit `task native:game:build`; `go.mod` pinnt Ebitengine wie `native/k3c-ebiten` und bindet `k3c` und `k3c-rig` per `replace`.
- **AC-02** Paket `view` enthält Frame, Cell, Camera, Drawer mit fester Ebenen-Reihenfolge, Eingabe-Typen, Screen und Session; `game` registriert je Fachpaket einen Platzhalter; ein Test prüft die Importregel.
- **AC-03** `art` lädt Sheets je Level, die Rig-Bibliothek und eine Schrift; fehlende Grafik bricht mit Meldung ab (Test); das Texturbudget steht im Log.
- **AC-04** Das Fenster zeigt 1 oder 2 Zellen mit Platzhaltern für alle Figuren und Gebäude aus dem Frame und eine FPS-Anzeige; Lage der Zellen und Kamera-Ausschnitt sind offline getestet.

## Offene Fragen

- **Ort und Weg:** neues Modul `native/k3c-game` statt Ausbau von `native/k3c-ebiten`. Vorschlag: neu, weil der Prototyp flach in `package main` liegt und parallele Sprints getrennte Ordner brauchen; der Prototyp bleibt Mess-Referenz, seine Bausteine (`river.go`, `paper.go`, `fps.go`) werden übernommen. Alternative `cmd/k3c-native` im Wurzel-Modul verworfen: Ebitengine käme in das `go.mod` des Servers, und Linux-CI und Docker müssten es übersetzen (vermutet: Ebitengine braucht unter Linux cgo und X11-Header). Entscheidet 🧑.
- **Abhängigkeiten:** Für Text (`ebiten/v2/text/v2`) und Ton (`ebiten/v2/audio/vorbis`) zieht Ebitengine weitere Module nach (vermutet: `go-text/typesetting`, `jfreymuth/oggvorbis`, `hajimehoshi/go-mp3`). Vorschlag: zulassen, nur was Ebitengine selbst verlangt, keine weiteren. Entscheidet 🧑.
- **Schrift:** Im Repo liegt keine Schriftdatei (der Browser nutzt CSS-Schriften). Vorschlag: eine freie Schrift (OFL) über `assets_*` mit Lizenz-Eintrag. Entscheidet 🧑.
- **Rig-Paket:** Das Spiel bindet `native/k3c-rig/rig` und `paint` aus dem Prototyp-Modul ein. Vorschlag: so lassen, Umzug in ein festes Modul erst als eigenes Ticket nach NG9.

## Notizen

Angelegt 2026-11-11 bei der Planung der Umsetzung (Projekt NAT, Sprints NG1–NG11). Gesamt-Spec B-775.
