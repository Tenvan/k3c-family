# B-320 · Der Reiter sitzt beim Laufen und Sprinten auf dem Sattel, nicht auf der Kruppe

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** GR7
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Abnahme S7.3 am 2026-10-06 (🧑, PC, Tastatur, 1 Spieler): Im Stand sitzt der Monarch mittig auf dem braunen Pferd, beim Laufen und Sprinten „hinten auf dem Hintern des Pferds“. Befund in den Daten: `data/sprites.json` › `mounts.horse.saddle` hat für `idle` den Punkt x = 49, für `run` x = 38 bis 42 (Faktor `scale` 2,2, also rund 15 bis 24 px weiter hinten). `blackHorse` nutzt dieselben Punkte. Die Lage berechnet `src/scenes/mountPose.ts` (`saddleOf`, Ursprung des Sheets).

## Ziel

Der Reiter sitzt in jeder Animation (Stehen, Laufen, Sprint) auf dem Sattel. Nutzen: Das Reittier wirkt wie ein Reittier, nicht wie ein Fehler.

## Beteiligte und Zielgruppen

Alle Spieler; 🧑 bei der Abnahme S7.3.

## Anforderungen

- Sattelpunkte je Frame von `horse` und `blackHorse` stimmen mit dem Sattel im Bild überein.
- Ursache klären: falsche Punkte in den Daten oder abweichender Ursprung bzw. Frame-Breite zwischen Lauf- und Steh-Frames.

## Nicht-Ziele

Beine des Reiters (nur Oberkörper, so in S7.2 vorgesehen); andere Reittiere als Standard (eigene Prüfung, falls nötig).

## Regeln und Einschränkungen

`src/scenes` rechnet nichts am Spiel; Darstellung als reine, getestete Funktion (`mountPose.ts`).

## Beispiele

Monarch läuft nach rechts → Oberkörper bleibt über dem Sattel wie im Stand, auch beim Wechsel Stehen → Laufen kein sichtbarer Sprung nach hinten.

## Ausnahme- und Fehlerfälle

Nicht relevant: Rückfall ohne Sattelpunkt regelt `mountPose.ts` schon.

## Akzeptanzkriterien

- **AC-01** Ein Test belegt: Für `horse` liegt der Sattelpunkt jedes `run`-Frames höchstens 4 px (Sheet-Pixel) vom `idle`-Punkt entfernt, sofern das Bild den Sattel dort hat; Abweichungen sind im Test begründet.
- **AC-02** 🧑 sieht am PC den Reiter beim Stehen, Laufen und Sprinten auf dem Sattel (Beobachtung).

## Offene Fragen

keine

## Notizen

Anlass: S7.3, Ergebnis 2026-10-06.
