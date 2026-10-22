# B-400 · Die Planungsseite trennt Backlog und Details mit einem Splitter, dessen Stellung gemerkt wird

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

Wunsch von 🧑 (2026-10-11): Auf der Planungsseite (`tools/k3c-workbench/frontend/src/planning/`, `SprintsBacklog.tsx`) soll zwischen Backlog und Details ein verschiebbarer Splitter sitzen, dessen Stellung beim nächsten Öffnen erhalten bleibt. Die Tasks-Seite hat dafür schon `Splitter` in `tools/k3c-workbench/frontend/src/tasks/TaskBars.tsx` samt gemerktem Anteil; die Planungsseite merkt heute nur den Filter (`planning.filter`).

## Ziel

Die Breite von Backlog und Detailbereich lässt sich per Splitter ändern, und die Stellung bleibt über Neuladen und Neustart erhalten.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Splitter zwischen Backlog und Details **nur in der Sprint-/Backlog-Ansicht**, nicht in der Projekt-Ansicht (🧑 2026-10-11).
- Per Ziehen bedienbar; Doppelklick setzt zurück (wie auf der Tasks-Seite).
- Es gibt **einen gemeinsamen Anteil**, der gemerkt (wie `planning.filter`, im Browser-Speicher der Workbench) und beim Öffnen wiederhergestellt wird; Grenzen 20–80 %.
- Vorhandenes `Splitter` aus `tasks/TaskBars.tsx` wiederverwenden statt neu bauen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Der Splitter steht zwischen Backlog und Details der Sprint-/Backlog-Ansicht und fehlt in der Projekt-Ansicht; Ziehen ändert die Breiten, Doppelklick setzt auf den Standard (Komponententest).
- **AC-02** Der eine gemeinsame Anteil bleibt nach Neuladen erhalten und liegt immer zwischen 20 und 80 % (Test mit Mock-Speicher, auch bei fehlendem oder ungültigem Wert).

## Offene Fragen

–  (Entschieden von 🧑 am 2026-10-11: Splitter nur in der Sprint-/Backlog-Ansicht, ein gemeinsamer Anteil.)

## Notizen

Aufgenommen in Sprint WZ2 (Revision 2, 🧑 2026-10-11), Session WZ2.7.
