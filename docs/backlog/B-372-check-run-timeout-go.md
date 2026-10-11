# B-372 · check_run go:test und task:check:go brechen im MCP-Aufruf mit Timeout ab

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ2
- **Projekt:** WZG
- **Erstellt:** 2026-10-09
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-11, 🧑 im Chat (Sprint WZ2, Revision 2)

## Ausgangslage

`check_run` mit `go:test` und `task:check:go` endet im MCP-Aufruf mit „The operation timed out.“ (2026-10-08 PJ3.4, 2026-10-09 K2.4), obwohl `task check:go` in der Shell grün durchläuft (`k3c-load` ~21 s, `engine/net` ~21 s, dazu Lint). Agenten müssen dann auf die Shell ausweichen. Zusätzlich lehnen laufende k3c-dev-Instanzen die Domäne `DEV` aus DV1 noch ab (alter Build), und `console_tail` hat andere Parameter als in den Server-Anweisungen beschrieben. Außerdem (2026-10-09): `plan_set` an Sprints verlangt noch das Feld `Prio` (seit B-361 entfernt, „unbekanntes Feld“), und `plan_create` lehnt Session-IDs mit Buchstaben wie `K3.2a` ab (K2 nutzt sie seit K2.1a).

## Ziel

Lange Prüfläufe liefern über `check_run` ein Ergebnis statt eines Timeouts (z. B. Lauf im Hintergrund mit Abfrage, oder Frist über der Dauer von `task check:go`).

## Beteiligte und Zielgruppen

Agenten in Review- und Umsetzungs-Sessions.

## Anforderungen

- `check_run` startet lange Läufe im Hintergrund und gibt eine Lauf-ID zurück, über die Exit-Code und Fehlerzeilen abgefragt werden (🧑 2026-10-11). Kurze Läufe liefern das Ergebnis wie bisher direkt.
- Die Server-Anweisungen nennen die aktuellen Parameter von `console_tail`.
- `plan_set` an Sprints verlangt kein Feld `Prio` mehr (seit B-361 entfernt).
- `plan_create` akzeptiert Session-IDs mit Buchstaben wie `K3.2a`.
- Ob der Timeout aus MCP-Client oder k3c-dev kommt, misst WZ2.4 und trägt es unter Notizen ein; der Hintergrundlauf gilt unabhängig davon.

## Nicht-Ziele

Go-Tests selbst beschleunigen.

## Regeln und Einschränkungen

Nur `tools/k3c-dev/`.

## Beispiele

`check_run task:check:go` nach ~90 s → `task:check:go · exit 0 · 90 s` statt Timeout.

## Ausnahme- und Fehlerfälle

Hängt ein Lauf wirklich, bricht er nach einer festen Frist mit klarer Meldung ab.

## Akzeptanzkriterien

- **AC-01** `check_run task:check:go` liefert Exit-Code und Fehlerzeilen (direkt oder über eine Lauf-ID), auch wenn der Lauf länger als 60 s dauert.
- **AC-02** Die Server-Anweisungen nennen die aktuellen Parameter von `console_tail`.
- **AC-03** `plan_set` an einem Sprint ohne Feld `Prio` ist erfolgreich (Go-Test).
- **AC-04** `plan_create` legt eine Session mit ID wie `K3.2a` an (Go-Test).

## Offene Fragen

–  (Lösungsweg entschieden von 🧑 am 2026-10-11: Hintergrundlauf mit Lauf-ID. Die Herkunft des Timeouts, MCP-Client oder k3c-dev, ist ungeprüft und wird in WZ2.4 gemessen; sie ändert den Lösungsweg nicht.)

## Notizen

Aus K2.4 (2026-10-09).
