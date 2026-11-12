# CU2 · INF · Altlasten: Testseiten des Browser-Clients und Pixel-Reste entfernen

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** INF
- **Reife:** bereit
- **Tickets:** B-822, B-824
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Nach CU1 und dem Wechsel auf das native Spiel (Entscheidung 008) liegen zwei Altlasten im Repo: sechs Test- und Werkzeugseiten des Browser-Clients aus der Xbox/Edge-Zeit samt `src/stress/` (B-822) und Reste alter Pixel-Grafik in Code, Tests, Task und Doku, darunter der rote `TestHTMLEigenstaendig` in `task check:all` (B-824). Beschluss 🧑 2026-11-12: als Sprint einplanen. Das Ablösen des ganzen Browser-Clients (B-823) ist eine eigene Entscheidung und gehört nicht in diesen Sprint.

## Ziel

Das Repo enthält keine Testseiten des Browser-Clients und keine Verweise auf alte Pixel-Grafik mehr.

Am Ende sichtbar: Spielmenü › Seiten ohne tote Kacheln, `dist/` nur mit `index.html`, `game.html`, `monitor.html`, `lizenzen.html`; `TestHTMLEigenstaendig` grün; die Suchen aus B-822 und B-824 ohne Treffer.

## Beteiligte und Zielgruppen

Claude (Umsetzung CU2.1, CU2.2; Review CU2.3); 🧑 Ralf entscheidet im Nachgang über die Folgen für Xbox-Abnahmen und alte Planung (Listen aus `B-822/AC-05` und `B-824/AC-05`, nicht blockierend).

## Anforderungen

`B-822 › Anforderungen`, `B-824 › Anforderungen`. Sprint-eigen: Gelöscht wird nur, was die Suche als ungenutzt belegt; Suchbefehl und Ergebnis stehen in den Session-Ergebnissen.

## Nicht-Ziele

Browser-Client als Ganzes, Server-Auslieferung, Pages, Workbench „Im Spiel ansehen“ (B-823, Entwurf); Server-Endpunkte `/api/dev`, `/api/report` und `docs/protocol.md`; `tools/assetkit` (B-760); `site/`; Statuswechsel fremder Sprints und Tickets; Sim-, Balance- und Gen-Golden-Läufe.

## Regeln und Einschränkungen

Domäne INF nach dem Grenzfall „Alt-Engine löschen“ (`docs/arbeitsweise.md` › Domänen): löschen und Verweise nachziehen, keine neuen Funktionen. Beide Tickets Klasse A (Schuld, INF, ohne Spielregeln, Protokoll, Spielstand, Abhängigkeiten). Sessions nacheinander auf einem Branch `sprint/cu2`. Review vor dem Commit nach Risiko-Regel (`CLAUDE.md`): CU2.1 ändert CI und mehr als 150 Zeilen. `docs/glossar.md` nur im letzten Commit des Sprints.

- **Bereich:** `dm.html`, `stress.html`, `texturetest.html`, `testing.html`, `soundtest.html`, `gamepad-test.html`, `src/stress/**`, `src/tools/**`, `src/scenes/GameScene.ts`, `src/scenes/menuPages.ts`, `src/scenes/menuPages.test.ts`, `src/scenes/render/hubGrowth.test.ts`, `src/scenes/render/buildingSprites.ts`, `src/scenes/render/buildingSprites.test.ts`, `src/scenes/render/natureSprites.ts`, `src/scenes/render/worldSprites.ts`, `src/scenes/render/scaffoldView.ts`, `src/core/panelHost.ts`, `src/core/panelHost.test.ts`, `src/core/shell.test.ts`, `src/core/runConfig.ts`, `tests/projectRules.test.ts`, `tests/textureLayout.test.ts`, `data/stress.json`, `public/audio-test/**`, `public/audio/README.md`, `.github/workflows/ci.yml`, `tools/Taskfile.yml`, `tools/k3c-workbench/internal/balance/report_html.go`, `tools/k3c-workbench/internal/balance/*_test.go`, `LICENSE`, `LICENSE-ASSETS`, `README.md`, `CLAUDE.md`, `docs/roadmap.md`, `docs/projekte/SND-sound.md`, `docs/release/**`, `docs/game-design.md`, `docs/rules/monarch.md`, `docs/assets/sounds.md`, `docs/arbeitsweise.md`, `codemap.md`, `src/**/codemap.md`
- **Abhängig von:** CU1
- **Braucht 🧑:** –
- **Sperre:** taskfile

## Beispiele

`B-822 › Beispiele`, `B-824 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-822 › Ausnahme- und Fehlerfälle`, `B-824 › Ausnahme- und Fehlerfälle`. Ein Modul oder Verweis wird doch noch gebraucht → bleibt, Fundstelle im Session-Ergebnis.

## Akzeptanzkriterien

- **AC-01** Seiten, `src/stress/`, `data/stress.json`, `public/audio-test/` und ihre Module fehlen, Suche ohne Treffer (`B-822/AC-01`).
- **AC-02** Kacheln, Menü, Panel und `GameScene` ohne Verweise, Tests grün (`B-822/AC-02`).
- **AC-03** CI-Smoke-Test angepasst, `task check` und `task build` grün, `dist/` nur mit bleibenden Seiten (`B-822/AC-03`).
- **AC-04** `CLAUDE.md`, Roadmap, SND und Codemaps ohne die Seiten (`B-822/AC-04`).
- **AC-05** Liste betroffener Planung für die Ralf-Queue (`B-822/AC-05`).
- **AC-06** Workbench-Bericht mit Papier-Pfaden, `TestHTMLEigenstaendig` grün (`B-824/AC-01`).
- **AC-07** Render-Tests und Kommentare ohne alte 32-px-Grafik (`B-824/AC-02`).
- **AC-08** `soundatlas` mit Quelle `data/sounds.json` (`B-824/AC-03`).
- **AC-09** Doku und `CLAUDE.md` ohne Pixel-Verweise, Suche ohne Treffer (`B-824/AC-04`).
- **AC-10** Liste alter Planung mit Pixel-Werkzeugen für die Ralf-Queue (`B-824/AC-05`).

## Offene Fragen

keine blockierenden. Defaults (Claude, 2026-11-12, Meldung im Nachgang) stehen in `B-822 › Offene Fragen` (kein Statuswechsel fremder Sprints, Server-Reste bleiben) und `B-824 › Offene Fragen` (doppelte Balancing-Pakete nur angleichen, Glossar umschreiben).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| CU2.1 | `CU2.1-testseiten-entfernen.md` | Umsetzung | autonom | offen |
| CU2.2 | `CU2.2-pixel-reste.md` | Umsetzung | autonom | offen |
| CU2.3 | `CU2.3-review.md` | Review | autonom | offen |

Keine eigenen Sim- oder Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
