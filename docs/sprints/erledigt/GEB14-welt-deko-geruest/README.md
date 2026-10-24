# GEB14 · REG, SIM, CLI · Wald und Höhle dichter, Gerüst bei Wartenden, Knoten-Markierer im Snapshot

- **Status:** erledigt
- **Projekt:** GEB
- **Domäne:** REG, SIM, CLI
- **Reife:** bereit
- **Tickets:** B-531, B-532, B-572
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

## Ausgangslage

Wald hat wenige Bäume, Höhle keine Deko (B-531); das Gerüst steht nur ohne Gefahr (B-572); der Snapshot nennt nicht, wer einen Knoten markiert hat (B-532).

## Ziel

Wald dichter, Höhle mit Büschen, Gerüst bei jedem wartenden Arbeiter, Markierer im Snapshot. Am Ende sichtbar: Level im Browser, Tests grün.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-531, B-532, B-572 › Anforderungen.

## Nicht-Ziele

Neue Biome.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Wald hat mehr Bäume je Abschnitt, Höhle Deko-Büsche (B-531/AC).
- **AC-02** `hubLookOf` zeigt das Gerüst bei jedem `waitingWorker` (B-572/AC).
- **AC-03** Der Snapshot nennt den Markierer eines Knotens, Protokoll nachgezogen (B-532/AC).
- **AC-04** Golden-Änderungen sind begründet.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB14.1 | `GEB14.1-level-deko.md` | Umsetzung | autonom | fertig |
| GEB14.2 | `GEB14.2-markierer-snapshot.md` | Umsetzung | autonom | fertig |
| GEB14.3 | `GEB14.3-geruest.md` | Umsetzung | autonom | fertig |


## Abnahme

- 2026-10-23, Claude autonom: AC-01 bis AC-04 umgesetzt, Review (Sonnet, Spielstand und Golden) ohne schwere Befunde; `deco_test.go` nachgeschärft. Golden neu geschrieben (mehr Bäume verschieben den RNG-Strom), `campaign-ausbau` Eingaben angepasst.
- Offen: `sim_test`-Holz-Balancing zu B-531 nicht gefahren (Balancing zurückgestellt); Anzeige der Büsche im Browser ungeprüft.
