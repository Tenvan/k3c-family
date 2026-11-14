# W2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Plantage, Adern, Stufenbreite und Mine
- **Projekt:** –

## Ziel

Plantage und Adern liefern Material über Zeit, die Insel führt fünf Stufen mit Eisenstollen und Kristallhöhle, die Mine ist spielbar. Am Ende sichtbar: Tests für Generator, Adern, Plantage und Biome grün, aktualisierte Golden-Level.

## Ergebnis

- 2026-10-05 (W2.4, autonom): AC-01 (W2.1: `TestAdernJeStufe`), AC-02 (W2.1: `vein_test.go`, `plantation_test.go`), AC-03, AC-04 (W2.2: `TestValidate500Seeds`, `TestInselFuenfStufen`, `TestBreiteUndDichte`), AC-05 (W2.3: `mine_test.go`), AC-06 (Golden aktuell, `task check:go` und `task check` grün, `-race` übersprungen) mit Nachweis. - Review des Diffs: keine schweren Befunde, keine Befunde behoben (Werte nur in `data/`, deterministisch, höchstens 2 Bauern je Ader, Startvorrat unberührt, `rng.json` und Spielstand unverändert); begründete Abweichungen in W2.2 erklärt. - Neue Tickets aus dem Spr …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Der Generator erzeugt je Mine-Stufe genau 2 Adern, Seeds bleiben reproduzierbar (Test) (B- … |
| AC-02 | – | Adern liefern mit 2 Bauern die Zielrate (±10 %), die Plantage lässt alle 30 s Bäume nachwa … |
| AC-03 | – | Eisenstollen und Kristallhöhle erzeugen mit 500 Seeds gültige Level, die Insel führt fünf … |
| AC-04 | – | Breiten von Höhle und Mine sind angepasst, die Ressourcendichte erfüllt die Zielwerte (Mes … |
| AC-05 | – | Die Mine ist spielbar mit eigenen Ressourcen und Gegnern, Go-Tests decken beides ab (B-012 … |
| AC-06 | geprüft | Golden-Level aktualisiert, `task check` und `task check:go` grün (B-114/AC-05, B-115/AC-04 … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-114 | erledigt | AC-01 –, AC-02 –, AC-06 geprüft | Farm-Plantage lässt Holz nachwachsen, Adern liefern Stein bis Kristall unendlich … |
| B-115 | erledigt | AC-03 –, AC-04 –, AC-06 geprüft | Die Stufen sind nach unten schmaler und dichter, Eisenstollen und Kristallhöhle … |
| B-012 | erledigt | AC-05 – | Mine (Tiefe 2) ist vollständig |
