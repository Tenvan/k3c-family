# B-514 · Chat-Entwurf: Planung bei gleicher Insel-id schützen, CLI-Prozessbaum, Test-Riegel

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-20
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Befunde des Reviews WZ19.4 (nicht schwer, daher Ticket):
1. `tools/k3c-studio/backend/app_world_answer.go:83-107` setzt bei Chat-Entwürfen `From` leer. Bringt die Antwort eine Insel mit schon vorhandener id ohne `plannedChunks`/`chunkCounts` mit, entsteht sie beim Speichern aus der Vorlage neu und die Planung der Datei geht verloren (Kommentar in `app_world_draft.go:33-34` gilt nur für Inseln mit `From`). Gemildert: Speichern ist eine Aktion von 🧑 mit Vorschau, „ersetzt alle Inseln“ ist in WZ19.3 dokumentiert.
2. `chat_cli.go:80,87`: Abbruch beendet nur den direkten Prozess, nicht den Prozessbaum (heute unkritisch: native EXE, keine Werkzeuge).
3. `chat_http.go:37`: kein struktureller Riegel, dass künftige Tests nie den echten `claude` starten (`TestMain` könnte `defaultCLI` sperren).
4. Frage: `tools/k3c-studio/server.go:32,96` liefert `public/`/`dist/` vom selben Origin wie `/api/studio/`; falls der Import SVG mit Skript zulässt, könnte es die Kopfzeile `X-K3C-Studio` setzen. Älter als WZ19, zu klären.

## Ziel

Die vier Punkte sind behoben oder begründet verworfen.

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

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
