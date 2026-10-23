# KR1.1 · Workshop: kritische Fragen zu BED entscheiden (🧑)

- **Status:** fertig
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

- **AC-01 geprüft:** Alle zehn Tickets (B-525 bis B-529, B-533 bis B-537) tragen unter Notizen Ralfs Entscheidung vom 2026-10-23 (Workshop mit Ralf, Claude moderiert und protokolliert).
- **AC-02 geprüft:** Acht Tickets bestätigt (B-525, B-526, B-527, B-528, B-529, B-533, B-535, B-537), alle `erledigt`. Zwei geändert und `erledigt` mit Verweis: B-534 → B-566 (Welt-Auswahl in der Lobby, Alternative A), B-536 → B-567 (BD3 und BD7 als ein Release v0.21.0, Alternative A; Vermerk in den Sprint-READMEs ist nachgetragen, B-567 `erledigt`).
- **Abweichungen:** B-534 trug irrtümlich die Offene Frage von B-537 (Hinweis in B-534 › Notizen). Nur B-566 bleibt offen (Spec Entwurf, Projekt BED).
- **Offen für Ralf:** Die Geräte-Abnahmen BD4.4 und BD5.4 bleiben in ihren Sprints; B-526 wird dort am Gerät geprüft.
- **Hinweis:** v0.21.0 ist in den Abnahmen von TS1, WZ18 und WZ14 ebenfalls vorgeschlagen; Ralf vergibt die Versionsnummern beim Release.
