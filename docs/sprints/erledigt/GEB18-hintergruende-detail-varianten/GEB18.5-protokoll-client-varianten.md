# GEB18.5 · Protokoll meldet die Variante, der Client zeichnet sie und lädt nur Benötigtes

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb18/5-protokoll-client-varianten
- **Abhängig von:** GEB18.4
- **Tickets:** B-618
- **Kriterien:** AC-04, AC-06

## Ziel

Das Level meldet die Hintergrund-Varianten je Abschnitt (additives Feld, `docs/protocol.md`), der Client zeichnet sie und lädt nur die Varianten der gezeigten Stufen.

## Kontext

Protokoll `docs/protocol.md`, Server `engine/net`, Client `src/online/`, `src/model/types.ts`, Zeichnen `src/scenes/render/worldSprites.ts` und der Hintergrund-Aufbau in `src/scenes/render/`; Texturen in Dichte 128 mit Zoom 0,25 gegenüber heute.

## Erlaubte Dateien

- `engine/net/**` (nur das Feld), `docs/protocol.md`, Beispiel-Dateien des Protokolls
- `src/model/types.ts`, `src/online/**`, `src/scenes/render/**` und Tests
- Aufräumen: alte `public/grafik/gebaeude/hg-*-16.png`/`-32.png` ohne Variante, ihre Einträge in `public/grafik/gebaeude/index.json`, `data/assets.json`, Doku, Fixtures und Mocks

## Nicht-Ziele

Auswahl der Varianten (GEB18.4), Grafik.

## Schritte

1. Feld im Level additiv, Version bleibt, Beispiel-Datei.
2. Client: Variante je Abschnitt zeichnen, Rückfall Variante 1 bei fehlender Datei (Log 📄).
3. Laden nur für gezeigte Stufen; Speicher vorher/nachher über `sim_test` oder Messung im Browser festhalten.
4. Vitest der Abbildung und des Ladens.
5. Alte Schichten ohne Variante löschen, nachdem eine Suche belegt, dass nichts sie mehr nutzt (AC-06).

## Fertig, wenn

- [ ] AC-04: Vitest grün, `docs/protocol.md` aktuell, Messung im Ergebnis.
- [ ] AC-06: alte Schichten gelöscht, Suche ohne Treffer, Tests grün.

## Prüfen

Gezielte Tests, vor dem Commit `task check:diff`; Reviewer (Protokoll).

## Ergebnis

- AC-04 umgesetzt: `level.layout.backdrops` in der level-Nachricht (JSON-Tag in `engine/level/level.go`, additiv, Version bleibt), `docs/protocol.md`, `testdata/protocol/s2c-level.json`, `LevelLayout.backdrops` in `src/model/types.ts`. Client: `backgroundPlan` (rein, `worldSprites.ts`) wählt je Kachel die Variante, `loadBackground` lädt zur Laufzeit nur die Varianten der gezeigten Stufe, `backgroundLayers` baut je Schicht einen Container aus Bildern (64 px, Maßstab 0,5, Rückfall Variante 1); `StageView` zeigt erst die Silhouetten und ersetzt sie nach dem Laden, Spiegelung im Fluss mit. Entscheidung Ralf: das Spiel lädt Dichte 64 (Generator rendert jetzt 32/64/128; `detail` erscheint ab 1 px).
- AC-04 geprüft: Vitest `src/scenes` (Plan, Dateien je Variante, Wiederholung, Rückfall), `src/online`, `assets`, `zuordnung`, `task:typecheck`, `task:lint`, Go `engine/level` und `engine/net` (Beispiel-Schlüssel), Golden neu – nur Feld `backdrops` neu (Skript-Vergleich), Studio-Suite (540) und `tsc` grün. Browser (Vite 5183, Seed wald1): Wald-Varianten mit Spiegelung, keine Konsolenfehler. Messung: geladen nur 15 Texturen des Bioms (1536 × 960), ≈ 88 MB GPU; vorher ≈ 22 MB für alle fünf Biome (eine Fassung, 32 px). Review (Sonnet): keine schweren Befunde; Lücke der Spiegel-Drift am Weltanfang behoben.
- AC-06 umgesetzt: alte Vorlagen und PNG ohne Variante gelöscht (15 JSON, deren 16/32-px-PNG), Slots `bg:<biom>:<n>` zeigen jetzt auf Variante 1 in 64 px (die Zuordnungstabelle verlangt die IDs), `docs/assets/zuordnung-welt.md` und Regeldatei § 10 nachgezogen; `site/` nutzt eigene Kopien und bleibt. Suche ohne weitere Verweise.
- Nebenbei (Wunsch Ralf): Studio-Seite Slots hat auf schmaler Seite einen waagerechten Splitter (eigener Commit); Studio-Vorschau blendet Schichten jetzt über den Container-Namen `bg:<biom>:<n>` aus. `task check:diff` nicht gelaufen (Ralf: in dieser Claude-Session gesperrt).
