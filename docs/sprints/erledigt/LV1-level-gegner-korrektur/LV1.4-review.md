# LV1.4 · Review LV1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** lv1/4-review
- **Abhängig von:** LV1.1, LV1.2, LV1.3
- **Tickets:** B-291, B-262, B-200, B-189, B-217, B-260
- **Kriterien:** alle

## Ziel

Alle Kriterien von LV1 sind geprüft, die Abnahme steht in der Sprint-README.

## Kontext

Diff von LV1.1 bis LV1.3 (SIM). `task check`, `task check:go`, `task check:race` nacheinander; Reviewer-Agent getrennt; Golden-Diffs lesen.

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/`
- Fixes unter 30 Zeilen in den Dateien von LV1.1 bis LV1.3

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Prüfläufe.
2. Reviewer.
3. Abnahme.

## Fertig, wenn

- [ ] AC-01 bis AC-06 mit Nachweis oder Ticket.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

LV1.1 bis LV1.3 waren schon per Squash in develop (fc14a8ef, PR #446), `sprint/lv1` gab es auf origin nicht mehr. Deshalb hat Claude den Branch neu von `origin/develop` angelegt und den LV1-Anteil von fc14a8ef im aktuellen Stand geprüft (Reviewer-Agent, Sonnet). Kein schwerer Befund.

- AC-01 geprüft: `TestLavaNichtAufLinien500Seeds`, Golden Eisenstollen mit Begründung.
- AC-02 geprüft: `TestCampAbstandZuLinien500Seeds`, B-262 „so lassen“.
- AC-03 geprüft: `TestAggressionspoolMitAdernNichtVorVierMinuten`; die Messung über 100 Seeds ist verschoben (B-565).
- AC-04 geprüft: `TestEventsKillMitGold`, `TestFluechtenderGoblinVerschwindetImPortal`; Münzen werden nicht doppelt abgerechnet.
- AC-05 geprüft: `TestEventBuiltMitOrt`, `TestEventPlayerDownMitOrt`, Client-Tests in `effects.test.ts` samt Rückfall.
- AC-06 geprüft: `TestSchutzplatzOhneEntityID`. Offen bleibt der stabile Platz, wenn eine Truppe stirbt oder dazukommt, außerdem der Golden-Nachweis aus B-260 (verschoben nach B-605).

Prüfläufe: `task check` grün. `task check:go` war rot, aber nur `cmd/k3c-load` und `engine/devrun` mit Timeouts unter Last (WZ28 lief parallel, B-284). Einzeln wiederholt sind beide grün, `go:lint` ebenfalls. `task check:race` wurde lokal übersprungen, weil kein C-Compiler da ist; `-race` prüft die CI.
