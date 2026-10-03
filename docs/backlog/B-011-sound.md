# B-011 · Spiel hat Sound und Musik

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** SO1
- **Erstellt:** 2026-09-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Spiel hat keinen Ton.

## Ziel

Spiel hat Sound und Musik. Nutzen: Rückmeldung und Stimmung, besonders nachts.

## Beteiligte und Zielgruppen

Spieler am TV (Edge auf der Xbox) und am Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Soundeffekte für wichtige Ereignisse.
- Musik je Tageszeit.
- Lautstärke einstellbar.

## Nicht-Ziele

Eigene Kompositionen.

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots und rechnet nichts; Seiten-Regeln aus `CLAUDE.md`; Taste B nicht belegen; 2 Spieler gleichzeitig (Split-Screen). Quellen: Kenney Audio / freesound.org (CC), Credits wie bei Grafiken.

## Beispiele

Die Nacht beginnt → die Musik wechselt; eine Münze fällt → Soundeffekt.

## Ausnahme- und Fehlerfälle

Browser blockiert Audio bis zur ersten Eingabe → Ton startet nach der ersten Taste, kein Fehler.

## Akzeptanzkriterien

- **AC-01** Wichtige Ereignisse haben Sound.
- **AC-02** Die Musik wechselt je Tageszeit.
- **AC-03** Die Lautstärke ist einstellbar.

## Offene Fragen

Welche Ereignisse sind „wichtig“? (🧑)

## Notizen

–
