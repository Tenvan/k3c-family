# PL2 · PLAT · Werkzeug-Seiten in der gewählten Sprache

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** PLAT
- **Prio:** niedrig
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-322
- **Start-Commit:** 07f98fc5
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-08 Chat (PL2 vor PL1 vorgezogen)

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

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| PL2.1 | `PL2.1-text-regel-level-sound.md` | Umsetzung | autonom | fertig |
| PL2.2 | `PL2.2-monitor-dm-credits-grafiken.md` | Umsetzung | autonom | fertig |
| PL2.3 | `PL2.3-restliche-werkzeug-seiten.md` | Umsetzung | autonom | fertig |
| PL2.4 | `PL2.4-review.md` | Review | autonom | fertig |
| PL2.5 | `PL2.5-abnahme-sprachwechsel.md` | Umsetzung | Mensch | offen |

## Abnahme

Review 2026-10-08 (PL2.4, eigener Review-Agent): keine schweren Befunde, `task check` und `task check:go` grün.
- AC-01 angenommen (B-322/AC-01: Regeltest ohne `OFFEN`, 307 Schlüssel de/en gleich, Platzhalter gleich).
- B-322/AC-02 angenommen, Validierung offen (PL2.5, 🧑 am Gerät).
- Kleinbefund behoben: `isKey` prüft mit `Object.hasOwn`.
- Versionsvorschlag: Minor (neue Sprachwahl in den Werkzeug-Seiten). B-322 bleibt `eingeplant`, Sprint bleibt aktiv bis PL2.5.
