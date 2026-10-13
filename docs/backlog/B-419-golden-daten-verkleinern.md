# B-419 · Golden-Daten werden gezielt verkleinert, ohne die Prüfschärfe zu verlieren

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** TST
- **Erstellt:** 2026-10-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Golden-Daten in `testdata/golden/` liegen bei 12,5 MB (Stand 2026-10-13), die Grenze ist vorläufig 16 MB (B-417). 85 % davon sind die zwölf `sim-*.json` mit einem vollständigen Welt-Snapshot alle 60 Ticks. Gemessene Wege für `sim-cave-belagerung.json` (1,66 MB): gzip → 37 KB (verlustfrei, aber Diffs binär); `snapshotEvery` 120 → 0,84 MB (halb so viele Vergleichspunkte). 🧑 kümmert sich gezielt darum.

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

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
