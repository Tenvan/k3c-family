# B-751 · gen:paper schneller: Cache je Asset und parallel rastern

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ein voller `gen:paper`-Lauf dauert gut 3 Minuten (127 Assets, davon `figuren/monarch` über 2 Minuten, 195 Ausgaben). Messung: 98 % der Zeit steckt in `resvg` (Rust, `@resvg/resvg-js`), nur 2 % in TypeScript; eine Umschreibung in Go, Rust oder Python bringt kaum etwas. `resvg` läuft je Aufruf auf einem Kern.

## Ziel

Ein Lauf nach kleiner Änderung dauert Sekunden, ein voller Lauf deutlich unter einer Minute.

## Beteiligte und Zielgruppen

Entwickler (🧑).

## Anforderungen

- Cache je Asset: das Manifest in `.work` trägt den `renderKey`; passt er zu Spec, Stil und Renderer, wird das Asset nicht neu gerendert.
- Rastern verteilt auf mehrere Prozesse oder Worker (Ausgaben sind unabhängig); Ergebnisse bleiben bitgleich (Golden).
- Optional: Kosten je Frame senken (Filter, Schatten), prüfen mit der Messung.

## Nicht-Ziele

Kein Wechsel der Sprache oder des Rasterisierers.

## Regeln und Einschränkungen

Emoji-Log-Konvention; stdout bleibt das JSON.

## Beispiele

`⏳ [7/127] figuren/monarch rendert …` erscheint sofort nach den Bossen.

## Ausnahme- und Fehlerfälle

nicht relevant: Fehler wie bisher.

## Akzeptanzkriterien

- **AC-01** Zweiter `task gen:paper` direkt nach dem ersten rendert 0 Assets.
- **AC-02** Voller Lauf mit 8 Kernen höchstens ein Drittel der heutigen Zeit; `tools/paper/golden` unverändert grün.

## Offene Fragen

Wie viele Kerne soll der Lauf nutzen (🧑)?

## Notizen

Gefunden beim Test der Workbench (Lauf schien bei den Bossen zu enden).
