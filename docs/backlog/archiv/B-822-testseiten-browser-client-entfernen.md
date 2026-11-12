# B-822 · Die Testseiten des Browser-Clients sind samt Code, Kacheln und Tests entfernt

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** CU2
- **Projekt:** NAT
- **Erstellt:** 2026-11-12
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Der Browser-Client hat sechs Test- und Werkzeugseiten aus der Xbox/Edge-Zeit. Mit dem nativen Spiel (Projekt NAT, Entscheidung 008) entfällt ihr Zweck; 🧑 hat am 2026-11-12 beschlossen, sie zu entfernen. Bestand (Suche `git grep` am 2026-11-12 auf `sprint/cu-altlasten`, ohne `docs/sprints/erledigt/`, `docs/backlog/archiv/`, `docs/decisions/`, Codemaps):

| Seite | Einstieg | Eigener Code | Wer verweist darauf |
|---|---|---|---|
| `dm.html` | `src/tools/dm.ts` | `src/tools/dmApi.ts` (+Test), Texte `dm.*` | Kachel `dm` (einzige `external`-Kachel) `src/tools/devTiles.ts:60`, `devTiles.test.ts:33`, `src/scenes/menuPages.test.ts:77`, Ausnahme in `tests/projectRules.test.ts:23-26`; Server `engine/net/static.go:58` (`/dm` → `dm.html`), `engine/net/devpage_test.go:93`, `docs/protocol.md:816` |
| `stress.html` | `src/stress/main.ts` | ganz `src/stress/` (12 Dateien, ca. 1,5k Zeilen), `data/stress.json` | Kacheln `stress`, `stressReal1`, `stressReal2` in `devTiles.ts:62` (Abschnitt `perf`), `devTiles.test.ts:6,27`; `src/scenes/GameScene.ts:38,154` (`startRealStress`, Parameter `?stress=`, B-766); `src/tools/textureTest.ts:6` (`deviceTag`); `src/core/panelHost.test.ts:46`; `src/scenes/menuPages.test.ts:79` |
| `texturetest.html` | `src/tools/textureTest.ts` | `glDiagnose.ts`, `glProbe.ts` (+Test), `textureProbe.ts` (+Test) | Kachel `textures` `devTiles.ts:62`; `src/core/panelHost.ts:37` (`RELEASE_PAGES`) mit `panelHost.test.ts:39-48`; `tests/textureLayout.test.ts` |
| `testing.html` | `src/tools/testing.ts` | `testScenarios.ts` (+Test) | Kachel `testing` `devTiles.ts:56`; `src/core/shell.test.ts:5`. **Achtung:** `testTiles.ts` nutzt auch `monitor.ts`, bleibt also |
| `soundtest.html` | `src/tools/soundtest.ts` | `soundtestLogic.ts` (+Test) | Kachel `sound` `devTiles.ts:59`; `docs/projekte/SND-sound.md:18,31`; Session HW1.9 |
| `gamepad-test.html` | `src/tools/gamepadTest.ts` | `audioProbe.ts` (+Test), `public/audio-test/` | Info-Eintrag `src/scenes/menuPages.ts:86` (`INFO_HREFS`) mit `menuPages.test.ts:41-42`; Kachel `gamepad` `devTiles.ts:58`; `src/core/shell.test.ts:5`; CI-Smoke-Test `.github/workflows/ci.yml:149`; `CLAUDE.md:84`; `docs/roadmap.md:8`; Wegwerf-Build in `engine/net/handler_test.go:15-59`; Server `POST /api/report` |

Dazu für alle: Texte in `src/tools/texts.de.ts` und `texts.en.ts` (Abschnitte der Seiten und `tile.*`), `src/tools/devTiles.test.ts:6` (Liste `TOOLS`), Codemaps (`src/tools/codemap.md`, `codemap.md`). Taskfiles, `Dockerfile`, `vite*.ts` und die Workbench nennen keine der Seiten (Vite baut jede `*.html` im Root automatisch). Bleiben: `index.html`, `game.html`, `monitor.html`, `lizenzen.html` samt `padScroll.ts`, `testTiles.ts`.

## Ziel

Im Repo gibt es die sechs Testseiten nicht mehr und keinen Code, keine Daten, Kacheln, Texte oder Tests, die nur ihnen dienen. Das Spielmenü zeigt keine toten Kacheln, Build und CI bleiben grün.

## Beteiligte und Zielgruppen

🧑 Ralf (Beschluss, entscheidet die Folgen für Xbox-Abnahmen); Claude (Umsetzung, Review). Spieler merken nichts.

## Anforderungen

- Die sechs `*.html` im Root, `src/stress/`, `data/stress.json`, `public/audio-test/` und die Module `dm.ts`, `dmApi.ts`, `gamepadTest.ts`, `audioProbe.ts`, `soundtest.ts`, `soundtestLogic.ts`, `testing.ts`, `testScenarios.ts`, `textureTest.ts`, `glDiagnose.ts`, `glProbe.ts`, `textureProbe.ts` samt Tests sind gelöscht.
- `GameScene` startet keinen Stresslauf mehr (`startRealStress`, `?stress=` entfallen); `runConfig` liest den Parameter nicht mehr, falls nur dafür vorhanden.
- Kacheln, `INFO_HREFS`-Eintrag, `RELEASE_PAGES`-Eintrag und Texte der Seiten entfallen; Code, der danach keinen Nutzer mehr hat (z. B. Kachel-Art `external`, leerer Abschnitt), entfällt mit, Tests werden angepasst statt gelöscht, wo sie Bleibendes prüfen.
- CI-Smoke-Test prüft `/` und `/game.html` ohne `/gamepad-test.html`; `CLAUDE.md`, `docs/roadmap.md`, `docs/projekte/SND-sound.md` und betroffene Codemaps nennen die Seiten nicht mehr als vorhanden.
- Gelöscht wird nur, was die Suche als ungenutzt belegt; Suchbefehl und Ergebnis stehen im Session-Ergebnis.

