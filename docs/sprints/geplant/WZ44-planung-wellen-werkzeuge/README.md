# WZ44 · DEV, INF · Planungs-Werkzeuge für Wellen: plan_waves, plan_queue, plan_check, Sim-Sperre im Werkzeug

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV, INF
- **Reife:** bereit
- **Tickets:** B-782, B-788, B-783, B-784, B-785
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12 (Gruppe von Ralf zur Umsetzung freigegeben; B-782, B-785, B-788 Klasse A)

## Ausgangslage

Die Wellen-Regeln (`docs/arbeitsweise.md` › Parallelität und Wellen) rechnet der planende Agent bisher von Hand; `plan_next` sperrt nur nach Domäne und Lane und nennt bei `Branch: sprint/…` den falschen Sprint-Branch (B-782). `check_run` kennt die Teilaufgaben von `check:diff` nicht, und die Sim-Sperre (Beschluss 🧑 2026-11-11) steht nur als Regeltext (B-788). Der Fahrplan enthält fertige Sprints in `aktiv/` und Abnahmen je Sprint statt je Gerät (B-785); einen Teil davon hat die Bereinigung vom 2026-11-12 (Entscheidung 009) erledigt.

## Ziel

Ein Aufruf liefert die Welle, die jetzt parallel laufen kann, ein zweiter die Ralf-Queue nach Gerät; Planungsangaben werden geprüft, die Sim-Sperre gilt im Werkzeug, und der Fahrplan ist aufgeräumt.

Am Ende sichtbar: `plan_waves`, `plan_queue`, `plan_check` im MCP; `sim_test start` lehnt mit Hinweis auf den Beschluss ab; `docs/sprints/aktiv/` ohne Sprint ohne offene Session.

## Beteiligte und Zielgruppen

Claude (Umsetzung WZ44.1–WZ44.4, Review WZ44.5); planende Agenten und 🧑 (Ralf-Queue).

## Anforderungen

`B-782 › Anforderungen`, `B-788 › Anforderungen`, `B-783 › Anforderungen`, `B-784 › Anforderungen`, `B-785 › Anforderungen`. Sprint-eigen: Nach B-783 entfällt in `docs/arbeitsweise.md` der Hinweis „Bis `plan_waves` das übernimmt“ bzw. „Übergang, bis `plan_next` Dateimengen liest“.

## Nicht-Ziele

Ticket-Relationen und Index-Verdichtung (B-696, WZ47); Planungsseite der Oberfläche; Sim- oder Balance-Läufe.

## Regeln und Einschränkungen

Domäne DEV (WZ44.4 INF, nur Planungsdateien über `plan_*`); nur lesende neue Tools außer der Sperre; Tool-Beschreibungen enden mit „Nutze es bei: … Statt: …“; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neubau der Workbench-EXE zum Nutzen der Tools (Sperre `exe`).

- **Bereich:** `tools/k3c-workbench/internal/planning/**`, `tools/k3c-workbench/internal/mcpsrv/tools_planning*.go`, `tools/k3c-workbench/internal/mcpsrv/check_targets*.go`, `tools/k3c-workbench/internal/mcpsrv/simtest*.go`, `tools/k3c-workbench/internal/mcpsrv/tools_sim*.go`, `tools/k3c-workbench/internal/mcpsrv/instructions.md`, `tests/planning.test.ts`, `tests/planningDocs.ts`, `docs/arbeitsweise.md`, `docs/sprints/**`, `docs/projekte/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** exe

## Beispiele

Zwei Sprints mit Sperre `taskfile` → `plan_waves` setzt den aus dem Projekt mit größerem Rang in die nächste Welle und nennt den Grund. Ticket `Spec: freigegeben` ohne `Bereich` → `plan_check` meldet den Mangel.

## Ausnahme- und Fehlerfälle

Unbekannte ID in `Abhängig von` → `plan_check`-Mangel, `plan_waves` behandelt den Knoten als blockiert mit Grund. Schalter der Sim-Sperre gesetzt → `sim_test`/`sim_run` laufen wie bisher.

## Akzeptanzkriterien

- **AC-01** `plan_next` leitet den Sprint-Branch richtig ab, auch mit führendem `sprint/` (`B-782/AC-01`, `B-782/AC-02`).
- **AC-02** `check_run` nimmt die drei Teilaufgaben, Sim-Sperre im Werkzeug mit Schalter (`B-788/AC-01`, `B-788/AC-02`).
- **AC-03** `plan_waves`, `plan_queue`, Kollision in `plan_next`, MCP-Instruktionen (`B-783/AC-01` bis `B-783/AC-04`).
- **AC-04** `plan_check` und Regel-Test (`B-784/AC-01`, `B-784/AC-02`).
- **AC-05** Fahrplan aufgeräumt (`B-785/AC-01` bis `B-785/AC-03`).

## Offene Fragen

keine blockierenden. Defaults (Claude, 2026-11-12, Meldung im Nachgang): Gerät einer Mensch-Session aus einer Zeile `Gerät:` im Kontext, sonst „unbekannt“ (B-783); Stichtag des Regel-Tests = Datum des Merges von WZ44 (B-784); Schalter der Sim-Sperre ist die Umgebungsvariable `K3C_SIM_ENTSPERRT=1` der Workbench (B-788).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ44.1 | `WZ44.1-branch-checkrun-simsperre.md` | Umsetzung | autonom | offen |
| WZ44.2 | `WZ44.2-plan-waves-queue.md` | Umsetzung | autonom | offen |
| WZ44.3 | `WZ44.3-plan-check.md` | Umsetzung | autonom | offen |
| WZ44.4 | `WZ44.4-fahrplan-aufraeumen.md` | Umsetzung | autonom | offen |
| WZ44.5 | `WZ44.5-review.md` | Review | autonom | offen |

Keine eigenen Sim- oder Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
