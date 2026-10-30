# BS4.1 · Baumschule im Server: farm heißt nursery, Wald-Plätze, Roden vor Kauf, Protokoll clearing

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** bs4/1-baumschule-server
- **Abhängig von:** –
- **Tickets:** B-636
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Die Baumschule ersetzt die Farm im Server: Daten-ID `nursery`, Baumschul-Plätze je Seite in Wald-Chunks, bezahlbar erst nach dem Roden, Protokollfeld `sites[].clearing`.

## Kontext

B-636 (Spec freigegeben), `docs/rules/materialien-gebaeude.md` § 3.3, BS12 (Freiraum-Regel `lineOpen`/`siteCleared` gilt auch für die Baumschule).

## Erlaubte Dateien

- `data/**`, `engine/**`, `src/**` (Umbenennung, Typ), `tools/**` (Umbenennung, Fixtures), `docs/rules`, `docs/data`, `docs/protocol.md`, `docs/glossar.md`

## Nicht-Ziele

Darstellung „erst roden“ im Client (BS6); Bots (BS8); neue Grafik.

## Schritte

1. farm → nursery (Daten, Code, Tests, Docs); Asset-Dateien `farm-*.png` bleiben.
2. Generator `placeNurseries` (Wald-Chunks, eigener Strom `…:nursery`), feste Hub-Farm-Plätze entfernt.
3. Sim: `SiteNeedsClearing`, Plantage auf nursery-Sites.
4. Protokoll `clearing`, Tests, Golden.

## Fertig, wenn

- [x] AC-01 farm heißt nursery in Daten und Go-Code
- [x] AC-02 Generator-Test (Wald, deterministisch)
- [x] AC-03 Sim-Test Roden vor Kauf, Plantage
- [x] AC-04 Protokoll clearing + Test
- [x] `task check:diff` grün

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: `farm` heißt `nursery` in `data/buildings.json`, `hub.json`, `ruins.json`, `assets.json` (IDs), Go-Code, Client, Studio, Bots, Docs; Asset-Dateien `farm-*.png` bleiben (Ausnahme, neue Grafik später). Die festen Hub-Plätze ±52 entfallen, `requiredSiteKinds` ohne Farm.
- AC-02 geprüft: `TestNurseryPlaetzeImWald` (100 Seeds, deterministisch, Baumschule nur im Wald, mindestens 90 % der Seiten), `TestNurseryHatBaeumeImFreiraum`.
- AC-03 geprüft: `TestBaumschuleErstNachDemRoden` (nicht bezahlbar mit Baum, bezahlbar danach), Plantage-Tests (`TestPlantageWaechstJePlatz` u. a.).
- AC-04 geprüft: `sites[].clearing` (`engine/net/payable.go`), `TestClearingImSnapshot`, Doku `docs/protocol.md`; Darstellung „erst roden“ im Client folgt mit BS6.
- Review (Sonnet): keine schweren Befunde; Doku-Widerspruch und fehlende Tests behoben.
- Offen/Folge: Bots roden für die Baumschule noch nicht (BS8); Fixtures v5–v9 behalten `farm` (alte Stände ohne Rücksicht).
- `task check:diff` grün.
