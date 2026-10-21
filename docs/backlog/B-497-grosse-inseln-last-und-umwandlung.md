# B-497 · Große Inseln: Last im Spiel messen, Umwandlung in allen Biomen testen, PlanSave gegen Parallelität

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ24
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Befunde der Umsetzung WZ17.5 (2026-10-18): (1) Bei „mega groß“ (160 Chunks, 4000 bis 8000 Units) ist die Vorschau-Welt 7- bis 10-mal so groß wie heute (bis 103 KB Snapshot ohne Bürger und Gegner, Erzeugung unter 3 ms); die Last im laufenden Spiel mit Clients (Übertragung, Spielstand, Split-Screen) ist nicht gemessen. (2) Der Test der Umwandlung eines Seed-Chunks ist nur für Wald, Platz −3 sowie Anzahl 0/24/160 geprüft; ob `PreviewChunkObjects` auch Objekte übernimmt, die nicht vom Seed stammen (Adern, Biom-Summen), ist für andere Biome, Stufen 0 bis 4 und mehrere Plätze offen. (3) `PlanSave` im Studio hat keinen Mutex gegen zwei gleichzeitige Aufrufe im selben Prozess (war schon vorher so). (4) Offen gelassen: Wird der letzte Eintrag entfernt, bleibt `"chunkCounts": {}` stehen, wie bei `plannedChunks: {}`. Wie viele Chunks je Stufe sinnvoll sind, ermittelt ein eigenes Projekt (Entscheidung von 🧑, 2026-10-18).

## Ziel

Welches Ergebnis, welcher Nutzen (oder welcher Schaden verschwindet)? Kein Umsetzungsschritt.

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

Entscheidet 🧑 im späteren Größen-Projekt: Welche Größen im Spiel sinnvoll sind (Last mit `sim_test` und Clients messen), und ob Punkte (2) bis (4) vorher oder dort mitlaufen.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
