# B-707 · assets_set und assets_render lassen golden.json der Gruppe veraltet

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-06
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Gefunden in MG1.1: `assets_set monarch {beschreibung: …}` rendert und schreibt Spec und `manifest.json` (neuer `specHash`/`renderKey`), aber `public/grafik/figuren/golden.json` behält den alten `renderKey`. Erst `task gen:paper` zieht ihn nach; bis dahin schlägt `tools/paper/golden/public.test.ts` fehl.

## Ziel

Nach `assets_set`, `assets_create`, `assets_delete` und `assets_render` passt `golden.json` der betroffenen Gruppe zur Ausgabe; `task gen:paper` ist danach nicht mehr nötig, damit `tools/paper/golden/public.test.ts` grün bleibt.

## Beteiligte und Zielgruppen

- Asset-Lanes (Claude über MCP `assets_*`): sparen `task gen:paper` und das Zurücksetzen fremder Gruppen (D-MG1-02).
- 🧑 Ralf: weniger Merge-Lärm in `golden.json`.

## Anforderungen

Eintrag des Assets aus `manifest.json` (renderKey) und SHA-256 seiner PNGs; Format wie `gen:paper`.

## Nicht-Ziele

Kein Neurendern anderer Assets; `task gen:paper` bleibt für Stiländerungen und Komplettläufe.

## Regeln und Einschränkungen

Code in `tools/k3c-workbench/internal/paperassets` (Domäne DEV); den Index pflegt der Go-Code schon (`inIndexAufnehmen`), golden.json kommt daneben.

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Nach `assets_set`/`assets_create`/`assets_render` steht der neue `renderKey` samt PNG-Hashes des Assets in `golden.json` seiner Gruppe.
- **AC-02** Nur die Gruppe des Assets wird angefasst; andere Einträge und andere Gruppen bleiben bytegleich.
- **AC-03** `assets_delete` nimmt den Eintrag aus `golden.json`.
- **AC-04** Format und Reihenfolge wie `gen:paper` (`JSON.stringify(x, null, 2)`, Ids nach Dateiname sortiert).
- **AC-05** Test zuerst rot.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Umsetzung (sprint/mc2-golden): `paperassets/golden.go` mit `gruppeNachziehen` (Index + golden.json) nach jedem Render in `schreibeSpec` und `RenderEins`, `goldenEntfernen` in `Loesche`; ohne Manifest (Render übersprungen) bleibt golden.json unverändert.
- AC-01, AC-02: `TestGoldenEintragen`, `TestGoldenEintragenNeu` (fremde Gruppe bytegleich).
- AC-03: `TestLoescheGolden`.
- AC-04: `TestGoldenFormatWieGen` (alle `public/grafik/*/golden.json` lesen und schreiben bytegleich).
- AC-05: mit Stub zuerst 3× FAIL, danach grün.
- `go test ./internal/paperassets ./internal/mcpsrv ./internal/assetsui` grün, `golangci-lint` 0 Befunde.
- Nummer: B-707 wie auf `sprint/mg1`; Domäne dort PAP, hier DEV – beim Merge angleichen.
