# B-039 · Bewegungen laufen trotz Snapshots flüssig

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** SP08
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat-Freigabe durch Ralf (mit SP02)

## Ausgangslage

Ab SP08 zeichnet der Browser Server-Snapshots (30 Hz) statt selbst zu rechnen.

## Ziel

Bewegungen laufen trotz Snapshots flüssig. Nutzen: Snapshots mit 30 Hz ruckeln ohne Interpolation.

## Beteiligte und Zielgruppen

Spieler am TV (Edge auf der Xbox) und am Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Interpolation zwischen Snapshots.
- Höchstens die eigene Laufbewegung wird lokal vorhergesagt.

## Nicht-Ziele

Vorhersage für fremde Figuren.

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots und rechnet nichts; Seiten-Regeln aus `CLAUDE.md`; Taste B nicht belegen; 2 Spieler gleichzeitig (Split-Screen).

## Beispiele

Bei 30 Hz und 100 ms Latenz → Figuren laufen am TV flüssig.

## Ausnahme- und Fehlerfälle

Snapshot fällt aus → Figuren bleiben stehen statt zu springen.

## Akzeptanzkriterien

- **AC-01** Bei 30 Hz und 100 ms Latenz laufen die Figuren am TV flüssig (Beobachtung durch 🧑).

## Offene Fragen

Ist die Vorhersage der eigenen Laufbewegung nötig? (nach der Messung)

## Notizen

–
