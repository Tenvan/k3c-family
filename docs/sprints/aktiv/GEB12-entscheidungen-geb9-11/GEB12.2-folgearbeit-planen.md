# GEB12.2 · Folgearbeit planen: Defaults festschreiben, Tickets mit Rang

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** geb12/2-folgearbeit-planen
- **Abhängig von:** –
- **Tickets:** B-531, B-532, B-545
- **Kriterien:** AC-02, AC-03

## Ziel

Defaults D1–D8 sind in den Specs von GEB9, GEB10, GEB11 vermerkt, Folgearbeit (B-531, B-532) ist mit Prio und Verweis geplant.

## Kontext

Vorlage: `docs/sprints/aktiv/GEB12-entscheidungen-geb9-11/README.md` › Offene Fragen. Frage-Tickets B-538 bis B-545 (KR2).

## Erlaubte Dateien

- `docs/sprints/aktiv/GEB9-*/README.md`, `GEB10-*/README.md`, `GEB11-*/README.md` (Offene Fragen)
- `docs/backlog/B-531*`, `B-532*` (über `plan_*`)

## Nicht-Ziele

Werteänderung in `data/`, Protokoll, Spielstand.

## Schritte

1. In GEB9, GEB10, GEB11 › Offene Fragen je Default das Frage-Ticket nennen.
2. B-531 und B-532: Spec mit Verweis auf B-540 bzw. B-539 ergänzen, Prio festlegen.

## Fertig, wenn

- [ ] AC-02: GEB9/10/11 verweisen auf B-538 bis B-545.
- [ ] AC-03: B-531, B-532 mit Verweis und Prio.

## Prüfen

```bash
task test -- planning
```

## Ergebnis

- AC-02 umgesetzt: Frage-Tickets B-538 (D1) bis B-545 (D8) im Sprint KR2 mit Frage, Default, Alternative, Folgen; GEB9, GEB10, GEB11 › Offene Fragen verweisen darauf. Geprüft: `task test -- planning` grün.
- AC-03 umgesetzt: B-531 (D3, nach B-540) und B-532 (D2, Prio niedrig, nach B-539) mit Spec-Abschnitten und Verweis; Folge-Umfang für D1 C, D4, D5, D6, D7 steht im jeweiligen Frage-Ticket. D8 umgesetzt in GEB12.3.
- Abweichung: Einträge in `docs/decisions/` erst nach Ralfs Bestätigung (GEB12.1), weil dort nur bestätigte Entscheidungen stehen.
