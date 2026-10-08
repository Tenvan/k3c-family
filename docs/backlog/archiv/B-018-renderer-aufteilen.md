# B-018 · worldRenderer und GameScene liegen unter 300 Zeilen

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** verworfen
- **Sprint:** –
- **Projekt:** –
- **Erstellt:** 2026-09-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`src/scenes/worldRenderer.ts` (383 Z.) und `src/scenes/GameScene.ts` (371 Z.) liegen über dem Ziel.

## Ziel

worldRenderer und GameScene liegen unter 300 Zeilen. Nutzen: Große Dateien sind schwer zu prüfen und zu ändern.

## Beteiligte und Zielgruppen

Entwickler und Review.

## Anforderungen

- `worldRenderer.ts` und `GameScene.ts` je ≤ 300 Zeilen.

## Nicht-Ziele

Verhalten ändern.

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots und rechnet nichts; Seiten-Regeln aus `CLAUDE.md`; Taste B nicht belegen; 2 Spieler gleichzeitig (Split-Screen). Aufteilen beim Umbau zum reinen Client (SP08).

## Beispiele

Nach dem Umbau zählt `wc -l` höchstens 300 Zeilen je Datei.

## Ausnahme- und Fehlerfälle

nicht relevant – Aufteilung ohne Verhaltensänderung.

## Akzeptanzkriterien

- **AC-01** Beide Dateien haben ≤ 300 Zeilen.
- **AC-02** Ihre Einträge sind aus der Ausnahmeliste entfernt.

## Offene Fragen

keine

## Notizen

Beim Umbau zum reinen Client (SP08) aufteilen. Verworfen 2026-09-30 (🧑, Chat): Das Review wurde entschärft, es gelten nur noch die harten Grenzen aus `docs/arbeitsweise.md` › Komplexitäts-Budget; der Zielwert 300 Zeilen und die Baseline-Ratsche entfallen.
