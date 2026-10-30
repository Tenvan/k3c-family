# KB5.6 · Abnahme im Spiel mit zwei Spielern und im Studio

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** kb5/6-abnahme-kb5
- **Abhängig von:** KB5.5
- **Tickets:** B-674, B-675, B-676
- **Kriterien:** B-674 AC-04, B-675 AC-04, B-676 AC-03

## Ziel

🧑 nimmt im Spiel mit zwei Spielern und im Studio ab: Zauber am Ziel, Status-Effekte sichtbar.

## Kontext

Tickets B-674, B-675, B-676. Spiel über `task serve` bzw. den Server, Studio über `task k3c-studio`.

## Erlaubte Dateien

- keine (Abnahme)

## Nicht-Ziele

- Codeänderungen; Befunde als Ticket.

## Schritte

1. Spiel mit zwei Spielern: fireball, meteor, lightningStorm, shieldBash, iceWall, ironWall, Auren wirken.
2. Studio: Monarch-Seite mit Ziel-Gegner, Gegner-Seite Modus „Status“.
3. Befunde als Ticket.

## Fertig, wenn

- [ ] B-674 AC-04, B-675 AC-04, B-676 AC-03 durch 🧑 bestätigt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
