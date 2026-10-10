# B-345 · Der Raum tauscht die Insel bei `SwitchReady`

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** K4
- **Projekt:** KMP
- **Erstellt:** 2026-10-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-09, 🧑 im Chat, mit Sprint K4 Revision 3

## Ausgangslage

Seit K2.3a meldet die Simulation `Island.SwitchReady` und das Ereignis `islandSwitch`, sobald nach dem Endboss alle lebenden Spieler am Wechselpunkt stehen (`engine/sim/island_switch.go`). `NextIsland(cur, def)` baut die nächste Insel. `engine/room/` ruft das nicht auf: Der Raum bleibt auf der alten Insel.

## Ziel

Der Raum wechselt gemeinsam auf die nächste Insel (`docs/rules/stufen.md` § 1, B-103).

## Beteiligte und Zielgruppen

Spieler (Koop), SRV; 🧑 gibt die Spec frei.

## Anforderungen

- Ist `SwitchReady` gesetzt, ersetzt der Raum seine Insel im selben Takt durch `NextIsland(isl, def)` mit `def, _ := isl.NextDef()` und behält Geräte und Slots.
- Die Geräte bekommen danach einen vollen Zustand der neuen Stufe; das Protokoll dazu legt K4 fest.
- Der Spielstand speichert die neue Insel (Format: K2.3b).

## Nicht-Ziele

Anzeige des Wechsels (K5), Inhalt weiterer Inseln, Protokollfelder (K4).

## Regeln und Einschränkungen

Domäne SRV (`engine/room/`); `engine/sim/` bleibt unverändert. Deterministisch: kein eigener Zufall im Raum.

## Beispiele

Endboss besiegt → beide Spieler stehen 2 s an der Burg der tiefsten Stufe → der Raum läuft auf Insel 2, beide Spieler an der Burg der ersten Stufe.

## Ausnahme- und Fehlerfälle

`NextIsland` scheitert (Daten kaputt) → Raum bleibt auf der alten Insel und loggt den Fehler mit ❌.

## Akzeptanzkriterien

- **AC-01** Test in `engine/room/`: nach `SwitchReady` tickt der Raum die neue Insel, Spieler und Geräte bleiben zugeordnet.

## Offene Fragen

keine

## Notizen

Angelegt in K2.3a (Sprint K2).
