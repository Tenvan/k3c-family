# B-029 · Lade-Szene zeigt Fortschritt

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

## Offene Fragen

keine

## Notizen

–
