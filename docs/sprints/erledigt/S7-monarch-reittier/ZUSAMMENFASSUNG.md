# S7 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Monarch auf dem Standard-Reittier
- **Projekt:** –

## Ziel

Der Monarch erscheint beritten, mit Animationen für Stehen, Laufen und Sprint. Am Ende sichtbar: Zwei Spieler im Split-Screen reiten am TV über die Stufe.

## Ergebnis

- **2026-10-04 (S7.4):** AC-01 (S7.1: `mountPose.ts`, 7 Tests), AC-03 (S7.2: Test mit vier Spielern, `noSim.test.ts`) mit Nachweis. AC-02 (Sichtnachweis) und AC-04: angenommen, Validierung offen (S7.3, 🧑 am Gerät; im Fahrplan unter „Offen am Gerät“). - **Befunde:** keine schweren: `src/scenes` rechnet nichts (liest `index`, `vx`, `facing`), kein `Math.random()`, Schlüssel aus `MONARCH.mount.sprite`, Rückfall auf die Figur mit Log, LPC-Credits stehen in `public/sprites/CREDITS.md`. `worldRenderer.ts` 402 Rohzeilen, Oxlint (ohne Leer- und Kommentarzeilen) unter 400, grün. Abweichung S7.2 (Reitti …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Auswahl von Sprite, Animation und Reiter-Position ist eine getestete reine Funktion (B-173 … |
| AC-02 | geprüft | Jeder Monarch erscheint mit Reittier, die Animationen unterscheiden sich je Bewegung (B-17 … |
| AC-03 | geprüft | Zwei Spieler im Split-Screen zeigen beide Reittiere korrekt, der Client rechnet nichts (B- … |
| AC-04 | – | 🧑 hat die Darstellung am TV und am Handy abgenommen (B-173/AC-04). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
