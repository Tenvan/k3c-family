# W3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Gebäude-Wirkungen
- **Projekt:** –

## Ziel

Tor, Kaserne, Taverne, Heilplatz und Zaubertum wirken laut Regelwerk, Schmiede und Rüstkammer sind baubar. Am Ende sichtbar: Tests je Gebäude grün, Werte aus den Daten, aktualisierte Golden-Daten.

## Ergebnis

- 2026-10-05 (W3.3, autonom): AC-01 (W3.1: `gate_test.go`, `barracks_test.go`, `tavern_test.go`; W3.2: `healing_test.go`, `spell_tower_test.go`), AC-02 (`sites_test.go`), AC-03 (`TestSchmiedeUndRuestkammerZerstoerbarWirkungOffen`), AC-04 (Golden geprüft, ohne Diff; `task check:go` und `task check` grün, `-race` übersprungen) mit Nachweis. - Review des Diffs: keine schweren Befunde, keine Befunde behoben; `TestTurmAusbauSchuetzeBleibt` durch Q31 angepasst (nicht gelockert). - Neue Tickets: keine. - Version: v0.10.0 gesetzt (2026-10-05, auf `3a6446a`, `task check:all` und CI grün; Minor: Tor, Ka …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Je Gebäude wirkt es wie beschrieben: Tor, Kaserne-Limit, Taverne, Heilplatz, Zaubertum-Sch … |
| AC-02 | – | Kosten, HP und Platz kommen aus den Daten, nicht aus dem Code (Test) (B-116/AC-02). |
| AC-03 | – | Schmiede und Rüstkammer lassen sich bauen und zerstören, ihre Wirkung ist als offen markie … |
| AC-04 | geprüft | Golden-Daten aktualisiert, `task check:go` grün (B-116/AC-04). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-116 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 geprüft | Tor, Kaserne, Taverne, Heilplatz, Schmiede, Rüstkammer und Zaubertum wirken im S … |
