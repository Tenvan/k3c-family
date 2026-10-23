# KR1.1 · Workshop: kritische Fragen zu BED entscheiden (🧑)

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** kr1/1-fragen-bed
- **Abhängig von:** –
- **Tickets:** B-525, B-526, B-527, B-528, B-529, B-533, B-534, B-535, B-536, B-537
- **Kriterien:** AC-01, AC-02

## Ziel

Ralf hat alle Frage-Tickets zu BED entschieden; bestätigte sind erledigt, geänderte haben ein Umsetzungs-Ticket.

## Kontext

Tickets und Defaults: Tabelle in `docs/sprints/geplant/KR1-fragen-bed/README.md` › Anforderungen, Details je Ticket unter Notizen. Die Defaults sind umgesetzt und gemergt (BD3 bis BD7). Dieselbe Durchsicht erfüllt BD6.1 (`docs/sprints/aktiv/BD6-entscheidungen-bed/`); beide Sessions zusammen abschließen.

## Erlaubte Dateien

- `docs/backlog/B-525*` … `B-537*` (über `plan_*`)
- neue Tickets im Projekt BED (über `plan_create`)

## Nicht-Ziele

Umsetzung geänderter Entscheidungen; Geräte-Abnahmen BD4.4, BD5.4.

## Schritte

1. Ralf geht die Tabelle durch und vermerkt je Ticket unter Notizen „bestätigt“ oder die Alternative.
2. Claude setzt bestätigte Tickets auf `erledigt`, legt für geänderte ein Umsetzungs-Ticket (Projekt BED) an.
3. BD6.1 und KR1.1 auf `fertig`.

## Fertig, wenn

- [ ] AC-01: Jedes Ticket trägt Ralfs Entscheidung.
- [ ] AC-02: Bestätigte erledigt, geänderte mit Umsetzungs-Ticket.

## Prüfen

```bash
task test -- planning
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
