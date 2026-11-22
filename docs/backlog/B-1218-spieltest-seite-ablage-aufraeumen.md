# B-1218 · Workbench-Seite Spieltest listet Aufnahmen in playtests/ und räumt sie per Knopf auf

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WZ64
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Chat 2026-11-22
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Feedback aus Spieltests am Sofa geht heute verloren: Niemand schreibt mit, während gespielt wird. Die Workbench (`tools/k3c-workbench`) hat keine Seite für Spieltests; Seiten hängen in `frontend/src/Header.tsx` (`PAGES`) und `frontend/src/App.tsx` (`Tabs.Content`), Go-API je Thema in `app_*.go`. Ablage-Muster wie `saves/` und `reports/` (im Repo, gitignored) gibt es schon.

## Ziel

Eine eigene Seite **Spieltest** im Bereich Development zeigt alle Spieltest-Sessions mit Datum, Dauer, Dateien und Stand (aufgenommen, transkribiert, ausgewertet) und räumt sie per Knopf auf. Sie ist das Gerüst für Aufnahme (B-1219) und Auswertung (WZ65).

## Beteiligte und Zielgruppen

Ralf (🧑) testet mit Familie am TV und wertet aus; Claude wertet über MCP aus (WZ65).

## Anforderungen

- Neue Seite `spieltest` im Bereich Development nach `docs/standards/workbench-seiten.md`, Oberfläche nur über `@webkit` (Atomic Design).
- Ablage je Session in `playtests/<JJJJ-MM-TT>-<nr>/` im Repo; `/playtests/` steht in `.gitignore`.
- Ordner-Vertrag (gilt für WZ64–WZ66): `session.json` (Start/Ende als Unix-ms, Mikrofon, Monitor, ffmpeg-Aufruf, Spiel-Argumente, Stand), `voice.wav`, `video.mkv`, `markers.jsonl` (schreibt das Spiel, B-1220), `survey.json` (B-1222), `transcript.json`, `frames/`, `findings.json` (WZ65). Zeitstempel überall Unix-ms der Systemuhr.
- Liste: je Session Datum, Dauer, Anzahl Marker, Größe, Stand; Detailansicht mit Audio- und Video-Player.
- Aufräum-Knöpfe: „Video löschen“ und „Session löschen“ je Session, „Videos ausgewerteter Sessions löschen“ für alle; jeder Knopf zeigt vorher, was gelöscht wird, und verlangt Bestätigung.
- Go-Paket `internal/playtest` (Ablage, Liste, Löschen) mit Unit-Tests; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Nicht-Ziele

- Aufnahme selbst (B-1219), Transkript und Auswertung (WZ65), Spielseite (WZ66).
- Automatisches Löschen nach Frist: Aufräumen nur per Knopf (Entscheidung 🧑 2026-11-22).

## Regeln und Einschränkungen

- Schreiben nur im Repo; alles bleibt lokal (Familienstimmen), kein Upload.
- Logging mit Emoji (🧹 aufgeräumt, 📂 geladen).

- **Bereich:** `tools/k3c-workbench/internal/playtest/**`, `tools/k3c-workbench/app_playtest.go`, `tools/k3c-workbench/frontend/src/playtest/**`, `tools/k3c-workbench/frontend/src/Header.tsx`, `tools/k3c-workbench/frontend/src/App.tsx`, `.gitignore`
- **Abhängig von:** WZ63
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

- Drei Sessions liegen in `playtests/` → die Seite listet sie neuste zuerst mit Stand „aufgenommen“.
- Ralf klickt bei einer ausgewerteten Session „Video löschen“ → Dialog nennt Datei und Größe, nach Bestätigung fehlt nur `video.mkv`, Transkript und Befunde bleiben.

## Ausnahme- und Fehlerfälle

- `playtests/` fehlt → Seite zeigt leere Liste, Ordner entsteht erst bei der ersten Aufnahme.
- `session.json` kaputt oder fehlt → Session erscheint mit Stand „unvollständig“, Löschen bleibt möglich.
- Löschen einer laufenden Aufnahme → Knopf gesperrt.

## Akzeptanzkriterien

- **AC-01** Die Seite Spieltest erscheint im Bereich Development und listet die Sessions aus `playtests/` (Test mit Beispielordnern).
- **AC-02** `git check-ignore playtests/x` meldet den Pfad als ignoriert.
- **AC-03** Die drei Aufräum-Knöpfe löschen genau die genannten Dateien erst nach Bestätigung (Go-Unit-Test für `internal/playtest`, Oberfläche im Browser geprüft).
- **AC-04** `check_run task:check:diff` grün.

## Offene Fragen

keine

## Notizen

Entstanden aus /promptize mit Grilling, 🧑 2026-11-22.
