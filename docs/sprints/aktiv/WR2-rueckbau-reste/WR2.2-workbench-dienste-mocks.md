# WR2.2 · Workbench: Dienste-Texte, Mocks und Fixtures ohne Spielserver und Vite

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/wr2
- **Abhängig von:** WR2.1
- **Tickets:** B-897
- **Kriterien:** AC-01, AC-04, AC-08

## Ziel

Dienste-Infrastruktur der Workbench bleibt generisch; Texte zum Port-Versatz, Frontend-Mocks und Fixtures nennen weder Spielserver noch Vite.

## Kontext

B-897 › Anforderungen; Default der offenen Frage: Dienste und `svc_*` bleiben.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/mcpsrv/**`, `tools/k3c-workbench/internal/services/**`, `tools/k3c-workbench/internal/studioview/**`
- `tools/k3c-workbench/frontend/src/api/**` und die Tests, die deren Mock-Daten prüfen
- `tools/checkdiff/**`

## Nicht-Ziele

- Dienste-Infrastruktur entfernen; Test-Fixtures mit „Spielserver“ als erfundenem Dienstnamen.

## Schritte

1. Kommentare zum Port-Versatz und zu Diensten in Go nachziehen.
2. Frontend-Mocks (Dienste, Tasks, Logs, Planung) auf erfundene generische Dienste umstellen.
3. Fixtures in `tools/checkdiff` ohne `engine/net`.

## Fertig, wenn

- AC-01, AC-04 erfüllt, `task check:diff` grün.

## Prüfen

- `check_run task:check:diff` (Worktree).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
