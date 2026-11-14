# WZ41 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** CI und Release grün, check:diff-Protokoll belegt
- **Projekt:** WZG

## Ziel

CI auf `develop` und der Release-Workflow sind grün und prüfen vor dem Bauen; die schon umgesetzten Tickets sind belegt und geschlossen. Am Ende sichtbar: GitHub Actions auf `develop` alle Jobs grün; `release.yml` mit `task typecheck` und `task test` vor `task build`; `logs_query service=k3c-workbench pattern=schritt` zeigt Schritte mit Dauer.

## Ergebnis

2026-11-13, Claude (WZ41.3, autonom). Kriterien: AC-04 belegt, AC-01 bis AC-03, AC-05, AC-06 lokal bzw. in der Konfiguration belegt (Ergebnisse WZ41.1, WZ41.2, WZ41.4, WZ41.3); der CI-Lauf auf `develop` mit Tests und `check:all` folgen nach dem Merge. Befunde: keine schweren; `task check:go` testet gesperrte Sim-Pakete → B-832. Abnahme-Ticket: B-833 (CI develop mit Tests grün, Laufzeiten, check:all); kein Gen-Feature geändert, Sim und Balancing gesperrt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | teilweise | Grafik-Golden je Gruppe, Schritt-Protokoll und Prüf-Slot belegt (`B-739/AC-01` bis `B-739/ … |
| AC-02 | – | CI-Jobs, die Grafik lesen, rendern sie vorher; CI auf `develop` grün (`B-756/AC-01`). |
| AC-03 | – | `task go:race` ohne `engine/balance`, datadoc-Test grün, Jobs `go` und `k3c-workbench` grü … |
| AC-04 | – | Docker-Image entsteht ohne `.work` im Kontext (`B-759/AC-01`). |
| AC-05 | – | `release.yml` mit `task typecheck` und `task test` vor `task build`, CI grün, `task check: … |
| AC-06 | – | Die CI führt die Tests wieder aus, ohne Sim- und Balance-Pakete, Laufzeit je Job belegt (` … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-739 | erledigt | AC-01 teilweise | check:diff: Grafik-Golden je Gruppe, Schritt-Logging und Sperre gegen parallele … |
| B-756 | erledigt | AC-02 – | CI rendert die Papierschnitt-Grafik vor den Go-Tests, die sie lesen |
| B-757 | erledigt | AC-03 – | CI: Race-Lauf ohne engine/balance, datadoc-Test kennt Balance-Berichtsdateien |
| B-758 | erledigt | AC-05 – | Release-Workflow: Typecheck und Tests vor dem Bauen wieder einschalten |
| B-759 | erledigt | AC-04 – | Docker-Build im Release scheitert an .work (EXDEV beim gen:paper) |
| B-825 | erledigt | AC-06 – | Die CI führt die Tests wieder aus (seit #531 abgeschaltet) |
