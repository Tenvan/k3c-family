# GEB12.1 · Workshop: D1–D8 bestätigen (🧑)

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** REG
- **Umgebung:** live
- **Branch:** geb12/1-workshop-entscheidungen
- **Abhängig von:** –
- **Tickets:** B-538, B-539, B-540, B-541, B-542, B-543, B-544, B-545
- **Kriterien:** AC-01

## Ziel

Ralf hat D1–D8 bestätigt oder geändert; bestätigte Regelentscheidungen stehen in `docs/decisions/`.

## Kontext

Frage-Tickets B-538 (D1) bis B-545 (D8) im Sprint KR2, je mit Default und Alternative unter Notizen. D4, D5, D7 sind Geschmacksfragen: am besten zusammen mit den Abnahmen GEB9.6, GEB10.5, GEB11.6 im Asset-Studio und im Spiel ansehen. Dieselbe Durchsicht erfüllt KR2.1.

## Erlaubte Dateien

- `docs/backlog/B-531*`, `B-532*`, `B-538*` … `B-545*` (über `plan_*`)
- `docs/decisions/` (neue Entscheidung, falls D1 C oder D2 Ja)
- `docs/sprints/aktiv/GEB9-*`, `GEB10-*`, `GEB11-*` (Offene Fragen)

## Nicht-Ziele

Umsetzung der Folgearbeit.

## Schritte

1. Ralf vermerkt je Ticket unter Notizen „bestätigt“ oder die Alternative.
2. Claude zieht Änderungen nach: Folge-Ticket bzw. Sprint im Projekt GEB, Eintrag in `docs/decisions/` bei Protokoll- oder Regelwirkung.
3. Frage-Tickets `erledigt`.

## Fertig, wenn

- [ ] AC-01: Jedes der acht Tickets trägt Ralfs Entscheidung; Regelentscheidungen stehen in `docs/decisions/`.

## Prüfen

```bash
task test -- planning
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
