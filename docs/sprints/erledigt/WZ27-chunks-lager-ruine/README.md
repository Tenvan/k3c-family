# WZ27 · REG, SIM, CLI · Geplante Chunks: Gegnerlager, Ruinen-Kollision, Dateigrenze

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** REG, SIM, CLI
- **Reife:** bereit
- **Tickets:** B-517, B-522, B-523, B-521
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

## Ausgangslage

Geplante Chunks tragen noch keine Gegnerlager (B-517); Zufalls-Ressourcen und Hub-Plätze können mit Ruinen kollidieren (B-522, B-523); `src/model/types.ts` überschreitet 400 Zeilen (B-521).

## Ziel

Geplante Chunks tragen Gegnerlager mit Wache, Ruinen kollidieren nicht mehr, die Dateigrenze hält. Am Ende sichtbar: `task check:diff` grün, Level-Generator-Tests mit den neuen Fällen.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-517, B-522, B-523, B-521 › Anforderungen.

## Nicht-Ziele

Neue Chunk-Arten.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Geplante Chunks tragen Gegnerlager mit stationärer Wache (B-517/AC).
- **AC-02** Ressourcen liegen nicht auf Ruinen, Ruine auf Hub-Platz gleicher Art wird abgelehnt (B-522/AC, B-523/AC).
- **AC-03** `src/model/types.ts` hat höchstens 400 Zeilen (B-521/AC).
- **AC-04** Golden-Änderungen sind begründet (`task go:golden`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ27.1 | `WZ27.1-level-ruinen-lager.md` | Umsetzung | autonom | fertig |
| WZ27.2 | `WZ27.2-sim-wachen.md` | Umsetzung | autonom | fertig |
| WZ27.3 | `WZ27.3-types-aufteilen.md` | Umsetzung | autonom | fertig |


## Abnahme

- 2026-10-23, Claude autonom: AC-01 bis AC-04 erfüllt, `task check:diff` grün, Review (Sonnet) ohne schwere Befunde; keine Golden-Änderung.
- Annahmen offen für 🧑: Lagerwerte in `data/guards.json` (Aggro 10 Units, höchstens 4 Wachen), Lager-Grafik und Studio-Vorschau fehlen noch.
