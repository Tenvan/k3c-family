# B-396 · Der Monarch holt beim Schlag ohne Ziel sichtbar aus

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** S9
- **Projekt:** BED
- **Erstellt:** 2026-10-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-11, 🧑 im Chat, Revision 1

## Ausgangslage

Abnahme S9.5 am 2026-10-11 (🧑, PC, Tastatur, am Tag ohne Gegner): Beim Schlag ohne Ziel blinkt ein Kreis auf, der Monarch holt aber nicht aus (keine Figuren-Animation). B-318 nennt als Beispiel „Monarch holt kurz aus“; S9.3 zeichnet nur das Ereignis `strike` als Effekt (`src/scenes/effects.ts`, `effectsView.ts`).

## Ziel

Jeder Schlag, mit und ohne Treffer, zeigt eine Ausholbewegung des Monarchen. Nutzen: Die Taste wirkt wie ein Schlag, nicht nur wie ein Effekt-Kreis.

## Beteiligte und Zielgruppen

Alle Spieler; 🧑 bei der Abnahme. Grafik: sofern der Monarch-Sprite keine Angriffs-Animation hat, braucht es ein Bild (Credits beachten).

## Anforderungen

- Beim Ereignis `strike` eines Monarchen (mit `hit`, mit und ohne Treffer) spielt dessen Figur die Angriffs-Animation (`attack` der Figur-Sprites in `data/sprites.json`) einmal ab; Beritten holt der Reiter aus.
- Je Monarch getrennt: nur die Figur von `from` holt aus, bei 2 Spielern nie die andere.
- Der Schlag eines Gegners (ohne `hit`) lässt keinen Monarchen ausholen.
- Der Client zeichnet nur (Server-Ereignis `strike`); keine neue SIM-Logik, kein Protokoll.
- Der Effekt-Kreis (`swing`) bleibt zusätzlich.

## Nicht-Ziele

Neue Schlagwerte (B-099); Treffer-Effekte (GR5); neue Skills.

## Regeln und Einschränkungen

CLI; `src/scenes` rechnet nichts (`noSim.test.ts`); Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Tag, kein Gegner, Schlag-Taste → der Monarch holt kurz aus, zusätzlich zum Effekt-Kreis.

## Ausnahme- und Fehlerfälle

Schlag während der Abklingzeit → keine neue Ausholbewegung (kein `strike`-Ereignis).

## Akzeptanzkriterien

- **AC-01** Ein Test belegt: Ein `strike` mit `hit` (mit oder ohne Treffer) ergibt die Ausholbewegung für genau den Spieler `from`; ein `strike` ohne `hit` (Gegner) ergibt keine.
- **AC-02** 🧑 sieht am PC bei Schlag ohne Gegner, dass der Monarch ausholt (Beobachtung).

## Offene Fragen

Entschieden 2026-10-11 (🧑, Chat): Die Ausholbewegung wird in S9 umgesetzt (Session S9.7); der Kreis allein genügt für B-318/AC-03 nicht. Sonst keine.

## Notizen

Anlass: S9.5, Schritt 3 (2026-10-11). Verwandt: B-318, S9.3.
