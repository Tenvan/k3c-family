# W9 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Welt spiegelt Lager-Maximum, Hub-Ausbau und Gefahr
- **Projekt:** –

## Ziel

W5.1 kann alle Wirtschaftswerte ohne eigene Regel-Rechnung aus `*sim.World` übernehmen. Am Ende sichtbar: ein Go-Test belegt die Werte im JSON der Welt, `task check:go` grün.

## Ergebnis

- 2026-10-06 (W9.2, autonom): AC-01 (W9.1: `economy_view_test.go`, `sim.EconomyOf` statt Weltfeldern, von der Spec erlaubt; Grund: neue Welt-Schlüssel brächen `TestFormWieBeispiele`), AC-02 (Welt-JSON und Golden unverändert, `task check` und `task check:go` grün) mit Nachweis. - Review des Diffs: keine schweren Befunde, keine behoben; Hinweis: `danger` ist ein Wert der Stufe, kein Wert je Bauplatz, `waitingWorker` plus `danger` heißt Gefahr. - Neue Tickets: keine. - Version: v0.13.1 vorgeschlagen (Patch: neue Lese-Schnittstelle `sim.EconomyOf` für W5.1, kein geändertes Verhalten im Spiel, Serv …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Ein Go-Test in `engine/sim/` belegt, dass `json.Marshal(world)` (oder eine exportierte Fun … |
| AC-02 | – | Golden-Daten sind nur um die neuen Felder gewachsen (Begründung im Commit), `task check` u … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-323 | erledigt | AC-01 erfüllt | Die Welt spiegelt Lager-Maximum, Hub-Ausbau mit Kosten und Wartegrund „Gefahr“ f … |
