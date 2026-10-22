# B-499 · Studio „Im Spiel öffnen“ in eigener Welt scheitert beim zweiten Mal mit demselben Seed

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ22
- **Projekt:** WZG
- **Erstellt:** 2026-10-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Gefunden im Review WZ18.7. `WorldOpenGame` (`engine/studio/app_game.go:102-106`) öffnet eine Welt außer `standard` mit `save=<seed>&fresh=1&world=<id>`. Beim zweiten Klick mit demselben Seed gibt es den Stand schon: `Manager.open` (`engine/room/manager.go:180`) antwortet `save_exists`, die Lobby (`src/scenes/lobbyLogic.ts`) lädt nicht nach. Umgekehrt lädt die Standard-Welt mit demselben Seed (`fresh:false`) still den Stand der anderen Welt, weil der Name des Stands die Welt nicht kennt. Kein Datenverlust (nichts wird überschrieben).

## Ziel

„Im Spiel öffnen“ startet in jeder Welt wiederholt ohne Fehlermeldung und lädt nie den Stand einer anderen Welt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Test in `engine/studio`: zweimal „Im Spiel öffnen“ in der Welt `nordmeer` mit demselben Seed liefert eine URL, die den Stand anlegt bzw. lädt (kein `save_exists`).
- **AC-02** Test: Standard-Welt und Welt `nordmeer` mit demselben Seed ergeben verschiedene Spielstand-Namen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Vorschlag aus dem Review: für Welten außer `standard` den Stand `<welt>-<seed>` nennen und `fresh=1` weglassen; der Weg `save_not_found` → `fresh:true` mit `world` legt ihn beim ersten Mal an.
