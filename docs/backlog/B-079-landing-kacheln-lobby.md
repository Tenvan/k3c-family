# B-079 · Die Kacheln der Landingpage passen zum Start über die Lobby

- **Domäne:** PLAT
- **Typ:** Schuld
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-01
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`src/landing/pages.ts` öffnet `game.html` mit `?continue=1`, `?save=1`, `?seed=…`, `?depth=…` und `?online=familie`. Ab SP08
startet `game.html` immer in der Lobby (Raumliste), rechnet nichts mehr lokal und kennt nur `?room=CODE` und `?save=NAME`.

## Ziel

Jede Kachel führt zu etwas, das SP08 versteht. Nutzen: keine toten Kacheln auf der Xbox.

## Beteiligte und Zielgruppen

Spieler am TV; Entwickler; 🧑 entscheidet, welche Kacheln bleiben.

## Anforderungen

- Kacheln für lokales Spiel mit eigenem Seed, Tiefe und `?online=` entfallen oder bekommen ein neues Ziel.
- „Weiterspielen“ und „Neues Spiel“ öffnen die Lobby.

## Nicht-Ziele

Lobby selbst (B-037, SP08).

## Regeln und Einschränkungen

Domäne PLAT; Seiten-Regeln aus `CLAUDE.md`; `tests/projectRules.test.ts` bleibt grün.

## Beispiele

Kachel „Neues Spiel“ → Lobby mit Auswahl „Spielen“.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Konfiguration.

## Akzeptanzkriterien

- **AC-01** Jede Kachel in `src/landing/pages.ts` mit Ziel `game.html` benutzt nur Parameter, die SP08 kennt (Test).

## Offene Fragen

Welche Dev-Kacheln (Tiefe 1/2) bleiben, über den Server erreichbar? (🧑)

## Notizen

Entstanden beim Bereitmachen von SP08.
