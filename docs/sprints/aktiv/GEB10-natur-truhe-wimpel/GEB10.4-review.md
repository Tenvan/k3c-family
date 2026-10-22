# GEB10.4 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb10/4-review
- **Abhängig von:** GEB10.3
- **Tickets:** B-454, B-455, B-456
- **Kriterien:** alle

## Ziel

Der Diff von GEB10 ist geprüft (nur schwere Befunde), die Abnahme steht in der Sprint-README, der eine PR des Sprints ist offen.

## Kontext

- Ablauf: `docs/arbeitsweise.md` › Review-Session. Reviewer getrennt vom Schreiben.
- Besonders: Wimpel-Zuordnung ohne `Math.random()`, mit zwei Spielern unterscheidbar; gelöschte PNGs nirgends mehr genutzt; Plantage-Pappel hinter der Farm; Truhe beim Stufenwechsel nicht fälschlich „offen“; Datei ≤ 400, Funktion ≤ 60 Zeilen.
- GEB10.5 (Mensch) ist keine Abhängigkeit des Reviews; der Sprint bleibt aktiv, bis sie fertig ist.

## Erlaubte Dateien

- Dateien der Sessions GEB10.1–GEB10.3 (nur Fixes schwerer Befunde)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Sichtprüfung im Spiel (GEB10.5); Stil- oder Vereinfachungsvorschläge.

## Schritte

1. `task check`, `task check:go`, `task check:race` grün.
2. `git diff origin/develop...origin/sprint/geb10` lesen, schwere Befunde beheben oder als Ticket.
3. Abnahme in der README (≤ 5 Zeilen), PR öffnen, Auto-Merge.

## Fertig, wenn

- [ ] Alle Kriterien haben Nachweis in den Session-Ergebnissen (AC-04 steht bei GEB10.5 aus).
- [ ] Checks grün, PR offen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
