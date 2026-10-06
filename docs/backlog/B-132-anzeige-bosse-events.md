# B-132 · Der Client zeigt Gegner-Fähigkeiten, Bosse, Phasen und Events

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** K5
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, mit Sprint K5

## Ausgangslage

Der Client zeichnet Gegner und Wellen; Bosse, Phasen, Flächenangriffe und Events gibt es nicht (`src/scenes/worldRenderer.ts`, `HudScene.ts`).

## Ziel

Spieler erkennen Flächenschläge (Warnkreis), Phasen, Boss-Lebensleiste und Events (Banner, Mondanzeige); neue Gegner haben Platzhalter. Nutzen: Kämpfe sind lesbar.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy; 🧑 testet am Gerät.

## Anforderungen

- Warnkreis vor Flächenschlägen, Phasenanzeige am Boss, Boss-Lebensleiste im HUD (oben, nicht über dem Home-Button).
- Event-Banner und Mondanzeige (Vollmond, Blutmond), Händler-Überfall-Hinweis.
- Platzhalter-Darstellung für neue Gegner, bis Grafiken kommen (B-010); Aktionen-Overlay für den Bau-Auslöser des Endbosses (B-125).
- Kein Spiel-Logik-Code in `src/scenes`.

## Nicht-Ziele

Simulation (B-128 bis B-131), Protokoll (B-154), Grafiken (B-010), Sound (B-011).

## Regeln und Einschränkungen

`CLAUDE.md` (Client zeichnet nur, 70 px oben frei, B-Taste frei). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Der Höhlentroll holt aus: ein roter Kreis (Radius 3) leuchtet 0,5 s, dann Schaden. Boss-Leiste zeigt Phase 2 von 3.

## Ausnahme- und Fehlerfälle

Snapshot ohne die Felder (alter Server) → Anzeige leer, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Reine Funktionen für Boss-Leiste, Phasen-Text und Event-Banner sind getestet.
- **AC-02** Warnkreis, Boss-Leiste und Event-Banner werden angezeigt.
- **AC-03** 🧑 hat die Anzeige am Gerät abgenommen.

## Offene Fragen

keine

## Notizen

Aus R4.2 und R4.3. Abhängig von B-154 und B-130.
