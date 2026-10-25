# KB1.4 · Feuergeist: Flammen-Fläche umsetzen oder Regel anpassen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** kb1/4-feuergeist-flaeche
- **Abhängig von:** KB1.3
- **Tickets:** B-328
- **Kriterien:** AC-04

## Ziel

Die Regel zum Feuergeist passt zur Simulation: bis zur Entscheidung von 🧑 ohne Flammen-Fläche, in `docs/rules/gegner.md` vermerkt.

## Kontext

`docs/rules/gegner.md` § 1 nennt „Fernkampf mit Flammen-Fläche“, `data/enemies.json` › Feuergeist hat `ranged` + `kiting`. Default KB1 › Offene Fragen: Regel anpassen statt neue Mechanik.

## Erlaubte Dateien

- `docs/rules/gegner.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Neue Mechanik Bodenfläche für Gegner, Werte.

## Schritte

1. Vermerk in `docs/rules/gegner.md` § 1 mit Verweis auf B-328.
2. Frage-Ticket in KR6.

## Fertig, wenn

- [ ] AC-04 (B-328/AC-01): Regel angepasst, Entscheidung offen bei 🧑 (KR6).

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
