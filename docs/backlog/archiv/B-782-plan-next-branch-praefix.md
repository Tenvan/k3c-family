# B-782 · plan_next nennt den Sprint-Branch richtig, auch wenn das Feld Branch schon mit sprint/ beginnt

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ44
- **Projekt:** WZG
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

`tools/k3c-workbench/internal/planning/next.go` (pickText) schneidet das Session-Feld `Branch` am ersten `/` ab. Bei `Branch: sprint/cu1` (CU1, NV1, RS2) entsteht „Branch: sprint/sprint“, und das Origin-Overlay sucht `origin/sprint/sprint` statt `origin/sprint/cu1`. Der Stand und die Sperre dieser Sprints stimmen dann nicht.

## Ziel

`plan_next` nennt für jede Session den richtigen Sprint-Branch und liest den Stand vom richtigen `origin/sprint/<präfix>`.

## Beteiligte und Zielgruppen

Agenten, die `plan_next` nutzen; 🧑 liest die Antwort.

## Anforderungen

- Ein Wert mit führendem `sprint/` wird unverändert übernommen, sonst gilt `sprint/` + Teil vor `/`.
- Dieselbe Ableitung gilt im Origin-Overlay (`next_origin.go`).

## Nicht-Ziele

Wellenberechnung (B-783).

## Regeln und Einschränkungen

Domäne DEV, Komplexitäts-Budget.

- **Bereich:** `tools/k3c-workbench/internal/planning/next.go`, `tools/k3c-workbench/internal/planning/next_origin.go`, `tools/k3c-workbench/internal/planning/next_test.go`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

`Branch: sprint/cu1` → `sprint/cu1`; `Branch: w15/1-fackeln` → `sprint/w15`.

## Ausnahme- und Fehlerfälle

Feld leer oder ohne `/` → `sprint/` + Sprint-ID in Kleinbuchstaben (wie heute).

## Akzeptanzkriterien

- **AC-01** Test in `next_test.go`: `sprint/cu1` ergibt `sprint/cu1`, `w15/1-x` ergibt `sprint/w15`.
- **AC-02** Das Overlay meldet für einen Sprint mit `Branch: sprint/<präfix>` den Stand von `origin/sprint/<präfix>` (Test).

## Offene Fragen

keine

## Notizen

Befund aus der Planungsanalyse 2026-11-11 (Session k3c-03).
