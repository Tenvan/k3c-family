# KB3.1 · Aktionsliste nennt den Bau des Endbosses, Overlay zeigt ihn

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** kb3/1-aktion-lair
- **Abhängig von:** –
- **Tickets:** B-563
- **Kriterien:** AC-01

## Ziel

Die Aktionsliste nennt `lair`, das Overlay zeigt den Bau des Endbosses.

## Kontext

B-563; `docs/protocol.md`.

## Erlaubte Dateien

- `engine/net/actions*.go`, `engine/sim/boss_*`, `data/bosses.json`, `src/scenes/hints/`, `src/core/texts.*`, `src/model/types.ts`, `docs/protocol.md`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. `AtLair`, `noticeUnits`. 2. Aktion `lair`. 3. Overlay-Zeile.

## Fertig, wenn

- [x] AC-01.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 umgesetzt und geprüft: `actions_lair_test.go`, `actionHints.test.ts`, `go test`, `vitest`, `tsc`. Annahme: Hinweisfenster `noticeUnits` = 40 Units (Auslöser 15 Units wäre im selben Tick sichtbar nie).
