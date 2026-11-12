# CU2.1 · Testseiten des Browser-Clients entfernen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** cu2/1-testseiten-entfernen
- **Abhängig von:** CU1
- **Tickets:** B-822
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

Die sechs Testseiten (`dm.html`, `stress.html`, `texturetest.html`, `testing.html`, `soundtest.html`, `gamepad-test.html`) samt `src/stress/`, Daten, Kacheln, Texten und Tests sind entfernt; Build, Tests und CI bleiben grün.

## Kontext

Spec: B-822 (Klasse A). Die Tabelle in `B-822 › Ausgangslage` nennt je Seite Einstieg, eigenen Code und alle Verweise mit Zeilen (Stand 2026-11-12, vor dem Start mit der Suche aus `B-822 › Beispiele` neu erheben, CU1 ist dann gemergt).

Fallstricke:
- `src/tools/testTiles.ts` nutzt auch `monitor.ts`, `padScroll.ts` nutzt `lizenzen.ts`: beide bleiben.
- `src/scenes/GameScene.ts:38,154` startet den Stresslauf des echten Clients (`startRealStress`, `?stress=`, B-766); mit `src/stress/` entfallen Aufruf, Kacheln `stressReal1`/`stressReal2` und, falls nur dafür da, der Parameter in `src/core/runConfig.ts`.
- Die Kachel-Art `external` (`devTiles.ts:15`) und `targetOf` mit `external` in `src/scenes/menuPages.ts` dienen nur `dm.html`: entfernen, wenn kein Nutzer bleibt.
- `src/core/panelHost.ts:37` `RELEASE_PAGES` enthält nur `texturetest.html`; die Funktion `releasesGame` bleibt nur, wenn sie danach noch Sinn hat (sonst samt Test entfernen).
- `.github/workflows/ci.yml:149` lädt im Smoke-Test `/gamepad-test.html`; auf `/` und `/game.html` kürzen. Sperre `taskfile`.
- `engine/net/handler_test.go` legt einen Wegwerf-Build mit `gamepad-test.html` an: Fixture, nicht anfassen (Nicht-Ziel `engine/net`).
- `tests/projectRules.test.ts:23-26` hat eine Ausnahme für `dm.html`; danach ohne Ausnahme.
- Vite baut jede `*.html` im Root automatisch, `vite*.ts` braucht keine Änderung.

## Erlaubte Dateien

- `dm.html`
- `stress.html`
- `texturetest.html`
- `testing.html`
- `soundtest.html`
- `gamepad-test.html`
- `src/stress/**`
- `src/tools/**`
- `src/scenes/GameScene.ts`
- `src/scenes/menuPages.ts`
- `src/scenes/menuPages.test.ts`
- `src/core/panelHost.ts`
- `src/core/panelHost.test.ts`
- `src/core/shell.test.ts`
- `src/core/runConfig.ts`
- `tests/projectRules.test.ts`
- `tests/textureLayout.test.ts`
- `data/stress.json`
- `public/audio-test/**`
- `.github/workflows/ci.yml`
- `CLAUDE.md`
- `docs/roadmap.md`
- `docs/projekte/SND-sound.md`
- `codemap.md`
- `src/**/codemap.md`
- Planung CU2, B-822 (über MCP)

## Nicht-Ziele

`engine/net`, `docs/protocol.md`, `monitor.html`, `lizenzen.html`, Statuswechsel fremder Sprints und Tickets, Pixel-Reste (CU2.2), Browser-Client als Ganzes (B-823).

## Schritte

1. Suche aus `B-822 › Beispiele` und Import-Suche je Modul neu laufen lassen, Ergebnis notieren.
2. Tests zuerst anpassen (rot): `devTiles.test.ts` (Liste `TOOLS`, Abschnitte), `menuPages.test.ts`, `panelHost.test.ts`, `shell.test.ts`, `tests/projectRules.test.ts` erwarten die Seiten nicht mehr.
3. Seiten, `src/stress/`, `data/stress.json`, `public/audio-test/` und die Module löschen; Kacheln, `INFO_HREFS`, `RELEASE_PAGES`, Texte, `GameScene`-Aufruf und tote Helfer nachziehen.
4. CI-Smoke-Test, `CLAUDE.md`, `docs/roadmap.md`, `docs/projekte/SND-sound.md`, Codemaps.
5. Liste für AC-05 aus den Planungs-Treffern (`docs/sprints/aktiv`, `docs/sprints/geplant`, `docs/backlog`).
6. Review-Agent parallel zu `task check:diff` (CI-Änderung, > 150 Zeilen).

## Fertig, wenn

- [ ] AC-01: Suche aus `B-822 › Beispiele` ohne Treffer außerhalb der Nicht-Ziele; Dateien gelöscht.
- [ ] AC-02: `devTiles`, `menuPages`, `panelHost`, `GameScene` ohne Verweise; ihre Tests und `tests/projectRules.test.ts` grün.
- [ ] AC-03: CI-Smoke-Test ohne `/gamepad-test.html`; `task check` und `task build` grün, Dateiliste `dist/*.html` im Ergebnis.
- [ ] AC-04: `CLAUDE.md`, Roadmap, SND, Codemaps ohne die Seiten.
- [ ] AC-05: Liste betroffener Planung im Ergebnis.
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

–
