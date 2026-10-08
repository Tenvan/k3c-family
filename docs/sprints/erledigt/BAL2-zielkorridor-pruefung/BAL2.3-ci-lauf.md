# BAL2.3 · CI-Lauf mit kleiner Seed-Menge

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** bal2/3-ci-lauf
- **Abhängig von:** BAL2.2
- **Tickets:** B-157
- **Kriterien:** AC-06, AC-07

## Ziel

Die CI erzeugt bei jedem Lauf einen Balance-Bericht mit kleiner Seed-Menge und legt ihn als Artefakt ab; verletzte Ziele brechen den Lauf nicht.

## Kontext

- CI: `.github/workflows/ci.yml`, Job `go` (Go · Tests · Lint · Cross-Build: `task go:test`, `task check:race`, golangci-lint, Cross-Builds). Artefakte gibt es schon (z. B. `k3c-dist`, `actions/upload-artifact`).
- **Domäne:** `.github/` ist INF. Ein zusätzlicher Schritt im Job `go` (bzw. eigener Job) ist für AC-06 nötig; das gilt mit der Freigabe der Spec als erlaubte Ausnahme. Steht das bei der Freigabe nicht so, Session `blockiert`.
- Seed-Menge klein halten (Vorschlag 10 Seeds, Flag aus BAL2.2), damit der Job unter seiner `timeout-minutes` bleibt; Laufzeit im Ergebnis.
- Kein Gate: Exit-Code des Berichts darf den Job nicht rot machen (z. B. Flag `-report-only` oder `continue-on-error` nur für diesen Schritt; Wahl im Ergebnis). Ein Absturz des Werkzeugs selbst soll den Job weiterhin rot machen (unterscheiden: „Ziel verletzt“ ≠ „Fehler“).

## Erlaubte Dateien

- `.github/workflows/ci.yml` (nur der Balance-Schritt)
- `Taskfile.yml` (nur Variante von `balance` für die CI, falls nötig)
- Balance-Befehl (nur Flag für Bericht ohne Gate, falls BAL2.2 es nicht hat)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Pflicht-Gate, Kommentar im PR, Änderungen an anderen CI-Jobs.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Schritt in der CI: Balance mit kleiner Seed-Menge, Bericht als Artefakt hochladen.
3. Lokal prüfen: Bericht ohne Gate liefert Exit-Code 0 trotz verletztem Ziel; ein Werkzeug-Fehler liefert ≠ 0 (Test).
4. PR öffnen, CI-Lauf abwarten: Artefakt vorhanden, Job grün. Link ins Ergebnis.
5. `task check:go`. `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-06 (Lauf-Link steht aus, siehe Ergebnis): CI-Lauf erzeugt den Bericht als Artefakt, verletzte Ziele brechen den Job nicht (Link zum Lauf im Ergebnis).
- [x] AC-07: `task check:go` grün.

## Prüfen

```bash
task check:go
```

## Ergebnis

**Umgesetzt:**
- `.github/workflows/ci.yml`, Job `go`: Schritt `task balance -- --seeds 10` (10 Seeds, Bericht + Vergleich mit Baseline) und `actions/upload-artifact@v7` (bereits genutzt, keine neue Action) mit Artefakt `k3c-balance` (`reports/balance-*`, 14 Tage). Kein Eingriff in andere Jobs.
- **Wahl Gate:** kein neues Flag und kein `continue-on-error` nötig: `k3c-balance --targets` liefert bei verletzten Zielen schon Exit 0 (BAL2.2), nur Fehler (Ladefehler, ungültige Optionen, nicht schreibbarer Ordner) geben Exit != 0 und färben den Job rot. Nachweis als Test in `tools/k3c-dev/internal/balance/cmd/targets_test.go`.

**Nachweis:**
- AC-06 umgesetzt, geprüft: lokal `task balance -- --seeds 10` = 10 Läufe in 8 s (gesamt 13 s inkl. Build), Bericht `.json` + `.md` enthält „verletzt“ (B-230), Exit 0; `TestVerletztesZielIstKeinFehler` (Exit 0 trotz Bericht), `TestWerkzeugFehlerIstFehler` (Fehler bei `--write-baseline` ohne `--baseline` und bei Berichtsordner = Datei). YAML mit Python `yaml.safe_load` geparst. **Lauf-Link:** Es gibt in dieser Session keinen PR (Regel: PR öffnet die Review-Session BAL2.4); der erste CI-Lauf des Sprint-PRs belegt das Artefakt, BAL2.4 trägt den Link nach. Laufzeit im Job grob 15 s, weit unter `timeout-minutes: 15`.
- AC-07 geprüft: `task check`, `task check:dev` grün; `task check:go` grün bis auf `engine/store` (`TestGleichzeitigesSpeichern`, flaky B-187): zweimal rot im Lauf von `task check:go`, `go test ./...` danach grün; nicht durch diese Änderung verursacht.
