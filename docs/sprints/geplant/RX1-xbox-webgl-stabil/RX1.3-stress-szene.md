# RX1.3 · Stress-Szene Live-Grafik mit Szenario-Datei, Rampe und Log

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** rx1/3-stress-szene
- **Abhängig von:** RX1.1
- **Tickets:** B-765
- **Kriterien:** AC-11, AC-12, AC-13, AC-14, AC-15, AC-16

## Ziel

`stress.html` startet die Stress-Szene mit Szenario-Datei, Rampe und auswertbarem Log (B-765), damit der Xbox-Lauf (RX1.4) gleich mit abgestuften Dichten misst.

## Kontext

Spec: B-765 (Szenario-Format `data/stress.json`, Rampe, Log-Felder, Presets, Entscheidungen E-1 bis E-3). Baut auf RX1.1 (Render-Log, `renderInfo.ts`, `clientLog`) auf. Wiederverwendbar: `Histogram` und `sendPerfReport` (`src/online/clientPerfReport.ts`), `collectPerf` (`src/online/clientPerf.ts`), Kachel in `src/tools/devTiles.ts` (Abschnitt `perf`). **Voraussetzung vor jedem Start der Szene: `task gen:paper`** (PNG und Manifeste sind nicht eingecheckt, B-737; `task dev`, `build` und `test` rufen es selbst auf, `check:diff` nicht).

## Erlaubte Dateien

- `stress.html`, `src/stress/**` (neu), `data/stress.json` (neu), `src/tools/devTiles.ts`, Texte `tile.stress.*`
- zugehörige `*.test.ts`; `src/core/renderLog.ts` nur zur Wiederverwendung erweitern

## Nicht-Ziele

Kein Server, keine Simulation, keine Bosse im Preset, keine Optimierung (B-765 › Nicht-Ziele).

## Schritte

1. `task gen:paper` ausführen, danach `data/stress.json` mit `typisch` und `worstCase` und Prüfung (Typen, `players`, Zuordnung in `data/paper.json`/`index.paper.json`).
2. Phaser-freie Logik: Szenario-Prüfung, Rampe mit Seed, Stufen-Zusammenfassung, Dichte-Grenze, Aufwärmzeit.
3. `StressScene` mit 1 oder 2 Viewports, echter Renderer, feste Bahnen; URL-Parameter `scenario`, `players`, `ramp`, `seed`; Start per Gamepad (A).
4. Log nach B-765 › Objektive Auswertung (`runId`, Stufen-Zeile, Ereignisse, Endzeile); mit `?perf=1` Bericht.
5. `stress.html` mit `installPageChrome()` und Kachel auf `dev.html`.
6. Reviewer-Agent ab ca. 150 geänderten Code-Zeilen, parallel zu `task check:diff`.

## Fertig, wenn

- [ ] AC-11: Test: `data/stress.json` mit beiden Presets, alle Typen mit Asset in `data/paper.json` und `index.paper.json`.
- [ ] AC-12: Test: ungültige Szenarien werden abgelehnt.
- [ ] AC-13: Test: Rampe alle 10 s um den Faktor, gleicher Seed gleiche Verteilung.
- [ ] AC-14: Test: je Stufe genau eine Log-Zeile mit `runId`, `device`, FPS, p95, `textureMiB`, `entities`, `sheets`.
- [ ] AC-15: Test: Endzeile mit Dichte-Grenze und Zusammenfassung, Aufwärmzeit zählt nicht in die Perzentile.
- [ ] AC-16: `tests/projectRules.test.ts` grün (Kachel, `installPageChrome()`).

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

–
