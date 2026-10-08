# B-034 · Große PLAT-Dateien liegen unter 300 Zeilen

- **Domäne:** PLAT
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** verworfen
- **Sprint:** –
- **Projekt:** –
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Über dem Ziel von 300 Zeilen: `src/tools/gamepadTest.ts` (332), `src/tools/spriteReference.ts` (330), `src/landing/landing.ts` (319).

## Ziel

Große PLAT-Dateien liegen unter 300 Zeilen. Nutzen: Komplexitäts-Budget: die Ausnahmeliste soll schrumpfen.

## Beteiligte und Zielgruppen

Entwickler und Review.

## Anforderungen

- Alle drei Dateien ≤ 300 Zeilen.

## Nicht-Ziele

Verhalten ändern.

## Regeln und Einschränkungen

Regel „Seiten & Navigation“ aus `CLAUDE.md` (`installPageChrome()`, `toggleFullscreen()`, `goHome()`); B nicht belegen, View + Menu reserviert.

## Beispiele

Nach dem Umbau zählt `wc -l` höchstens 300 Zeilen je Datei.

## Ausnahme- und Fehlerfälle

nicht relevant – Aufteilung ohne Verhaltensänderung.

## Akzeptanzkriterien

- **AC-01** Alle drei Dateien haben ≤ 300 Zeilen.
- **AC-02** Ihre Einträge sind aus der Ausnahmeliste entfernt.

## Offene Fragen

keine

## Notizen

Verworfen 2026-09-30 (🧑, Chat): Das Review wurde entschärft, es gelten nur noch die harten Grenzen aus `docs/arbeitsweise.md` › Komplexitäts-Budget; der Zielwert 300 Zeilen und die Baseline-Ratsche entfallen.
