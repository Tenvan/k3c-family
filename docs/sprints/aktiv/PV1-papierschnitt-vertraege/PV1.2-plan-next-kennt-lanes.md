# PV1.2 · plan_next und plan_* kennen Domäne PAP und sperren nach Lane

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** pv1/2-plan-next-kennt-lanes
- **Abhängig von:** PV1.1
- **Tickets:** B-699
- **Kriterien:** AC-02

## Ziel

`plan_next`, `plan_create`, `plan_set` und `plan_list` kennen die Domäne PAP und die Felder `Lane` und `Modell`; `plan_next` sperrt nach Domäne und Lane (PV1.1), nennt parallele Sessions auch innerhalb einer Domäne und gibt je Session das Modell aus.

## Kontext

- Auswahl-Logik: `tools/k3c-workbench/internal/planning/next.go` (`locked map[string]string` Domäne → Session in Arbeit; Parallel-Liste nur über andere Domänen, Zeile ~192). Domänenliste und Feldprüfung im Paket `internal/planning`; MCP-Hülle `internal/mcpsrv/tools_planning.go` (Beschreibung von `domain` nennt die Domänen).
- Regel aus PV1.1: Session mit Lane sperrt Domäne+Lane; Session ohne Lane sperrt die ganze Domäne und ist gesperrt, sobald irgendeine Session derselben Domäne in Arbeit ist.
- Abhängig von PV1.1 (Domäne und Feld in Vorlage und Arbeitsweise).

## Erlaubte Dateien

- `tools/k3c-workbench/internal/planning/` (Code und Tests)
- `tools/k3c-workbench/internal/mcpsrv/tools_planning.go` und Test

## Nicht-Ziele

Keine Änderung an Arbeitsweise oder Vorlagen (PV1.1), keine Oberfläche.

## Schritte

1. Tests in `internal/planning` (Fixture-Sprints im Testordner):
   - `TestNextLanesParallel`: zwei offene Sessions DEV/`mcp` und DEV/`ui` → Antwort wählt eine, „Parallel möglich“ nennt die andere.
   - `TestNextSameLaneLocked`: DEV/`mcp` in Arbeit → offene DEV/`mcp` nicht wählbar, Grund `Domäne DEV, Lane mcp gesperrt (… in Arbeit)`.
   - `TestNextNoLaneLocksDomain`: DEV ohne Lane in Arbeit → keine DEV-Session wählbar; DEV/`ui` in Arbeit → DEV ohne Lane nicht wählbar.
   - `TestNextShowsModel`: Session mit `Modell: opus` → Ausgabezeile enthält `Modell: opus`; ohne Feld `Modell: sonnet (Standard)`; parallele Sessions nennen ihr Modell.
   - `TestNextMaxFourParallel`: fünf freie Lanes → „Parallel möglich“ listet höchstens drei weitere (gesamt vier Agenten).
   - `TestDomainPAPValid`: `plan_create` mit Domäne PAP gelingt, `plan_set {Lane: "mcp", Modell: "opus"}` schreibt die Felder, ungültige Werte (`MCP!`, `gpt`) werden mit Wertebereich abgelehnt.
   Laufen lassen: rot.
2. Sperrschlüssel von Domäne auf `Domäne` bzw. `Domäne/Lane` umstellen; Parallel-Liste: je freiem Schlüssel die erste Session, höchstens drei. Ausgabezeile ergänzt `· Lane: x · Modell: y`.
3. PAP in die Domänenliste; Felder `Lane` und `Modell` beim Lesen, Prüfen und Schreiben; Tool-Beschreibungen anpassen.
4. Tests grün; Commit `feat(dev): plan_next sperrt nach Lane, nennt Modell, Domäne PAP`.

## Fertig, wenn

- [ ] AC-02: die sechs Tests grün.
- [ ] `plan_next` auf dem echten Repo liefert weiterhin die bisherige Auswahl plus Modell – Ausgabe im Ergebnis zitieren.

## Prüfen

```bash
task check:diff
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
