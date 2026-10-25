# GD1 · INF · Golden-Daten kleiner bei gleicher Prüfschärfe

- **Status:** aktiv
- **Projekt:** TST
- **Domäne:** INF
- **Reife:** bereit
- **Tickets:** B-419
- **Start-Commit:** 80f2d3e9
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25, Revision 1 (Ziel von Ralf: Specs genehmigt; Weg vorentschieden über B-624; gestapelt auf sprint/geb17 wegen der Golden-Regel)

## Ausgangslage

Die Golden-Daten in `testdata/golden/` liegen bei 12,5 MB (Stand 2026-10-13), vorläufige Grenze 16 MB (B-417). 85 % sind die zwölf `sim-*.json` mit vollem Welt-Snapshot alle 60 Ticks. Gemessen: gzip → 37 KB statt 1,66 MB (Diffs binär), `snapshotEvery` 120 → halbe Größe bei halb so vielen Vergleichspunkten (B-419). 🧑 will sich gezielt darum kümmern.

## Ziel

Die Golden-Daten bleiben deutlich unter der Grenze, ohne dass die Go-Tests weniger Abweichungen finden. Am Ende sichtbar: kleinere `testdata/golden/`, `task go:test` grün, Diffs bleiben lesbar.

## Beteiligte und Zielgruppen

Entwickler und Agenten mit Golden-Änderungen, CI; 🧑 wählt den Weg; Agent in INF.

## Anforderungen

B-419 › Ausgangslage (Spec noch Entwurf). Sprint-eigen: Prüfschärfe gilt als gehalten, wenn eine absichtlich eingebaute Abweichung weiter erkannt wird.

## Nicht-Ziele

Neue Golden-Szenarien; Änderung von `rng.json`; Regel- oder Wertänderungen.

## Regeln und Einschränkungen

Domäne INF; `docs/arbeitsweise.md` › Golden aktualisieren (eigener Commit mit Begründung, nur ein aktiver Sprint ändert Golden). Golden amd64 und arm64 grün.

## Beispiele

Ein Tick der Simulation ändert absichtlich ein Feld → Golden-Test scheitert wie vorher an der richtigen Stelle.

## Ausnahme- und Fehlerfälle

Gewählter Weg macht Diffs unlesbar → Lesehilfe (`task`) oder anderer Weg, im Ergebnis begründet.

## Akzeptanzkriterien

- **AC-01** 🧑 hat den Weg gewählt (z. B. Kompression, seltenere Snapshots, nur geänderte Felder); die Größe vorher und nachher steht im Ergebnis (B-419).
- **AC-02** Eine absichtlich eingebaute Abweichung wird weiter erkannt (Test), `task go:test` amd64 und arm64 grün (B-419).

## Offene Fragen

🧑: Welcher Weg (gzip, `snapshotEvery`, Deltas)? Neue Grenze statt 16 MB?

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GD1.1 | `GD1.1-wege-messen.md` | Workshop | Mensch | fertig |
| GD1.2 | `GD1.2-golden-gzip.md` | Umsetzung | autonom | fertig |
| GD1.3 | `GD1.3-review.md` | Review | autonom | offen |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GD1.1 (INF, Mensch) Workshop: Wege messen und wählen (AC-01).
- GD1.2 (INF) Gewählten Weg umsetzen, Golden neu schreiben, Abweichungs-Test (AC-01, AC-02).
- GD1.3 (INF) Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
