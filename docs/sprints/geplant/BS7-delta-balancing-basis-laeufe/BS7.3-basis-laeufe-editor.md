# BS7.3 · Basis-Läufe im Balancing-Editor unter Beispiele

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
