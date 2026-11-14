# BI1.1 · Datenmodell, Datenprüfung und Level-Generator mit Tests und Golden

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** bi1/1-datenmodell-generator
- **Abhängig von:** –
- **Tickets:** B-891
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Biome tragen `traits` und `weights`, die Welt setzt Werte je Insel und Tiefe fest, der Level-Generator würfelt die übrigen aus dem Seed und wendet sie an; Datenprüfung, Spielstand und Golden ziehen nach.

## Kontext

Spec: B-891 › Anforderungen (Tabelle aller Eigenschaften). Generator `engine/level/level.go` (Hauptstrom ist Vertrag), geplante Chunks `planned_gen.go` (eigener Strom), Prüfung `validate.go`. Welt-Datei: `engine/sim/island_worlds.go` (strikt), `island_data.go`, `island.go` (CreateOptions). Spielstand: `save.go` (HubSave), `island_save.go` (savedCreateOptions). `k3c-sim world` (`cmd/k3c-sim/world.go`) liefert dem Welten-Editor das Level.

## Erlaubte Dateien

- `data/biomes/**`
- `engine/level/**`
- `engine/sim/island*.go`
- `engine/sim/save.go`
- `engine/sim/types.go`
- `engine/sim/world.go`
- `engine/sim/campaign.go`
- `engine/sim/mine_test.go`
- `cmd/k3c-sim/world.go`
- `testdata/golden/**`
- `docs/rules/welten.md`

## Nicht-Ziele

Welten-Editor-Oberfläche (BI1.2), Darstellung im nativen Spiel (BI1.3), Balancing.

## Schritte

1. `engine/level/nature.go` und `nature_check.go`: Trait, WeightGroup, NatureValues, ResolveNature, withNature, assignVariants, Datenprüfung.
2. Biom-Daten aller fünf Biome nach der Tabelle in B-891.
3. Welt: `nature`/`natureByDepth` lesen und prüfen, CreateOptions.Nature, Spielstand `nature` je Stufe.
4. Tests (AC-01 bis AC-04), `task go:golden` mit Begründung.
5. `docs/rules/welten.md` § 2 um die Felder ergänzen.

## Fertig, wenn

- [ ] AC-01: `TestGoldenNatur` und Level-Golden grün.
- [ ] AC-02: `TestNaturSeedsUnterscheidenSich`, `TestNaturWalddichteSkaliertBaeume` grün.
- [ ] AC-03: `TestNaturFesterWertGilt`, `TestInselNaturFestImSpiel` grün.
- [ ] AC-04: `TestNaturMonokultur`, `TestNaturDatenpruefung`, `TestInselNaturAbgelehnt` grün.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: `TestGoldenNatur` (neue Golden `testdata/golden/nature.json`) und Level-Golden grün.
- AC-02 geprüft: `TestNaturSeedsUnterscheidenSich`, `TestNaturWalddichteSkaliertBaeume`, `TestNaturChunkGewichte`, `TestNaturAdern` grün.
- AC-03 geprüft: `TestNaturFesterWertGilt`, `TestInselNaturFestImSpiel`, `TestInselNaturImSpielstand` grün.
- AC-04 geprüft: `TestNaturMonokultur`, `TestNaturKristallfarben`, `TestNaturDatenpruefung`, `TestInselNaturAbgelehnt` grün.
- `task check:diff:go` grün. `check:diff:js` rot nur durch fremde Tickets aus dem Basis-Commit (B-887, B-892 bis B-894: AC-Format und Umgebung), nicht Teil dieser Session.

Abweichungen: Das Datenmodell ist allgemein (`traits` mit `scales`/`chunks`/`display`, `weights` als Gewichts-Gruppen) nach der Erweiterung auf alle Biome (Ralf, 2026-11-14). Variante (`Variant`) steht in Level-Entität und Sim-Knoten nur im Prozess (`json:"-"`), Natur-Werte nicht in der level-Nachricht; der Spielstand speichert `nature` je Stufe. Die alte Kampagne (`CreateCampaign`, nur Tests) und die Test-Inseln laufen mit neutraler Natur, damit aufgezeichnete Eingaben gelten; die Dichte-Regel (`TestBreiteUndDichte`) gilt für das Biom mit neutralen Werten. Golden neu (`task go:golden`): Level, Sim und Kampagnen, weil gewürfelte Natur-Werte Anzahl und Lage der Objekte ändern.
