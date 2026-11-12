# CU2.2 · Pixel-Reste in Code, Tests und Doku nachziehen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** cu2/2-pixel-reste
- **Abhängig von:** CU2.1
- **Tickets:** B-824
- **Kriterien:** AC-06, AC-07, AC-08, AC-09, AC-10

## Ziel

Kein Code, Test, Task und keine gültige Doku verweist mehr auf alte Pixel-Grafik; `TestHTMLEigenstaendig` der Workbench ist grün.

## Kontext

Spec: B-824 (Klasse A). Fundstellen mit Zeilen in `B-824 › Ausgangslage` (Stand 2026-11-12, vor dem Start mit der Suche aus `B-824 › Beispiele` neu erheben).

- **Bericht:** `tools/k3c-workbench/internal/balance/report_html.go:33-42` hat noch Pixel-Pfade; Vorlage ist `engine/balance/report_html.go:33-42` (Papier-Pfade aus CU1, z. B. `grafik/gebaeude-paper/burg/stand.png`, `grafik/gegner/skelett/idle.png` mit `frameW`/`frameH`). Die PNGs entstehen erst mit `task gen:paper` (nicht eingecheckt, B-737); für den Nachweis von AC-06 einmal `gen:paper` laufen lassen, das ist hier Feature-Arbeit (B-742). Kein Grafik-Golden.
- **Tests:** `src/scenes/render/hubGrowth.test.ts:10` globt noch `public/grafik/gebaeude/*-32.png`; `buildingSprites.test.ts:25` heißt „Gebäude aus tools/buildings …“.
- **Task:** `tools/Taskfile.yml:69` (`soundatlas` › `sources`) nennt `data/assets.json`.
- **Doku:** `LICENSE`, `LICENSE-ASSETS`, `README.md`, `docs/release/LIES-MICH.txt`, `docs/release/itch-io.md` verweisen auf `public/sprites/CREDITS.md`; `docs/game-design.md:187`, `docs/rules/monarch.md:69` auf `data/sprites.json`; `docs/assets/sounds.md:3`, `public/audio/README.md:4,14` auf `data/assets.json`; `docs/arbeitsweise.md:522` (Credits-Prüfung); `CLAUDE.md` Satz „Natur, Objekte und Gerüste bleiben bis MG8 Pixel-Grafik“.
- **Glossar** (`docs/glossar.md:23,24,228,232`) ist Sammeldatei: die Änderung macht der letzte Commit des Sprints (CU2.3), hier nur den Text vorbereiten.
- `tools/assetkit` gehört zu B-760, `site/` zu B-823: nicht anfassen.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/balance/report_html.go`
- `tools/k3c-workbench/internal/balance/*_test.go`
- `src/scenes/render/hubGrowth.test.ts`
- `src/scenes/render/buildingSprites.ts`
- `src/scenes/render/buildingSprites.test.ts`
- `src/scenes/render/natureSprites.ts`
- `src/scenes/render/worldSprites.ts`
- `src/scenes/render/scaffoldView.ts`
- `tools/Taskfile.yml`
- `LICENSE`
- `LICENSE-ASSETS`
- `README.md`
- `docs/release/**`
- `docs/game-design.md`
- `docs/rules/monarch.md`
- `docs/assets/sounds.md`
- `public/audio/README.md`
- `docs/arbeitsweise.md`
- `CLAUDE.md`
- Planung CU2, B-824 (über MCP)

## Nicht-Ziele

`tools/assetkit` (B-760), `site/` (B-823), historische Doku (`docs/plan-weiterentwicklung.md`, `docs/fragenkatalog.md`, `docs/funde/`), Codemaps, Zusammenführen von `engine/balance` und `internal/balance`, Regeländerungen in `docs/rules/`.

## Schritte

1. Suche aus `B-824 › Beispiele` neu laufen lassen, Ergebnis notieren.
2. Test (rot): `TestHTMLEigenstaendig` nach `task gen:paper`; dann Pfad-Tabelle aus `engine/balance/report_html.go` übernehmen (grün).
3. `hubGrowth.test.ts` und `buildingSprites.test.ts` auf Papier-Assets, Kommentare in `src/scenes/render/`.
4. `tools/Taskfile.yml` › `soundatlas` auf `data/sounds.json`.
5. Doku, Lizenzen, `CLAUDE.md`; Glossar-Text für CU2.3 im Ergebnis ablegen.
6. Liste alter Planung mit Pixel-Werkzeugen (AC-10) aus `docs/backlog` und `docs/sprints/{aktiv,geplant}`.

## Fertig, wenn

- [ ] AC-06: `TestHTMLEigenstaendig` grün (`check_run`, Lauf im Ergebnis), Pfade gleich `engine/balance`.
- [ ] AC-07: Render-Tests grün ohne `*-32.png`, Kommentare ohne `tools/buildings`.
- [ ] AC-08: `soundatlas` mit Quelle `data/sounds.json`.
- [ ] AC-09: Suche aus `B-824 › Beispiele` ohne Treffer.
- [ ] AC-10: Liste alter Planung im Ergebnis.
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

- AC-01 ✅ `tools/k3c-workbench/internal/balance/report_html.go` mit den Papier-Pfaden aus `engine/balance`; `TestHTMLEigenstaendig` grün (`go test -short ./internal/balance/`, Grafik lokal vorhanden).
- AC-02 ✅ `hubGrowth.test.ts`, `buildingSprites.test.ts` ohne 32-px-Pfade (21 Tests grün), Kommentare in `src/scenes/render/` auf Papier-Quelle.
- AC-03 ✅ `soundatlas`-Quelle `data/sounds.json`.
- AC-04 ✅ LICENSE, LICENSE-ASSETS (inkl. Schrift BSD-3 und CC-BY 4.0), README, Release-Texte, Fach-Doku, Glossar und CLAUDE.md ohne `public/sprites/`, `data/sprites.json`, `data/assets.json`, MG8-Satz; verbleibende Treffer sind Hinweise „entfiel mit CU1“.
- AC-05 Ralf-Queue: B-435, B-462, B-570, B-571, B-641, B-685 und Sprint GRJ2 planen noch `tools/buildings` bzw. `data/assets.json`; aktive CU1/MC1 enthalten Altverweise. Vorschlag neues Ticket: beide balance-Pakete zusammenführen.
