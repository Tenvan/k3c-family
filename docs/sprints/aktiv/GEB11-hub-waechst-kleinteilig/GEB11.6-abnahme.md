# GEB11.6 · Abnahme der Wachstumsreihe in Vorschau und Spiel

- **Status:** blockiert
- **Typ:** Review
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb11/6-abnahme
- **Abhängig von:** GEB11.4
- **Tickets:** B-458
- **Kriterien:** AC-03, AC-04

## Ziel

🧑 nimmt die Wachstumsreihe in der Studio-Vorschau und im Spiel ab.

## Kontext

Vorschau: Asset-Studio, Grafik › Szenen › Gebäude, Streifen „Hub wächst (GEB11)“. Spiel: neue Insel (Lagerfeuer), Münzen an der Burg zahlen (Zelt ab der ersten Münze, Hütte ab der Hälfte, Holzburg bei vollem Gold, Gerüst während der Bauer baut), Hub-Stufe 2 (Steinburg). Dev-Aktionen für Gold und Material helfen.

## Erlaubte Dateien

- Planungsdateien

## Nicht-Ziele

Neue Grafik; Befunde werden Tickets.

## Schritte

1. Vorschau ansehen.
2. Im Spiel eine Insel von Lagerfeuer bis Steinburg ausbauen, 2 Spieler.
3. Ergebnis eintragen, Befunde als Tickets.

## Fertig, wenn

- [ ] AC-03, AC-04: 🧑 hat Vorschau und Spiel abgenommen (Datum im Ergebnis).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Blockiert (2026-10-22): manuelle Abnahme in Studio-Vorschau und Spiel gehört 🧑 (`docs/arbeitsweise.md` › SDD, Abnahme); wartet auf 🧑, nachdem GEB11.5 (Review) gelaufen ist.
