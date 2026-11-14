# WZ50 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Katalog-Vitests der Workbench bleiben unter Last grün
- **Projekt:** WZG

## Ziel

Die Katalog-Vitests bleiben unter Last grün, `task check:dev` zweimal hintereinander grün.

## Ergebnis

2026-11-14, Claude. AC-01 geprüft (WZ50.1 › Ergebnis, WZ50.2 › Ergebnis): Katalog-Vitests 40 von 40 Mal grün unter Last, `task check:dev` zweimal hintereinander grün, `task check` und `go:test` grün. Behoben: Die Katalog-Tests warten jetzt auf Bedingungen statt auf `flush()` und feste Zeiten (`catalog/catalogTesting.tsx`). Neue Tickets (anzulegen, durch den Aufrufer): `readonly.test.tsx` unter Last, `TestDateiAenderungStartetNeu` unter Last. Abnahme-Ticket für Sim, Balance und Gen: keins nötig, der Sprint ändert nur Frontend-Tests ohne Sim-, Balance- oder Gen-Bezug.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `B-837` Kriterien grün; `task check:dev` zweimal hintereinander grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-837 | erledigt | AC-01 geprüft | Katalog-Vitests der Workbench bleiben unter Last grün |
