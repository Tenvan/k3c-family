# SK2.1 · Workshop: Dieb-Skills und Darstellung der Skills

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** –
- **Abhängig von:** –
- **Tickets:** B-504, B-503
- **Kriterien:** AC-01

## Ziel

🧑 bestätigt oder ändert die Defaults zu Dieb und Darstellung (Sprint-README › Offene Fragen); Regeltext folgt der Entscheidung.

## Kontext

Defaults (autonom, Claude, 2026-10-23): Preset Dieb ohne Skills bis zum Dieb-Inhalt (SK2.2); Ring je Klasse beim Wirken und Lernen ohne neue Ereignisse (SK2.3). Frage-Ticket im Sprint KR5. Entscheidet 🧑 anders: Dieb bauen = neuer SIM-Sprint (Backstab „von hinten“, Smoke Bomb Unsichtbarkeit, Balancing); Effekt je Skill = Erweiterung von `LINE_COLORS` zu einer Tabelle je Skill (CLI), evtl. neues Ereignis `cast` (SRV-Session).

## Erlaubte Dateien

- `docs/rules/monarch.md`, `docs/game-design.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Umsetzung.

## Schritte

1. Defaults ansehen (Spiel am TV nach SK2.3).
2. Entscheiden, Regeltext nachziehen oder Folge-Ticket.

## Fertig, wenn

- [ ] AC-01: Entscheidung steht in der Sprint-README, Regeltext nachgezogen.

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
