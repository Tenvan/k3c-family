# rust/crates/k3c-sim/src/story/

## Responsibility

Geschichte je Welt und Insel (Port von `engine/sim/story*.go`, docs/rules/welten.md § 2.2): Kunstsprache mit eingestreuten echten Wörtern, feste
Zeilen in der Welt-Datei. Alles optional; fehlt ein Eintrag, gibt es dort keine Zeilen.

## Design

- `mod.rs`: `WorldStory`, `IslandStory`, `decode_world_story`, `decode_island_story` (ein Eintrag = 1 bis 2 Zeilen).
- `check.rs`: `check_story` (Reihenfolge lexicon, start, etappen, end, bottles, dann je Insel arrival, boss, done), `story_words`, `is_real_word`,
  `has_story`; Meldung `story.<Feld> Zeile <n>: <Grund>`.
- `points.rs`: reine Funktionen für Wendepunkte: `story_start`, `story_arrival`, `story_boss`, `story_defeated`, `story_victory`, dazu
  `island_story_*` für eine laufende `Island`. Ohne Zustand, Zufall und neue Ereignisse.

## Flow

Beim Laden der Welt-Datei decodieren und `check_story` nach den Inseln. Zur Laufzeit ruft das Spiel die Punkt-Funktionen bei `islandSwitch`,
`bossSpawned`, `bossDefeated` und blendet die Zeilen ein; ein Laden löst sie nicht aus.

## Integration

- Abhängigkeiten: `island` (`WorldDef`, `Island`), `k3c-data`.
- Daten: `story` in `data/worlds/<id>.json`.
- Konsumenten: `island/worlds*.rs`, `k3c-room`, `k3c-game` (Einblendung).
