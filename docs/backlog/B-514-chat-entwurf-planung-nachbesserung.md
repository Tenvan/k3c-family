# B-514 · Chat-Entwurf: Planung bei gleicher Insel-id schützen, CLI-Prozessbaum, Test-Riegel

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ21
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

Punkt 1 behoben (2026-10-20, PR Tenvan/k3c-family#400): `draftOfAnswer` setzt `From` für Inseln, deren id schon in der Welt-Datei steht, Test `TestAntwortBehaeltPlanungBestehenderInsel`. Offen bleiben Punkt 2 bis 4 sowie aus dem Review des Fixes: `mockWorldDraft.ts:96` setzt `from` noch immer leer (Mock weicht von Go ab); ein offener Chat-Entwurf bleibt beim Weltwechsel stehen (`useWorldDraft.ts:32`, älter).

**Einmaliger Fehlschlag von `go test ./backend` (2026-10-20 untersucht):** Ursache ist `writeAtomic` (`tools/k3c-studio/backend/plan_file.go:210`): `os.Rename` auf die bestehende Welt-Datei scheitert unter Windows mit `Access is denied`, wenn ein Fremdprozess (vermutlich Virenscanner oder Indexer) die frisch geschriebene Datei kurz offen hält. Betroffen war `TestNotizenAnlegenUndZeilenendeDerDatei`. 15 Läufe nacheinander grün; erst bei 8 parallelen Test-Binaries (je `-test.count=5`) trat er in einem von 8 auf, also last- und zeitabhängig, nicht in der Logik. Dasselbe Muster wie B-187 (`engine/store`); der Fehler kann im Betrieb auch ein Speichern der Welt im Studio treffen (Meldung statt Datenverlust, die Datei bleibt unverändert). Lösung gemeinsam mit B-187: Rename mit kurzer Wiederholung (z. B. 5 × 20 ms bei `ERROR_ACCESS_DENIED`/`ERROR_SHARING_VIOLATION`).
