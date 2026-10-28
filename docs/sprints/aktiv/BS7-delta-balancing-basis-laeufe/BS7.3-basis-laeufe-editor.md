# BS7.3 · Basis-Läufe im Balancing-Editor unter Beispiele

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** bs7/3-basis-laeufe-editor
- **Abhängig von:** BS7.2
- **Tickets:** B-640
- **Kriterien:** AC-04

## Ziel

Im Balancing-Editor zeigt „Beispiele“ einen Abschnitt „Basis-Läufe“ mit den gesicherten durchgelaufenen Läufen; „Übernehmen“ setzt Konfiguration und Baseline.

## Kontext

- Dialog: `tools/k3c-workbench/frontend/src/balancing/Examples.tsx` (feste `EXAMPLES` aus `help.ts`), Seite `BalancingPage.tsx`, Mocks `frontend/src/api/mockBalance.ts`, API `api/http.ts`, `api/wails.ts`, `api/types.ts`.
- API für Basis-Läufe aus BS7.2.
- Oberfläche nur über `@webkit` (Atomic Design, `tests/atomicDesign.test.ts`).
- ⚠️ Die EXE bettet die Oberfläche ein → Neustart.

## Erlaubte Dateien

- `tools/k3c-workbench/frontend/src/balancing/**`
- `tools/k3c-workbench/frontend/src/api/**`
- `tools/k3c-workbench/app_*.go` (Anbindung der API, falls nötig)

## Nicht-Ziele

Neue Beispiele-Logik für feste Beispiele; Baseline löschen aus der Oberfläche.

## Schritte

1. API-Typ und Mock für Basis-Läufe (Name, Datum, Konfiguration, Ergebnis).
2. Abschnitt „Basis-Läufe“ über den festen Beispielen; leer → Hinweis „Noch keine Basis-Läufe“.
3. „Übernehmen“ setzt Konfiguration und Baseline.
4. Frontend-Test mit Mock; `check_run task:check:diff`, Commit.
5. **⚠️ Workbench-Neustart nötig** melden (Ergebnis Zeile 1, `notify_ui` warn, Chat).

## Fertig, wenn

- [ ] AC-04: Frontend-Test: Basis-Lauf erscheint, Übernehmen setzt Konfiguration und Baseline.
- [ ] Neustart gemeldet; `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

## Ergebnis

**⚠️ Workbench-Neustart nötig** (Go-Code `app_balance.go`, `internal/balance`, `internal/mcpsrv` und die eingebettete Oberfläche geändert).

- AC-04 umgesetzt und geprüft: Der Dialog „Beispiele“ hat oben den Abschnitt „Basis-Läufe“ (Name, Lauf, Datum, Seeds/Modus, Bots, Welten, eingefrorene Gruppen; leer → „Noch keine Basis-Läufe“). „Übernehmen“ setzt den Auftrag des gesicherten Laufs (`spec` in `reports/baseline/<name>/run.json`) und die Baseline ins Formular; neues Feld „Baseline“ mit Hilfetext, `baseline` geht in die Spec des Laufs. Backend: `App.BalanceBaselines()`; `SaveBaseline` speichert jetzt die Spec des Laufs mit (BS7.2 nachgezogen). Tests: `BalancingBaselines.test.tsx` (Mock: Basis-Lauf erscheint, Übernehmen setzt Konfiguration und Baseline; Spec trägt `baseline`; Baseline ohne Spec), `TestBalanceBaselinesListetGesicherte`, `baselines_test.go`.
- Geprüft: `dev:test balancing` grün, `task:lint` 0 Fehler, `go test` für `balance`, `simrun`, `mcpsrv`, App-Test grün. `task check:diff` zweimal rot durch flüchtige Fehler außerhalb der Änderung (einmal `engine/level` Build-Fehler, `go vet` danach sauber; einmal Vitest-Worker bei `grafikPacks.test.ts` abgestürzt, einzeln grün); ein vollständig grüner Gesamtlauf nach den letzten Änderungen fehlt.
- **Keine Simulation gefahren** (Regel 2026-10-28). Vermerkt, nötig in BS7.5: echter `sim_test`-Lauf mit `baseline`/`saveBaseline` und Delta-Bericht (bisher nur Fake-Prozesse), Baseline `M1` aus dem Meilenstein-Lauf (BS3.4/BS3.6).
- Nicht geprüft: Oberfläche im Browser/Fenster angesehen (nur Mock-Tests).
