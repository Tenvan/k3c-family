# B-392 · Im Split-Screen scrollt nur die Kamera von Spieler 1 mit; Spieler 2 ist nicht an seine Ansicht gebunden

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** BD1
- **Projekt:** BED
- **Erstellt:** 2026-10-10
- **Spec:** freigegeben
- **Revision:** 4
- **Freigabe:** 2026-10-14, Chat 🧑 (Revision 4)

## Ausgangslage

Beobachtung 🧑 in der Abnahme S8.4 (2026-10-10): Im Split-Screen mit 2 lokalen Spielern scrollt nur die Ansicht von Spieler 1 mit seiner Figur; die Ansicht von Spieler 2 folgt ihrer Figur nicht. Die Kamera-Logik liegt in `GameScene.layoutCameras`/`aimAtStage` (`src/scenes/GameScene.ts`). Vermutete Ursache (ungeprüft, BD1.1 Schritt 2): `startFollow` wird nur gesetzt, wenn `playerView(monarch)` beim Layout schon eine View liefert; die View von Spieler 2 entsteht erst im folgenden `sync`, und das Layout wird nicht wiederholt.

## Ziel

Jeder Spieler hat im Split-Screen eine eigene Ansicht, die seiner Figur folgt.

## Beteiligte und Zielgruppen

Spielende im Couch-Koop (höchstens 2 Spieler); 🧑 meldete den Fehler.

## Anforderungen

- Jede Spieler-Ansicht folgt der Figur ihres Sitzes, mit 2 lokalen Spielern gleichzeitig.
- Das gilt auch, wenn die Figur erst nach dem Aufbau der Kameras erscheint (Beitritt, Stufenwechsel, Neuaufbau des Layouts).
- Ein Test belegt das Verhalten; die Abnahme am PC bleibt bei 🧑.

## Nicht-Ziele

Neues Split-Screen-Layout; Änderungen an Server oder Protokoll; 3–4 Spieler (entfallen seit 2026-10-11); Umbau der Partner-Kamera, außer die Ursache liegt dort.

## Regeln und Einschränkungen

CLI zeichnet nur Snapshots (`CLAUDE.md`); jede Mechanik muss mit 2 Spielern gleichzeitig funktionieren.

## Beispiele

Spieler 1 läuft nach rechts, Spieler 2 nach links → jede Ansicht scrollt mit ihrer Figur.

## Ausnahme- und Fehlerfälle

nicht relevant: Das Ticket beschreibt selbst den Fehlerfall.

## Akzeptanzkriterien

- **AC-01** Zwei Spieler an einer Tastatur (A/D und Pfeile) laufen in verschiedene Richtungen: Beide Ansichten folgen ihrer Figur.

## Offene Fragen

keine. 🧑 bestätigt (S8.4): Der Fehler tritt immer auf, nicht nur in einer bestimmten Eingabe-Kombination. Entschieden 2026-10-11 (🧑, Chat): 3 und 4 Spieler entfallen; es zählen höchstens 2 Spieler.

## Notizen

Aus S8.4 (Browser-Abnahme).
