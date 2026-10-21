# B-518 · Agentische Sessions laufen schneller: weniger Reviews und Prüfläufe, mehr parallel

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Agentische Sessions brauchen oft eine Stunde und mehr. Auswertung der Transkripte (2026-10-21): In einer Session kosteten 224 Planungs-Aufrufe rund 63 min, obwohl k3c-dev je Aufruf nur etwa 6 ms rechnet (vermutet: Freigabe-Prüfung des Auto-Modus je Aufruf). Dazu kamen 15 Reviewer, 6× `check:go` (je 3:16 min wegen `go:race`) und 7× `check`. Außerdem liefen 249 automatische Security-Reviews des Plugins `security-guidance`. Reviews, Prüfläufe und Agenten liefen meist nacheinander.

## Ziel

Eine einfache Session ist in Minuten fertig: Prüfungen einmal und nach Diff, Review nur bei Risiko und parallel, Planung in wenigen Aufrufen, unabhängige Arbeit parallel.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `task check:diff` wählt die Prüfungen nach dem Diff zu `origin/develop`; `check:go` ohne Race-Detector, dafür `task check:race`; `check` läuft parallel.
- k3c-dev: `plan_sections` (mehrere Abschnitte in einem Aufruf), `plan_next` (nächste Session samt Kriterien und „Parallel möglich“), `check_run`-Ziele `task:check:diff` und `task:check:race`.
- Regeln: `docs/arbeitsweise.md` › Tempo im Lauf; CLAUDE.md: Review vor dem Commit nur bei Risiko, Lesepflichten nach Bedarf.
- Projekt-Skill `k3c-session` mit dem schlanken Ablauf; Allowlist für `mcp__k3c-dev` und `task check*`.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `task check:diff` führt bei einer reinen Doku-Änderung nur `check` aus, bei Go-Änderungen zusätzlich `check:go`, bei `tools/k3c-dev` `check:dev`.
- **AC-02** `task check:go` enthält kein `go:race`; `task check:race`, `check:all` und die CI prüfen den Race-Detector weiter.
- **AC-03** `plan_sections` schreibt mehrere Abschnitte in einem Aufruf und schreibt nichts bei einem unbekannten Abschnitt (Go-Test).
- **AC-04** `plan_next` liefert die Session nach Rang, Abhängigkeit und Domänen-Sperre sowie „Parallel möglich“ (Go-Test).
- **AC-05** `docs/arbeitsweise.md`, `CLAUDE.md`, Session-Vorlage und Skill `k3c-session` beschreiben Prüfstufen, Review bei Risiko, parallele Spuren und Polling-Verbot gleichlautend.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Umgesetzt 2026-10-21 (Branch `inf/agentic-tempo`): AC-01 bis AC-05 umgesetzt, geprüft mit `task check:diff` und Go-Tests in `tools/k3c-dev` (`next_test.go`, `write_test.go`). `check:diff` läuft immer `check` (Repo-Regeln scannen auch Go und Frontends), `engine/`/`data/` ziehen zusätzlich k3c-dev, Studio und Starter nach. Das Plugin `security-guidance` hat 🧑 für das Projekt abgeschaltet; dafür ist Shell-, Pfad- und Eingabe-Code ein Review-Auslöser. Folge: `plan_next` gegen `origin` (B-519).
