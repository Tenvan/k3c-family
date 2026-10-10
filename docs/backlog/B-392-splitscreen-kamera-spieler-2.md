# B-392 · Im Split-Screen scrollt nur die Kamera von Spieler 1 mit; Spieler 2 und weitere sind nicht an ihre Ansicht gebunden

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-10
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Beobachtung 🧑 in der Abnahme S8.4 (2026-10-10): Im Split-Screen mit mehreren Spielern scrollt nur die Ansicht von Spieler 1 mit seiner Spielfigur. Die Ansichten von Spieler 2 (und weiteren) folgen ihrer Figur nicht. Die Kamera-Logik liegt in `src/scenes/` (`GameScene`, Kameras je Spieler); die Ursache ist noch nicht untersucht.

## Ziel

Jeder Spieler hat im Split-Screen eine eigene Ansicht, die seiner Figur folgt.

## Beteiligte und Zielgruppen

Spielende im Couch-Koop (2–4 Spieler); 🧑 meldete den Fehler.

## Anforderungen

- Jede Spieler-Ansicht folgt der Figur ihres Spielers, mit 2 und mehr Spielern gleichzeitig.

## Nicht-Ziele

Neues Split-Screen-Layout; Änderungen an Server oder Protokoll.

## Regeln und Einschränkungen

CLI zeichnet nur Snapshots (`CLAUDE.md`); jede Mechanik muss mit 2 Spielern gleichzeitig funktionieren.

## Beispiele

Spieler 1 läuft nach rechts, Spieler 2 nach links → jede Ansicht scrollt mit ihrer Figur.

## Ausnahme- und Fehlerfälle

nicht relevant: Das Ticket beschreibt selbst den Fehlerfall.

## Akzeptanzkriterien

- **AC-01** Zwei Spieler an einer Tastatur (A/D und Pfeile) laufen in verschiedene Richtungen: Beide Ansichten folgen ihrer Figur. Gleiches mit 3 und 4 Spielern.

## Offene Fragen

keine. 🧑 bestätigt (S8.4): Der Fehler tritt immer auf, nicht nur in einer bestimmten Eingabe-Kombination.

## Notizen

Aus S8.4 (Browser-Abnahme).
