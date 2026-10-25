# PG1 · PLAT · Präsentationsseite mit echten Spielbildern

- **Status:** aktiv
- **Projekt:** REL
- **Domäne:** PLAT
- **Reife:** bereit
- **Tickets:** B-184
- **Start-Commit:** 819c53d3
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25, Revision 1 (Ziel von Ralf: Specs genehmigt)

## Ausgangslage

Die Präsentationsseite zeigt keine echten Bilder aus dem Spiel (B-184).

## Ziel

Besucher sehen, wie das Spiel wirklich aussieht. Am Ende sichtbar: `site/` zeigt Split-Screen, Hub und eine Nacht als echte Bilder.

## Beteiligte und Zielgruppen

Besucher der GitHub-Pages-Seite; Agent erzeugt Bilder im Browser-Pane.

## Anforderungen

B-184 › Anforderungen.

## Nicht-Ziele

Video, neue Texte.

## Regeln und Einschränkungen

Einschiebbar; nur `site/`, Bilder komprimiert.

## Beispiele

Seite öffnen → drei Bilder mit Bildunterschrift.

## Ausnahme- und Fehlerfälle

Bild fehlt → Seite bleibt lesbar, Alt-Text.

## Akzeptanzkriterien

- **AC-01** Die Präsentationsseite zeigt echte Bilder aus dem Spiel (B-184/AC-01).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| PG1.1 | `PG1.1-bilder-einbinden.md` | Umsetzung | autonom | fertig |
| PG1.2 | `PG1.2-review.md` | Review | autonom | offen |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- PG1.1 Bilder aufnehmen und einbinden (AC-01).
- PG1.2 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
