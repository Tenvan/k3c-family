# M11 · SRV · MCP-Seite: alle Tools mit Statistik, Zeitfilter

- **Status:** aktiv
- **Projekt:** –
- **Domäne:** SRV
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-350
- **Start-Commit:** edb2a8f
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑, Revision 1

## Ausgangslage

Die MCP-Seite der Workbench zeigt keine Tool-Liste mehr, die Statistik hat keinen Zeitfilter (B-350).

## Ziel

Die Übersicht zeigt alle Tools mit kleiner Aufruf-Statistik, die Statistik filtert nach 15 min / 1 h / 24 h / 7 T.

Am Ende sichtbar: Workbench, Reiter MCP.

## Beteiligte und Zielgruppen

🧑 und Agenten in der Workbench.

## Anforderungen

B-350 › Anforderungen.

## Nicht-Ziele

Neue Messgrößen im Go-Server, `sim_test` (TR1).

## Regeln und Einschränkungen

Nur `tools/k3c-dev/frontend/`; einschiebbar neben TR1 (gleiche Domäne, andere Dateien).

## Beispiele

B-350 › Beispiele.

## Ausnahme- und Fehlerfälle

B-350 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Tool-Liste mit Statistik in der Übersicht (B-350/AC-01).
- **AC-02** Zeitfilter der Statistik (B-350/AC-02).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| M11.1 | `M11.1-tools-zeitfilter.md` | Umsetzung | autonom | fertig |
| M11.2 | `M11.2-review.md` | Review | autonom | in Arbeit |

## Abnahme

–
