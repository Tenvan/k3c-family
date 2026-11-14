# B-784 · Planungsangaben für Wellen werden geprüft: plan_check und Regel-Test für freigegebene Tickets

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ44
- **Projekt:** WZG
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Die Planungsangaben für Wellen (`Bereich`, `Abhängig von`, `Braucht 🧑`, `Größe`, `Sperre`) sind eine Konvention im Abschnitt „Regeln und Einschränkungen“. Niemand prüft, ob ein freigegebenes Ticket sie trägt; 35 Tickets haben `Umgebung: ?`.

## Ziel

Fehlende oder ungültige Planungsangaben fallen vor der Freigabe auf.

## Beteiligte und Zielgruppen

Agenten beim Planen; 🧑 bei der Freigabe.

## Anforderungen

- `plan_check {id}` nennt je Ticket oder Sprint die Mängel: `Bereich` leer, `Umgebung: ?`, `Größe: L`, unbekannte ID in `Abhängig von`, Überlappung mit anderen offenen Tickets.
- Regel-Test in `tests/planning.test.ts`: Ein Ticket mit `Spec: freigegeben`, das nach dem Stichtag freigegeben wurde, trägt die Angaben.
- Bestand bleibt gültig; `plan_check` listet, was fehlt.

## Nicht-Ziele

Angaben als Kopffelder (später, wenn die Konvention sich bewährt hat).

## Regeln und Einschränkungen

Domäne DEV, Test in INF-Datei `tests/planning*.ts`.

- **Bereich:** `tools/k3c-workbench/internal/planning/**`, `tools/k3c-workbench/internal/mcpsrv/tools_planning*.go`, `tests/planning.test.ts`, `tests/planningDocs.ts`
- **Abhängig von:** B-783
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Ticket mit `Spec: freigegeben` ohne `Bereich` → `plan_check` meldet „Bereich fehlt“.

## Ausnahme- und Fehlerfälle

Archivierte Tickets werden nicht geprüft.

## Akzeptanzkriterien

- **AC-01** `plan_check` meldet die fünf Mängel an einem Test-Repo (Go-Test).
- **AC-02** Der Regel-Test schlägt bei einem neu freigegebenen Ticket ohne `Bereich` fehl und lässt den Bestand grün.

## Offene Fragen

Stichtag für den Regel-Test: Default (Claude, 2026-11-12, Meldung im Nachgang) Datum des Merges von B-783 (Sprint WZ44).

## Notizen

–
