# B-555 · Entscheidung S2: Nach dem Fall der Burg bleibt die Musik still, das Siegstück wartet auf einen Sieg-Auslöser

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** KR4
- **Projekt:** SND
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Musik-Automat (SO4.2) kennt einen Zustand „Ende“ für Niederlage und Sieg. Ausgelöst wird er heute nur durch `castleFallen` (Niederlage); einen Sieg-Auslöser liefert die Simulation erst mit K2/K4. Im Slot `music:ende` liegt nur ein Siegstück (8-Bit Victory Loop, Wolfgang_, CC0), für die Niederlage gibt es keinen Kandidaten.

## Ziel

🧑 entscheidet, was nach dem Fall der Burg klingt.

## Beteiligte und Zielgruppen

🧑 entscheidet (Geschmack).

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

Nur CC0 oder CC-BY (Q15); kindgerecht.

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen A, B oder C gewählt.

## Offene Fragen

Siehe Notizen.

## Notizen

**Frage:** Welche Musik läuft im Zustand „Ende“?

- **A (Default, gilt, umgesetzt):** Niederlage = Stille (`MUSIC_SLOTS.ending = null` in `src/audio/music.ts`); das Siegstück bleibt im Slot für den späteren Sieg.
- **B:** das Siegstück auch bei Niederlage (eine Zeile: `ending: 'music:ende'`).
- **C:** zwei Zustände Sieg und Niederlage mit eigenem Niederlage-Stück (Kandidatensuche, Slot `music:niederlage`, Automat trennt `victory` und `castleFallen`).
- **Wenn 🧑 B oder C wählt:** B eine kleine Änderung; C Ticket CLI in SND mit Kandidatensuche.
