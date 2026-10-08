# PL2 · PLAT · Werkzeug-Seiten in der gewählten Sprache

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** PLAT
- **Prio:** niedrig
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-322
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Werkzeug-Seiten haben ihre Texte als deutsche Literale (B-322); PL1 übersetzt nur Touch-Overlay und Shell (B-215).

## Ziel

Jede Werkzeug-Seite folgt der gewählten Sprache mit Rückfall auf Deutsch. Am Ende sichtbar: Die Seiten unter `src/tools/` zeigen Deutsch oder Englisch nach Wahl.

## Beteiligte und Zielgruppen

🧑 und Mitspielende an PC, Handy und TV; Agent baut in `src/tools/`.

## Anforderungen

B-322 › Anforderungen.

## Nicht-Ziele

Weitere Sprachen, Texte außerhalb von `src/tools/`.

## Regeln und Einschränkungen

Einschiebbar; nach PL1. Beschluss 🧑 2026-10-06 (Chat) zu B-215: Werkzeug-Seiten werden übersetzt. Seitenweise Sessions (≤ ~400 Code-Zeilen).

## Beispiele

Sprache Englisch → `monitor.html` mit englischen Beschriftungen.

## Ausnahme- und Fehlerfälle

Text fehlt in `texts.en.ts` → deutscher Text.

## Akzeptanzkriterien

- **AC-01** Die Werkzeug-Seiten holen ihre Texte aus den zentralen Textdateien (B-322/AC-01, B-322/AC-02).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- PL2.1 Text-Regel für `src/tools/` und erste Seiten (Level-Betrachter, Hörprobe) (AC-01).
- PL2.2 Restliche Werkzeug-Seiten (Monitor, DM, Credits, Grafiken) (AC-01).
- PL2.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
