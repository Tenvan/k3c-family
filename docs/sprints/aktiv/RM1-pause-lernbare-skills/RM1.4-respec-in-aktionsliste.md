# RM1.4 · Aktionsliste nennt respec

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** rm1/4-respec
- **Abhängig von:** RM1.2
- **Tickets:** B-495
- **Kriterien:** AC-03

## Ziel

Die Aktionsliste nennt `{ "action": "respec" }` genau dann, wenn `sim.CanRespec` nil liefert.

## Kontext

`engine/net/actions.go` (`actionsOf`), `sim.CanRespec` in `engine/sim/monarch.go` (Tag und Burg-Nähe; ohne gelernte Skills ebenfalls erlaubt, Beschluss 🧑 2026-10-06). Der Client kennt `respec` schon (`PlayerAction`, `menuEntries`). Reihenfolge laut `docs/protocol.md`: attack, skill, learn, respec. Gefallener Monarch: kein respec (wie alle Aktionen am Ort).

## Erlaubte Dateien

- `engine/net/actions.go`, `engine/net/*_test.go`
- `docs/protocol.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Client-Anzeige, Ausbilden und Tauschen (B-410).

## Schritte

1. `actionsOf` ruft `sim.CanRespec` auf (nur lebend) und hängt `respec` an.
2. Test in `engine/net`: Tag an der Burg → respec; Nacht oder fern → fehlt.
3. Doku-Absatz nachziehen; `testdata/protocol/` bleibt unverändert (Format gleich, Schlüssel nur im ersten Listeneintrag geprüft).
4. `task check:diff`.

## Fertig, wenn

- [ ] AC-03 (B-495/AC-01): Test zeigt respec am Tag an der Burg, nicht nachts, nicht fern der Burg.
- [ ] AC-03 (B-495/AC-02): `go:test`/`go:lint` grün, `testdata/protocol/` unverändert.

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
