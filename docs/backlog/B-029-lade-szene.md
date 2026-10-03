# B-029 · Lade-Szene zeigt Fortschritt

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** GR4
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1, durch 🧑; mit Sprint GR4; mit Änderungen aus dem Spec-Review (Voraussetzungen, AC-06 Ladefehler, Texturgröße in GR4.3)

## Ausgangslage

Beim Start lädt das Spiel Sprites ohne Anzeige.

## Ziel

Lade-Szene zeigt Fortschritt. Nutzen: Auf der Xbox wirkt ein leerer Bildschirm wie ein Absturz.

## Beteiligte und Zielgruppen

Spieler am TV (Edge auf der Xbox) und am Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Lade-Szene mit Fortschrittsbalken, bis alle Assets geladen sind.

## Nicht-Ziele

Vorladen im Hintergrund während des Spiels.

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots und rechnet nichts; Seiten-Regeln aus `CLAUDE.md`; Taste B nicht belegen; 2 Spieler gleichzeitig (Split-Screen).

## Beispiele

Start auf der Xbox → ein Balken läuft statt eines schwarzen Bildes.

## Ausnahme- und Fehlerfälle

Ein Asset lädt nicht → Meldung statt ewig laufendem Balken.

## Akzeptanzkriterien

- **AC-01** Beim Start zeigt eine Lade-Szene den Fortschritt, bis alle Assets geladen sind.
- **AC-02** Lädt ein Asset nicht, zeigt die Lade-Szene eine Meldung mit Dateinamen statt eines hängenden Balkens.

## Offene Fragen

keine

## Notizen

–
