# NG4.1 · Gebäude mit Stufe, Schaden und Ruine

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** bauten
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng4/1-gebaeude
- **Abhängig von:** NG1
- **Tickets:** B-797
- **Kriterien:** AC-01

## Ziel

Paket `props` zeichnet Gebäude mit Stufe, Zustand, Schaden und Ruine, Rückwand und Wachstum der Hub-Stufe.

## Kontext

Spec: B-797. Modul `native/k3c-game` (NG1): Vertrag `view/`, Platzhalter in `props/` ersetzen, `game/` und `view/` unverändert; Importregel nur `view`, `art`, `k3c/engine/sim`, `k3c/data`. Vorbild: `src/scenes/render/buildingLook.ts`, `buildingSprites.ts`, `buildingSize.ts`, `backWallRules.ts`, `backWallView.ts`, `hubGrowth.ts`, `damageView.ts`, `src/scenes/hubArt.ts`, `hubArtData.ts`, `hubView.ts` samt Tests. Gebäude-Zustand: `engine/sim/building_*.go`. Grafik über `art`. Superpowers TDD; keine Sim- und Balancing-Läufe.

## Erlaubte Dateien

- `native/k3c-game/props/**`
- Planung NG4, B-797 (über MCP)

## Nicht-Ziele

Baustellen und Schilder (NG4.2), Natur (NG4.3); neue Grafik.

## Schritte

1. Fälle aus den TS-Tests zu Look, Größe und Rückwand als Go-Tests (rot).
2. Gebäude je Typ und Stufe, Schaden und Ruine.
3. Rückwand und Hub-Wachstum.

## Fertig, wenn

- [ ] AC-01: Tests grün; Screenshot mit Gebäuden verschiedener Stufen im Ergebnis.
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

AC-01 umgesetzt: Gebäude je Typ und Stufe (Mauer, Turm, Hub-Gebäude, Schiff, Treppen), Schaden mit Rissen, Ruine, Hintergrundmauer, Wachstumsreihe der Burg. Geprüft mit `buildings_test.go` (Fälle aus den Browser-Tests) und am Bild `reports/native-game-shots/ng4/bauten/shot-01.png` aus einem gebauten Spielstand (Agent und Claude: Mauern, Türme, Heilplatz, Taverne, Lager, Werkstatt, Burg Stufe 3, beschädigter Turm, Gerüst).
Ruine nur per Test belegt (im Seed wald gibt es keine).
