# B-401 · Der Backlog-Filter wirkt nur auf den Backlog, der Projekt-Filter nur auf die Projekte

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ2
- **Projekt:** WZG
- **Erstellt:** 2026-10-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-11, 🧑 im Chat (Sprint WZ2, Revision 2)

## Ausgangslage

Wunsch von 🧑 (2026-10-11): Auf der Planungsseite gilt ein gemeinsamer Filter (`FilterBar`, `planning.filter` in `tools/k3c-dev/frontend/src/planning/SprintsBacklog.tsx`) für Sprints, Backlog und Projekte. Der Backlog-Filter soll nur den Backlog betreffen und der Projekt-Filter nur die Projekte; heute geben `SprintsBacklog.tsx` die gefilterten Sprints und Tickets an `ProjectsView` weiter („Projekte zeigen nur diese“).

## Ziel

Der Backlog hat einen eigenen Filter, der nur die Backlog-Liste einschränkt; die Projekte haben einen eigenen Filter, der nur die Projekte einschränkt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **Backlog-Filter** (Backlog-Liste und Sprint-Liste der Sprint-/Backlog-Ansicht): Prio, Umgebung, Projekt und Suche (🧑 2026-10-11). Die Sprint-Liste folgt dem Backlog-Filter.
- **Projekt-Filter** (nur Projekte): Status und Domäne (🧑 2026-10-11).
- Eine Änderung des einen Filters verändert den anderen Bereich nicht; die Detailauswahl bleibt unberührt.
- Beide Filter werden gemerkt wie bisher (`planning.filter`), getrennt je Bereich.
- Die Trefferzahl nennt je Bereich nur dessen Treffer.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Der Backlog-Filter (Prio, Umgebung, Projekt, Suche) ändert die Backlog-Liste und die Sprint-Liste, nicht die Projekte (Test).
- **AC-02** Der Projekt-Filter (Status, Domäne) ändert nur die Projekte, nicht die Backlog- und die Sprint-Liste (Test).
- **AC-03** Beide Filter bleiben nach Neuladen erhalten, getrennt je Bereich (Test mit Mock-Speicher).

## Offene Fragen

–  (Entschieden von 🧑 am 2026-10-11: Projekt-Filter Status und Domäne; Backlog-Filter Prio, Umgebung, Projekt und Suche; die Sprint-Liste folgt dem Backlog-Filter.)

## Notizen

Aufgenommen in Sprint WZ2 (Revision 2, 🧑 2026-10-11), Session WZ2.7.
