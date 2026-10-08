# DBG4 · PLAT · Dev-Seite Asset-Vorschau im Spielmaßstab

- **Status:** geplant
- **Projekt:** GRA
- **Domäne:** PLAT
- **Prio:** mittel
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-299
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Assets lassen sich nur einzeln ansehen, nicht im Zusammenhang (B-299).

## Ziel

🧑 prüft die Asset-Wahl je Kategorie in einer Mini-Szene im Spielmaßstab. Am Ende sichtbar: `assetvorschau.html` zeigt je Kategorie eine Mini-Szene aus dem Slot-Register.

## Beteiligte und Zielgruppen

🧑 wählt; Agent baut die Seite.

## Anforderungen

B-299 › Anforderungen.

## Nicht-Ziele

Zuordnung bearbeiten (M10).

## Regeln und Einschränkungen

Einschiebbar; nach M10 (Slot-Register). Seite in `src/landing/pages.ts`, `installPageChrome()`.

## Beispiele

Kategorie „Gebäude“ → Mini-Hub mit aktiven Sprites.

## Ausnahme- und Fehlerfälle

Slot ohne Asset → Platzhalter mit Slot-Namen.

## Akzeptanzkriterien

- **AC-01** Eine Dev-Seite zeigt die Asset-Zuordnung je Kategorie als Mini-Szene im Spielmaßstab (B-299/AC-01, B-299/AC-02, B-299/AC-03, B-299/AC-04).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- DBG4.1 Seite mit Mini-Szenen je Kategorie (AC-01).
- DBG4.2 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
