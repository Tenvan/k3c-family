# AZ1.4 · Workshop: Abnahme am PC

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** sprint/az1
- **Abhängig von:** AZ1.3
- **Tickets:** B-371, B-333
- **Kriterien:** AC-03, AC-04

## Ziel

🧑 nimmt am PC ab: zwei Spieler an einer Tastatur sehen je ihre Tasten, und eine Pause über `/dm` zeigt in jeder Zelle „Pausiert“ bei stillstehenden Figuren.

## Kontext

Stand nach AZ1.3 (PR des Sprints offen oder gemergt). Server im Dev-Mode (sonst kein `devPaused`), `game.html` am PC, `/dm` in einem zweiten Tab.

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Code ändern; Befunde werden Tickets.

## Schritte

1. Zwei Spieler an einer Tastatur beitreten (Leertaste, Enter). Spieler 2 an einen Bauplatz: Hinweis, Skill-Leiste und Skill-Menü (Ziffernblock 5) nennen seine Tasten.
2. `/dm` → „⏸ Pause“: in beiden Zellen „Pausiert“, alle Figuren stehen. „▶ Weiter“: Anzeige weg, Figuren laufen.
3. Ergebnis eintragen; ist dies die letzte offene Session, Sprint abschließen.

## Fertig, wenn

- [ ] AC-03: 🧑 sieht die Pause-Anzeige in jeder Zelle und sie verschwindet nach „Weiter“.
- [ ] AC-04: 🧑 sieht die Figuren während der Pause stillstehen und danach wieder laufen.

## Prüfen

Beobachtung am PC durch 🧑.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
