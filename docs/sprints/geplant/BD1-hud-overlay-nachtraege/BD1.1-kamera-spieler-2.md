# BD1.1 · Split-Screen: Die Kamera folgt jedem lokalen Spieler

- **Status:** offen
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
- `docs/sprints/geplant/BD1-hud-overlay-nachtraege/` und `docs/backlog/B-392-*.md` (Status, Ergebnis)

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

- [ ] AC-01: Ein Test belegt, dass bei 2 lokalen Sitzen jede Spieler-Kamera der Figur ihres Sitzes folgt, auch wenn die View erst nach dem Layout entsteht.
- [ ] AC-01: Bei 2 Spielern an einer Tastatur (A/D und Pfeile) in verschiedene Richtungen scrollen beide Ansichten mit (Abnahme 🧑 am PC; bis dahin „angenommen, Validierung offen“).
- [ ] `task check` grün; keine neue Abhängigkeit, keine Server-Änderung.

## Prüfen

```bash
task test -- cameraFollow
task check
```

Manuelle Prüfung im Browser (2 Spieler, Tastatur) nur, wenn 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
