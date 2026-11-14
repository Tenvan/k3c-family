# W10 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Kämpfer-Zahl und Truppen-Limit im Zustand
- **Projekt:** –

## Ziel

Der Server sendet Kämpfer-Zahl und Truppen-Limit je Stufe im Zustand, damit W6.1 den Limit-Text ohne Rechnung im Client bildet. Am Ende sichtbar: `fighters` und `troopLimit` in `docs/protocol.md` › Wirtschaft und in `testdata/protocol/`, `task check` und `task check:go` grün.

## Ergebnis

- 2026-10-06 (W10.3, autonom): AC-01 (W10.1: B-332 Variante A, W6.1 abhängig von W10), AC-02 (W10.1: `TestEconomyKaempferUndLimit`, Golden und Welt-JSON unverändert), AC-03 (W10.2: `TestKaempferZustandUndDelta`, `TestFormWieBeispiele`, `TestWirtschaftBeispiel`, `clientWirtschaft.test.ts` mit und ohne die Felder) mit Nachweis. - Review des Diffs: keine schweren Befunde, keine behoben; in `engine/sim/` nur `economy_view*.go`, Regel nur in `barracks.go`, Version bleibt 5. - Neue Tickets: keine. B-332 erledigt und archiviert. - Version: v0.14.1 vorgeschlagen (Patch: zwei zusätzliche optionale Zust …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Variante A ist in B-332 mit Datum durch 🧑 festgehalten, W6 ist angepasst (W6.1 hängt von W … |
| AC-02 | geprüft | `sim.EconomyOf` liefert Kämpfer-Zahl und Truppen-Limit der Stufe, abgelesen aus `fighters( … |
| AC-03 | – | Der Server sendet `fighters` und `troopLimit` oben in `s` (`snap`, `delta`); `docs/protoco … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-332 | erledigt | AC-01 –, AC-02 geprüft, AC-03 – | Kämpfer-Zahl und Truppen-Limit stehen im Zustand |
