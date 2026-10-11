# B-396 · Der Monarch holt beim Schlag ohne Ziel sichtbar aus

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Abnahme S9.5 am 2026-10-11 (🧑, PC, Tastatur, am Tag ohne Gegner): Beim Schlag ohne Ziel blinkt ein Kreis auf, der Monarch holt aber nicht aus (keine Figuren-Animation). B-318 nennt als Beispiel „Monarch holt kurz aus“; S9.3 zeichnet nur das Ereignis `strike` als Effekt (`src/scenes/effects.ts`, `effectsView.ts`).

## Ziel

Jeder Schlag, mit und ohne Treffer, zeigt eine Ausholbewegung des Monarchen. Nutzen: Die Taste wirkt wie ein Schlag, nicht nur wie ein Effekt-Kreis.

## Beteiligte und Zielgruppen

Alle Spieler; 🧑 bei der Abnahme. Grafik: sofern der Monarch-Sprite keine Angriffs-Animation hat, braucht es ein Bild (Credits beachten).

## Anforderungen

- Beim Ereignis `strike` spielt der Monarch dieses Spielers eine kurze Ausholbewegung, auch ohne Treffer.
- Je Monarch getrennt, mit 2 Spielern.
- Der Client zeichnet nur; keine neue SIM-Logik.

## Nicht-Ziele

Neue Schlagwerte (B-099); Treffer-Effekte (GR5); neue Skills.

## Regeln und Einschränkungen

CLI; `src/scenes` rechnet nichts (`noSim.test.ts`); Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Tag, kein Gegner, Schlag-Taste → der Monarch holt kurz aus, zusätzlich zum Effekt-Kreis.

## Ausnahme- und Fehlerfälle

Schlag während der Abklingzeit → keine neue Ausholbewegung (kein `strike`-Ereignis).

## Akzeptanzkriterien

- **AC-01** Ein Test belegt: Das Ereignis `strike` setzt am Monarchen des Spielers die Ausholbewegung, `hit` ändert daran nichts.
- **AC-02** 🧑 sieht am PC bei Schlag ohne Gegner, dass der Monarch ausholt (Beobachtung).

## Offene Fragen

Ist die Ausholbewegung für die Abnahme von B-318/AC-03 nötig oder reicht der Effekt-Kreis? Entscheidet 🧑 (siehe S9.5).

## Notizen

Anlass: S9.5, Schritt 3 (2026-10-11). Verwandt: B-318, S9.3.
