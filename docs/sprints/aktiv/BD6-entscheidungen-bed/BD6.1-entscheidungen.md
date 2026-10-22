# BD6.1 · Entscheidungen zu BD2 bis BD7 bestätigen (🧑)

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** bd6/1-entscheidungen
- **Abhängig von:** –
- **Tickets:** B-525, B-526, B-527, B-528, B-529, B-533, B-534, B-535, B-536, B-537
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Ralf hat jede autonome Entscheidung aus BD2 bis BD7 bestätigt oder geändert; die Frage-Tickets sind erledigt, Abweichungen stehen als Umsetzungs-Ticket im Projekt BED.

## Kontext

Die Frage-Tickets liegen im KRT-Sprint KR1 (`docs/sprints/geplant/KR1-fragen-bed/`), damit Ralf alle kritischen Fragen zu BED an einer Stelle findet; dieselbe Durchsicht erfüllt KR1.1. Je Ticket stehen unter Notizen Frage, Default (umgesetzt) und Alternative. Bis zur Entscheidung gelten die Defaults.

- B-525 Goldtopf, B-526 fester WORLD_TOP, B-527 Abmelden mit zwei Tastatur-Spielern (BD4)
- B-528 Regeldetails der Raum-Pause, B-529 Protokollform pause/resume (BD3)
- B-533 Löschen ohne Token mit Sicherung, B-534 Spielmenü gleich Lobby, B-537 vierter Punkt der Meldung (BD5)
- B-535 Pause nach Wiederverbinden erneut senden (BD7)
- B-536 Versionen v0.21.0 (BD3) und v0.21.1 (BD7)

## Erlaubte Dateien

- `docs/backlog/B-525*` … `B-537*` (über `plan_*`)
- neue Tickets im Projekt BED (über `plan_create`)
- `docs/sprints/erledigt/BD3-raum-pause/README.md`, `docs/sprints/erledigt/BD7-pause-client-skill-schutz/README.md` (nur Abnahme › Version)

## Nicht-Ziele

Umsetzung geänderter Entscheidungen (eigene Tickets); Geräte-Abnahmen BD4.4 und BD5.4.

## Schritte

1. Ralf liest je Ticket die Notizen und vermerkt dort „bestätigt“ oder die gewählte Alternative.
2. Bestätigt: Ticket `erledigt`. Geändert: Claude legt ein Umsetzungs-Ticket an, Frage-Ticket `erledigt` mit Verweis.
3. B-536: Version in den Abnahmen von BD3 und BD7 nachtragen oder verwerfen.

## Fertig, wenn

- [ ] AC-01: Jedes der zehn Tickets hat unter Notizen Ralfs Entscheidung.
- [ ] AC-02: Zu jeder geänderten Entscheidung existiert ein Umsetzungs-Ticket; bestätigte sind erledigt.
- [ ] AC-03: BD3 und BD7 nennen ihre Version oder „verworfen“.

## Prüfen

```bash
task test -- planning
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
