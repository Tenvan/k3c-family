# WZ27 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Geplante Chunks: Gegnerlager, Ruinen-Kollision, Dateigrenze
- **Projekt:** WZG

## Ziel

Geplante Chunks tragen Gegnerlager mit Wache, Ruinen kollidieren nicht mehr, die Dateigrenze hält. Am Ende sichtbar: `task check:diff` grün, Level-Generator-Tests mit den neuen Fällen.

## Ergebnis

- 2026-10-23, Claude autonom: AC-01 bis AC-04 erfüllt, `task check:diff` grün, Review (Sonnet) ohne schwere Befunde; keine Golden-Änderung. - Annahmen offen für 🧑: Lagerwerte in `data/guards.json` (Aggro 10 Units, höchstens 4 Wachen), Lager-Grafik und Studio-Vorschau fehlen noch.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Geplante Chunks tragen Gegnerlager mit stationärer Wache (B-517/AC). |
| AC-02 | erfüllt | Ressourcen liegen nicht auf Ruinen, Ruine auf Hub-Platz gleicher Art wird abgelehnt (B-522 … |
| AC-03 | erfüllt | `src/model/types.ts` hat höchstens 400 Zeilen (B-521/AC). |
| AC-04 | erfüllt | Golden-Änderungen sind begründet (`task go:golden`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-517 | erledigt | AC-01 erfüllt | Geplante Chunks tragen Gegnerlager mit stationärer Wache |
| B-522 | erledigt | AC-02 erfüllt | Zufalls-Ressourcen eines geplanten Chunks können auf seiner Ruine liegen |
| B-523 | erledigt | AC-02 erfüllt | Eine Ruine auf derselben Lage wie ein Hub-Platz gleicher Art wird nicht abgelehn … |
| B-521 | erledigt | AC-03 erfüllt | src/model/types.ts liegt über der Dateigrenze von 400 Zeilen |