## Nicht-Ziele

- Server-Endpunkte `/api/dev`, `/api/report`, die Abbildung `/dm` → `dm.html` in `engine/net/static.go`, `docs/protocol.md` (Domäne SRV, Sperre `protokoll`; bei Bedarf eigenes Ticket).
- `monitor.html`, `lizenzen.html`, `game.html`, `index.html`; der Browser-Client als Ganzes (B-823).
- Statuswechsel fremder Sprints und Tickets (siehe Offene Fragen).
- Pixel-Reste (B-824).

## Regeln und Einschränkungen

Domäne INF nach dem Grenzfall „Alt-Engine löschen“ (`docs/arbeitsweise.md` › Domänen): INF löscht und zieht Importe in `src/scenes/`, `src/core/`, `src/tools/`, Tests, CI und Doku nach, keine neuen Funktionen. Keine Änderung an Spielregeln, Protokoll, Spielstand oder `package.json`. `tests/projectRules.test.ts` bleibt grün (jede verbleibende Seite hat Kachel und `installPageChrome()`). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

- **Bereich:** `dm.html`, `stress.html`, `texturetest.html`, `testing.html`, `soundtest.html`, `gamepad-test.html`, `src/stress/**`, `src/tools/**`, `src/scenes/GameScene.ts`, `src/scenes/menuPages.ts`, `src/scenes/menuPages.test.ts`, `src/core/panelHost.ts`, `src/core/panelHost.test.ts`, `src/core/shell.test.ts`, `src/core/runConfig.ts`, `tests/projectRules.test.ts`, `tests/textureLayout.test.ts`, `data/stress.json`, `public/audio-test/**`, `.github/workflows/ci.yml`, `CLAUDE.md`, `docs/roadmap.md`, `docs/projekte/SND-sound.md`, `codemap.md`, `src/**/codemap.md`
- **Abhängig von:** CU1
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** taskfile

## Beispiele

- Spielmenü › Seiten › Entwicklung zeigt nur noch die Kachel „Spiel (Dev)“, Performance nur „Monitor“; keine Kachel führt ins Leere.
- `git grep -n -E "(dm|stress|texturetest|testing|soundtest|gamepad-test)\.html|src/stress|stress\.json|audio-test|startRealStress" -- ':!docs/sprints' ':!docs/backlog' ':!docs/decisions' ':!docs/fragenkatalog.md' ':!docs/plan-weiterentwicklung.md' ':!graphify-out' ':!.codemap'` → keine Treffer außer `engine/net` und `docs/protocol.md` (Nicht-Ziel) sowie der Sprint-Tabelle in `docs/projekte/SND-sound.md` (Historie SO3).
- `task build` erzeugt in `dist/` nur `index.html`, `game.html`, `monitor.html`, `lizenzen.html`.

## Ausnahme- und Fehlerfälle

- Ein Modul wird doch von einer bleibenden Seite gebraucht (wie `testTiles.ts` von `monitor.ts`, `padScroll.ts` von `lizenzen.ts`) → bleibt, Fundstelle im Ergebnis.
- Ein Lesezeichen auf eine entfernte Seite → der Server liefert 404 wie bei jeder unbekannten Datei; keine Weiterleitung.

## Akzeptanzkriterien

- **AC-01** Die sechs Seiten, `src/stress/`, `data/stress.json`, `public/audio-test/` und die genannten Module fehlen; die Suche aus „Beispiele“ ohne Treffer außerhalb der Nicht-Ziele.
- **AC-02** `devTiles`, `menuPages`, `panelHost`, `GameScene` ohne Verweise auf die Seiten und den Stresslauf; ihre Tests und `tests/projectRules.test.ts` grün.
- **AC-03** CI-Smoke-Test ohne `/gamepad-test.html`; `task check` und `task build` grün, `dist/` enthält nur die bleibenden Seiten.
- **AC-04** `CLAUDE.md`, `docs/roadmap.md`, `docs/projekte/SND-sound.md` und Codemaps nennen die Seiten nicht mehr als vorhanden.
- **AC-05** Das Session-Ergebnis listet alle offenen Planungs-Einträge, die eine entfernte Seite nennen (Stand 2026-11-12: HW1.9, HW1.10, LG1.6, RX1.5, RX1.8, PF1.4, B-194, B-195, B-317, B-394, B-765, B-766, B-769, B-773), mit je einem Satz zur Folge.

## Offene Fragen

Nicht blockierend, Default autonom (Claude, 2026-11-12), Entscheidung 🧑:

- **Folgen für Xbox-Abnahmen und RX1:** HW1.9 (Hörprobe am TV) braucht `soundtest.html`, HW1.10 und B-394 den Bericht aus `gamepad-test.html`, RX1.5/RX1.8 `stress.html`, PF1.4 Schritt 3 `testing.html`. Default: Claude ändert keinen Status fremder Sprints; die Liste aus AC-05 geht in die Ralf-Queue, 🧑 verwirft oder passt an, zusammen mit B-823.
- **Server-Reste** (`/dm`-Abbildung, `/api/report` ohne Absender aus dem Browser): Default bleiben, bis B-823 entschieden ist.

## Notizen

Beschluss 🧑 2026-11-12 (Altlasten-Sprint CU2). Referenzliste erhoben mit `git grep -n -e "<seite>.html"` je Seite und Import-Suche je Modul.
