# KR2.1 · Workshop: kritische Fragen zu GEB entscheiden (🧑)

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** REG
- **Umgebung:** live
- **Branch:** kr2/1-fragen-geb
- **Abhängig von:** –
- **Tickets:** B-538, B-539, B-540, B-541, B-542, B-543, B-544, B-545
- **Kriterien:** AC-01, AC-02

## Ziel

Ralf hat D1–D8 (B-538 bis B-545) entschieden; bestätigte sind erledigt, geänderte haben Folgearbeit im Projekt GEB.

## Kontext

Tabelle in `docs/sprints/aktiv/KR2-fragen-geb/README.md` › Anforderungen, Details je Ticket unter Notizen. Dieselbe Durchsicht erfüllt GEB12.1 (`docs/sprints/aktiv/GEB12-entscheidungen-geb9-11/`); beide zusammen abschließen. Geschmacksfragen D4, D5, D7 zusammen mit den Abnahmen GEB9.6, GEB10.5, GEB11.6 im Asset-Studio und im Spiel ansehen.

## Erlaubte Dateien

- `docs/backlog/B-531*`, `B-532*`, `B-538*` … `B-545*` (über `plan_*`)
- neue Tickets/Sprints im Projekt GEB (über `plan_create`)
- `docs/decisions/` (bei Protokoll- oder Regelwirkung)

## Nicht-Ziele

Umsetzung der Folgearbeit; die Geräte-Abnahmen selbst.

## Schritte

1. Ralf vermerkt je Ticket unter Notizen „bestätigt“ oder die Alternative.
2. Claude setzt bestätigte auf `erledigt`, plant geänderte als Ticket/Sprint im Projekt GEB.
3. GEB12.1 und KR2.1 auf `fertig`.

## Fertig, wenn

- [ ] AC-01: Jedes Ticket trägt Ralfs Entscheidung.
- [ ] AC-02: Bestätigte erledigt, geänderte mit Folgearbeit.

## Prüfen

```bash
task test -- planning
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
