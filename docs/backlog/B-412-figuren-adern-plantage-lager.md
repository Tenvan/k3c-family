# B-412 · Adern, Plantage und Lager haben keine Anzeige als Figuren im Client

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** WRT
- **Erstellt:** 2026-10-11
- **Spec:** Entwurf
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

Aus W6.2 verschoben (B-117 › Anforderungen): Adern (Abbau läuft), Plantage (Bäume wachsen) und Lager als Figuren werden im Client nicht angezeigt. Stand 2026-10-16 (Beleg `src/model/types.ts`, `engine/sim/plantation.go`, GEB5/GEB6-Specs):
- **Grafik** kommt aus GEB, nicht aus GR3: Lager (`storage`) und Plantage (Setzling auf leerem Platz, Pappel auf gewachsenem; Client leitet leere Plätze aus Farm-Position und Bäumen ab) in GEB5.4 (CLI), Adern (`vein:*`, 2 × 2 Kacheln, vier Stufen) in GEB6.5 (CLI).
- **Zustand im Snapshot:** Adern-Abbau (`nodes[].workerId`, `progress`) und Lager voll (`stock` gegen `stockMax`) sind schon da; „Bäume wachsen“ nicht: `growIn` steht nur im Spielstand (`engine/sim/save_economy.go`), nicht im Snapshot; GEB5 verzichtet darauf (feinere Phasen bräuchten ein Server-Feld, eigenes Ticket SIM/SRV).

## Ziel

Nach GEB5 und GEB6 zeigt der Client den Zustand an den neuen Figuren: Ader mit laufendem Abbau, Lager voll, Plantage mit Wachstum (letzteres erst mit Server-Feld). Die Grafik selbst gehört GEB; dieses Ticket bleibt die Anzeige des Zustands.

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

Vorschlag (2026-10-16, Entwurf, entscheidet 🧑): Das Ticket verlässt W11 (`Sprint: –`), weil GEB5.4 und GEB6.5 dieselben Dateien in `src/scenes/` ändern und die Grafik der Adern erst mit GEB6 kommt. Es wird nach GEB6 neu eingeplant, dann mit Kriterien (Ader: Abbau läuft aus `workerId`/`progress`; Lager voll aus `stock`/`stockMax`; Plantage: Wachstum nur mit Server-Feld, eigenes Ticket SIM/SRV).

## Notizen

Revision 2 (2026-10-16, Entwurf): aus W11 genommen und auf `offen` gesetzt; vorher `eingeplant` in W11 (AC-04). „Figuren“ meint hier Objekte und Gebäude, nicht Bürger (Glossar › Bürger).
