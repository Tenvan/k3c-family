# WZ42 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Workbench-Tests schnell und stabil
- **Projekt:** WZG

## Ziel

Ein geändertes Workbench-Paket prüft `check:diff` in Sekunden, und `check:dev` ist auch bei parallelen Agenten verlässlich grün. Am Ende sichtbar: `go test -short -count=1 -json ./...` in `tools/k3c-workbench` ohne Test ≥ 3 s; `check_run task:check:dev` zweimal nacheinander grün.

## Ergebnis

2026-11-13, Review WZ42.4 (Claude): `task check` und `task check:go` grün, `task check:dev` Lauf 1 und 4 grün, 2 und 3 rot nur in Katalog-Vitests außerhalb des Bereichs (B-837). Kriterien: AC-01, AC-02, AC-04 geprüft (WZ42.1, WZ42.3 › Ergebnis); AC-03 teilweise, Rest `mcpsrv`/`planning` verschoben → B-830, „zweimal grün“ → B-837 (WZ42.2, WZ42.4 › Ergebnis). Behoben: falscher CI-Hinweis in `internal/balance/hints_test.go`. Neue Tickets: B-828 (CI ohne Workbench-Go-Tests), B-829 (Flake `TestDateiAenderungStartetNeu`, nicht reproduziert), B-830, B-831 (`check_run dev:test`), B-837. Kein Abnahme-T …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Workbench-Go-Suite auf frischem Checkout grün, Tests unverändert aussagekräftig (`B-738/AC … |
| AC-02 | geprüft | Wackelnde Tests stabil: `B-702/AC-01`, `B-621/AC-01`, `B-695/AC-01`, `B-695/AC-02`, `B-704 … |
| AC-03 | teilweise | Kein Test ≥ 3 s mit `-short`, jedes Paket allein ≤ 10 s, `check:dev` zweimal grün (`B-787/ … |
| AC-04 | geprüft | Keine unbegründeten Verweise auf `bin/k3c-balance`, `internal/enginetools`, `internal/bala … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-738 | erledigt | AC-01 geprüft | Workbench-Go-Tests rot: Style-Fixture, Rig-Vorlage vierbeiner ohne Gangarten, Re … |
| B-787 | erledigt | AC-03 teilweise | Workbench-Tests bleiben mit -short unter 3 s je Test, der volle Lauf ist grün |
| B-704 | erledigt | AC-02 geprüft | Lastabhängige Go-Tests scheitern bei parallelen Agenten |
| B-695 | erledigt | AC-02 geprüft | Zwei Tests flackern: simrun-TempDir-Cleanup und Abbruch eines wartenden Laufs |
| B-702 | erledigt | AC-02 geprüft | TestRunProcessExitUndAusgabe wackelt: fehlendes Programm ohne Fehler |
| B-621 | erledigt | AC-02 geprüft | TestSourcesUndConsoleTail flackert, wenn die Ressourcen-Zeile früh kommt |
| B-747 | erledigt | AC-04 geprüft | Verweise auf bin/k3c-balance und internal/enginetools außerhalb der Workbench na … |
