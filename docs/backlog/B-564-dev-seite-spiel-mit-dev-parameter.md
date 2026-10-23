# B-564 · Die Entwicklerseite öffnet das Spiel mit Debug-Overlay (?dev=1)

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit K5.3 (B-098) erscheint das Debug-Overlay nur mit `game.html?dev=1`. Die Entwicklerseite (`src/tools/devTiles.ts`) hat keine Kachel für das Spiel mit Overlay; Entwickler müssen den Parameter von Hand setzen.

## Ziel

Entwickler öffnen das Spiel mit Overlay vom Controller aus.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Kachel „Spiel (Dev)“ in `src/tools/devTiles.ts` öffnet `game.html?dev=1` über `openPage()`.

## Nicht-Ziele

Overlay für Spieler-Seiten.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Test: `devTiles` enthält eine Kachel mit `game.html?dev=1`.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
