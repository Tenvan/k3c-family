# GR9.5 · Abnahme der Monarch-Animationen am TV

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** gr9/5-abnahme-tv
- **Abhängig von:** –
- **Tickets:** B-505
- **Kriterien:** AC-02

## Ziel

🧑 nimmt die Monarch-Animationen am TV ab.

## Kontext

Manuelle Abnahme, nur durch 🧑 (Hardware entkoppelt, `docs/arbeitsweise.md`).

## Erlaubte Dateien

- `docs/sprints/`

## Nicht-Ziele

Umsetzung.

## Schritte

1. Spiel am TV starten, Monarch: Münze geben, Treffer nehmen, sterben, wiederbeleben, zu Fuß und beritten.
2. Ergebnis eintragen; bei Abweichung Ticket.

## Fertig, wenn

- [x] AC-02: 🧑 hat bestätigt (2026-10-24, PC mit Tastatur).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-02 geprüft (Ralf, 2026-10-24, PC mit Tastatur): Münze geben, Treffer nehmen, Sterben und Wiederbeleben ok. Keine Abweichung, keine neuen Tickets. TV/Xbox nicht geprüft.
