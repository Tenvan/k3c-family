# B-433 · Werkzeug-Links auf game.html öffnen das Debug-Overlay mit ?dev=1

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit K5.3 (B-098) ist das Debug-Overlay nur mit `?dev=1` verfügbar (`src/scenes/debugOverlay.ts`, `debugEnabled`). Werkzeuge, die das Spiel öffnen, setzen den Parameter nicht: `src/tools/levelApi.ts` (`fullSaveTarget` → `game.html?autostart=1&save=…`) und der Start im Level-Betrachter (`src/tools/levelView`, `startTarget`). Wer dort bisher das Overlay sah, sieht es nicht mehr. Spieler-Kacheln in `src/landing/pages.ts` bleiben bewusst ohne Overlay.

## Ziel

Entwickler-Werkzeuge öffnen das Spiel mit Debug-Overlay, Spieler-Seiten ohne. Erst umsetzen, wenn 🧑 das für die Werkzeuge will.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `levelApi.test.ts` und `levelView.test.ts` erwarten `dev=1` in den Zielen von `fullSaveTarget` und `startTarget`.

## Offene Fragen

Sollen Werkzeug-Starts das Overlay überhaupt zeigen? Entscheidet 🧑.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
