# B-1214 · Workbench: Spielstart (Dev und Release) als Dienste eintragen und loggen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WK1
- **Projekt:** NAC
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Workbench kennt Dienste (`svc_start`/`svc_stop`, Logs `logs_*`), aber den Start des Rust-Spiels in Dev und Release gibt es dort nicht als Dienst (Meldung Ralf 2026-11-21). Das Spiel startet über `task rust:game:run`/`rust:game:dev` bzw. `dist/k3c-game/k3c-game.exe`.

## Ziel

Dev- und Release-Start des Spiels sind Dienste der Workbench (Start, Stopp, Status) und ihre Ausgabe landet in den Workbench-Logs.

## Beteiligte und Zielgruppen

Ralf (Entwickler); Hauptrechner.

## Anforderungen

- **AC-01** Zwei Dienste `game-dev` (Task `rust:game:dev`) und `game-release` (`dist/k3c-game/k3c-game.exe`), start-/stoppbar in der Oberfläche und per MCP `svc_*`.
- **AC-02** Stdout/Stderr der Dienste landen im Log-Speicher (`logs_query`, Quelle je Dienst, Emoji-Meldungen unverändert).
- **AC-03** Sauberer Stopp (Job-Objekt), laufende EXE blockiert keinen Neubau mehr (Hinweis statt Fehler).

## Nicht-Ziele

Spielstand-Steuerung aus der Workbench (RU34).

## Regeln und Einschränkungen

Entscheidung 004 (Schnittstelle sind Dateien/Prozesse); Atomic Design im Frontend.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `tools/k3c-workbench/**`, `engine/devrun/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** exe

## Beispiele

`svc_start game-dev`: Spiel startet, Log zeigt 🚀-Zeilen; `svc_stop`: beendet.

## Ausnahme- und Fehlerfälle

Release-EXE fehlt: Dienst meldet „erst `task build`“.

## Akzeptanzkriterien

- **AC-01** Test der Dienst-Definitionen und Start/Stopp mit Platzhalter-Prozess.
- **AC-02** Test: Ausgabe erscheint im Log mit Quelle.
- **AC-03** Test des Stopps.

## Offene Fragen

keine.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
