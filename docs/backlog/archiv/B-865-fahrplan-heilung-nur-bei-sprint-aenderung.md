# B-865 · Die Fahrplan-Heilung schreibt nur bei Sprint- oder Session-Änderungen und erzeugt keine Konflikte zwischen parallelen Worktrees

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ54
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

WZ49.3: `refreshRoadmap` schreibt `docs/sprints/README.md` bei jedem schreibenden `plan_*`-Aufruf, sobald sich etwas ändert. In parallelen Worktrees kann das Merge-Konflikte am Fahrplan erzeugen (vermutet).

## Ziel

Die Heilung läuft nur bei Sprint- oder Session-Änderungen (oder als eigener Aufruf), nicht bei reinen Ticket-Änderungen.

## Beteiligte und Zielgruppen

Planende Agenten in Wellen; Claude (Umsetzung).

## Anforderungen

- Ticket-Änderungen schreiben den Fahrplan nicht.
- Sprint- und Session-Änderungen heilen wie bisher.

## Nicht-Ziele

Neue Fahrplan-Abschnitte.

## Regeln und Einschränkungen

Klasse A (Problem, DEV).

- **Bereich:** `tools/k3c-workbench/internal/planning/**`
- **Abhängig von:** WZ49
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

`plan_set B-123 Prio hoch` → `docs/sprints/README.md` unverändert.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** Test: Ticket-Änderung lässt den Fahrplan unberührt, Sprint-Änderung heilt.

## Offene Fragen

keine

## Notizen

Befund WZ49.3 (2026-11-14).
