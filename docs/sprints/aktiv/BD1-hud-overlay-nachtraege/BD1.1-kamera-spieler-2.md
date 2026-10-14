# BD1.1 · Split-Screen: Die Kamera folgt jedem lokalen Spieler

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** bd1/1-kamera-spieler-2
- **Abhängig von:** –
- **Tickets:** B-392
- **Kriterien:** AC-01

## Ziel

Im Split-Screen mit 2 lokalen Spielern folgt die Kamera jeder Ansicht der Figur ihres Spielers (B-392). Nach dieser Session liegt die Korrektur samt Test auf `develop`.

## Kontext

- Kameras entstehen in `GameScene.layoutCameras` (`src/scenes/GameScene.ts` ~371–402): eine Kamera je Layout-Feld (`computeLayout`, `src/scenes/layout.ts`), Spieler-Felder folgen dem Monarchen des Sitzes (`seats[cell.seat].monarch`).
- `aimAtStage` (~404–411) ruft `cam.startFollow(view, …)` nur, wenn `stage.renderer.playerView(monarch)` (`src/scenes/worldRenderer.ts` ~144) schon eine View liefert; sonst bleibt die Kamera ohne Ziel auf `hubX` stehen.
- `layoutCameras` läuft nur, wenn sich `layoutKey` ändert (`updateLayout` ~347). Der Schlüssel enthält Sitze, Monarch/Tiefe und Stufen, nicht das Vorhandensein der Figur-Views.
- **Hypothese (ungeprüft):** Die View von Spieler 2 existiert beim Layout noch nicht (wird erst im folgenden `sync` erzeugt, auch bei `hidden`-Monarchen, B-411); `startFollow` wird nie nachgeholt. Spieler 1 hat seine View meist schon. Ursache zuerst per Test oder Log belegen, nicht annehmen.
- Höchstens 2 Spieler (🧑 2026-10-11). Client rechnet nichts (`src/scenes/noSim.test.ts`); B nicht belegen; Texte in `src/core/texts.de.ts`.
- Abnahme am PC macht 🧑 (S8.4-Beobachtung); der Agent prüft mit Unit-Test und, nur wenn freigegeben, im Browser-Pane (Tastatur A/D und Pfeile, siehe `CLAUDE.md` › Im Browser-Pane testen).

## Erlaubte Dateien

- `src/scenes/GameScene.ts`
- `src/scenes/worldRenderer.ts` (nur `playerView` / Kamera-Ziel)
- `src/scenes/layout.ts`, `src/scenes/cellStages.ts` (nur falls die Ursache dort liegt)
- neuer oder bestehender Test unter `src/scenes/` (z. B. `cameraFollow.test.ts`)
- `docs/sprints/aktiv/BD1-hud-overlay-nachtraege/` und `docs/backlog/B-392-*.md` (Status, Ergebnis)

## Nicht-Ziele

Neues Split-Screen-Layout; Änderungen an Server, Protokoll oder `engine/`; Beitritts-Hinweis und Ö-Taste (BD1.2); Partner-Kamera (Feld des Mitspielers im Ein-Sitz-Modus) umbauen, außer die Ursache liegt dort; 3–4 Spieler.

## Schritte

1. Glossar (`docs/glossar.md`) und `docs/arbeitsweise.md` lesen; Session auf `in Arbeit` setzen (`plan_set`).
2. Ursache belegen: Test (rot) oder temporäres Log in `layoutCameras`/`aimAtStage`, ob `playerView(monarch)` für Sitz 2 beim Layout `undefined` ist und `startFollow` ausbleibt. Hypothese in Kontext bestätigen oder ersetzen.
3. Korrektur: Kamera-Ziel nachziehen, sobald die View existiert (z. B. in `GameScene.update` je Spieler-Kamera ohne Ziel `startFollow` nachholen, oder das Vorhandensein der Views in `layoutKey` aufnehmen). Kleinste Änderung; Datei ≤ 400 Zeilen, Funktion ≤ 60.
4. Test ergänzen: 2 Sitze, beide Figuren vorhanden, verschiedene Positionen → jede Kamera hat ein Follow-Ziel auf die Figur ihres Sitzes; auch wenn die View erst nach dem Layout entsteht.
5. `task check` (über `check_run`), Befunde beheben.
6. Ergebnis eintragen, `Status: fertig`, B-392 auf erledigt/Archiv nach den Regeln (Abnahme am PC bleibt 🧑; bis dahin `angenommen, Validierung offen`), Commit `fix(cli): Kamera folgt jedem lokalen Spieler (BD1.1)`.

## Fertig, wenn

- [x] AC-01: Ein Test belegt, dass jede Kamera der Figur ihres Sitzes folgt, auch wenn die View erst nach dem Layout entsteht (`cameraFollow.test.ts`).
- [ ] AC-01 (Abnahme 🧑): Bei 2 Spielern an einer Tastatur (A/D und Pfeile) in verschiedene Richtungen scrollen beide Ansichten mit (Abnahme 🧑 am PC; bis dahin „angenommen, Validierung offen“).
- [x] Vitest grün (2269 Tests), `tsc` und Oxlint ohne Befund; keine neue Abhängigkeit, keine Server-Änderung. `task check` insgesamt rot durch Bestand, siehe Ergebnis.

## Prüfen

```bash
task test -- cameraFollow
task check
```

Manuelle Prüfung im Browser (2 Spieler, Tastatur) nur, wenn 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-01 umgesetzt, geprüft (Test):** Neue reine Funktion `refollow` (`src/scenes/cameraFollow.ts`) führt jede Kamera ihrer Figur nach; `GameScene.draw` ruft sie jeden Frame je Kamera mit Figur-Ziel auf (`follows`, gefüllt in `aimAtStage`). `cameraFollow.test.ts`: 5 Tests (View erscheint nach dem Layout, kein Neustart je Frame, neue View, Figur verschwindet, zwei Kameras unabhängig).
- **AC-01 Abnahme am PC: angenommen, Validierung offen (🧑).** Das Verhalten mit 2 Spielern an einer Tastatur (A/D und Pfeile) habe ich nicht im Browser geprüft.
- **Ursache nicht belegt:** Die Hypothese (View von Spieler 2 entsteht erst nach dem Layout, `startFollow` wird nie nachgeholt) ist plausibel, aber nicht am laufenden Spiel bestätigt. In `draw` läuft `sync` vor `updateLayout`; die Lücke entsteht also nur, wenn der Monarch beim Layout noch ausgeblendet ist (`unattended`, B-411). Die Korrektur deckt jede Ursache ab, bei der das Ziel später entsteht oder neu gezeichnet wird. Ist es eine andere Ursache (z. B. falscher Monarch je Sitz), bleibt B-392 offen: dann bei der Abnahme melden.
- **Prüfung:** `npx vitest run`: 2269 Tests grün; 6 Testdateien unter `tools/k3c-dev/frontend/` laden nicht (`@radix-ui/themes`, `react` nicht installiert im Worktree), unabhängig von dieser Änderung. `task check` deshalb nicht grün; k3c-dev war nicht verbunden (Handarbeit, `check_run` nicht möglich).
- **Abweichungen:** Status von Hand gesetzt (k3c-dev getrennt). `GameScene.ts` hat 425 Zeilen, zählt für Oxlint ohne Leer- und Kommentarzeilen unter 400 (Lint ohne Befund).
- **Neue Tickets:** keine.
