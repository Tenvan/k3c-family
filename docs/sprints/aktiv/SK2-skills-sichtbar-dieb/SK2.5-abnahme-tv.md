# SK2.5 · Abnahme am TV

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** –
- **Abhängig von:** SK2.3
- **Tickets:** B-503
- **Kriterien:** AC-03

## Ziel

🧑 sieht am TV mit zwei Spielern die Ringe beim Wirken und Lernen und nimmt AC-03 ab.

## Kontext

Umsetzung SK2.3. Zwei Spieler, je ein Skill wirken (Tank Taunt, Zauberer Fireball), einen Passiven lernen.

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Neue Effekte.

## Schritte

1. Spiel am TV starten, zwei Spieler.
2. Skills wirken, Passiven lernen, Ringe beobachten.
3. Ergebnis eintragen, Befunde als Ticket.

## Fertig, wenn

- [ ] AC-03 (B-503/AC-02): Ring am TV sichtbar, Farbe passt zur Klasse.

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
