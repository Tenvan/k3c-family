# SV1.2 · Client kennt Eisenstollen und Kristallhöhle

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** sv1/2-client-biome
- **Abhängig von:** SV1.1
- **Tickets:** B-290
- **Kriterien:** AC-03, AC-06

## Ziel

Der Client lädt alle Biome aus `data/biomes/` und zeichnet Eisenstollen (Tiefe 3) und Kristallhöhle (Tiefe 4), wenn der Server sie schickt.

## Kontext

- Grenzfall Protokoll/Client (`docs/arbeitsweise.md`): Diese Session ist der Client-Anteil des SRV-Sprints, beschlossen von 🧑 am 2026-10-06. Das Protokoll ändert sich nicht, `biomeId` gibt es schon.
- `src/model/biome.ts` importiert nur `forest`, `cave`, `mine` und baut daraus `BIOMES`; `biomeForDepth` wirft bei unbekannter Tiefe. `data/biomes/` enthält zusätzlich `ironhold.json` und `crystal.json`.
- Nutzer von `BIOMES`: `src/online/clientStages.ts` (unbekanntes Biom → `clientLog('error', '💥 Unbekanntes Biom …')`, der Strom bekommt kein Level), `src/scenes/HudScene.ts`, `src/scenes/viewRules.ts`, `src/tools/leveltest.ts`.
- Fallstrick: Die JSON-Dateien der neuen Biome können Felder anders füllen als `BiomeConfig` erwartet (z. B. `cycle`, `palette`); der Cast `as unknown as BiomeConfig[]` versteckt das. Ein Test muss Pflichtfelder (`id`, `depth`, `palette`) je Biom prüfen. Statt fester Imports bietet sich `import.meta.glob('../../data/biomes/*.json', { eager: true })` an (Vite); Vitest kann das ebenfalls.
- Kein `Math.random()`; Log-Meldungen mit Emoji.

## Erlaubte Dateien

- `src/model/biome.ts` und ein Test daneben (`src/model/biome.test.ts`)
- `src/online/clientStages.ts` nur, falls der Fehlerfall „unbekanntes Biom“ nachgeschärft werden muss
- Planungs-Dateien (`docs/sprints/`, `docs/backlog/`)

## Nicht-Ziele

Eigene Grafik oder Gegner der neuen Stufen (B-010, K1, B-129), Änderungen am Server oder Protokoll, `data/biomes/*.json` ändern.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Test zuerst: `BIOMES` enthält jede Datei aus `data/biomes/*.json` (außer `codemap.md`), Tiefen 0 bis 4 lückenlos, je Biom `id`, `name`, `palette` gesetzt; `biomeForDepth(3)` liefert `ironhold`, `biomeForDepth(4)` `crystal`.
3. `src/model/biome.ts` lädt alle Biome aus dem Ordner, nach Tiefe sortiert.
4. Prüfen, dass `src/tools/leveltest.ts` die neuen Biome in der Auswahl zeigt (keine Änderung nötig, sonst Ticket).
5. `task check` grün.

## Fertig, wenn

- [ ] AC-03: Vitest belegt, dass der Client alle Biome aus `data/biomes/` lädt (B-290/AC-02).
- [ ] Unbekanntes Biom bleibt ein klarer Fehler (Log 💥), kein Absturz.
- [ ] AC-06: `task check` grün.

## Prüfen

```bash
task check
```

## Ergebnis

–
