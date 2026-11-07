# US1 · CLI, SRV, INF · Spiel lädt Papierschnitt-Grafik, alte Pixel-Assets und Gebäude-Renderer entfernt

- **Status:** erledigt
- **Projekt:** PAP
- **Domäne:** CLI, SRV, INF
- **Reife:** bereit
- **Tickets:** B-699
- **Start-Commit:** 094be01c
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-07, autonom, Claude

## Ausgangslage

Welle 3 nach RD1, MC1, WB1, RV1 und MG1–MG8. Das Spiel lädt heute Pixel-Sheets über `data/sprites.json`, `public/atlas/` und `public/grafik/` (`src/scenes/render/`).

## Ziel

Das Spiel lädt die Papierschnitt-PNGs über die Gruppen-Indizes `public/grafik/<gruppe>/index.paper.json`; Server-Watch und Server-Check kennen die neuen Dateien; Doku ist nachgezogen.

Am Ende sichtbar: Spiel im PC-Browser im Papierschnitt-Look (Abnahme in PS1).

## Beteiligte und Zielgruppen

Spieler sehen das Spiel im Papierschnitt-Look; Claude setzt um; 🧑 nimmt in PS1 ab.

## Anforderungen

Entscheidung 005 › Umsetzung (Welle 3); B-701 D-09, D-11.

## Nicht-Ziele

Skalierung Xbox/Mobile (Folgeticket); Entfernen alter Grafik (CU1); Abnahme (PS1).

## Regeln und Einschränkungen

Sessions nacheinander nach Domäne: CLI → SRV → INF → Doku. Engine-Schatten nach B-701 D-09 (Default Phaser-Filter, sonst Sprite-Kopie).

## Beispiele

- Spiel startet → LoadScene liest die Gruppen-Indizes, lädt Manifeste und PNG-Sheets, Krieger erscheint als Papierschnitt mit `idle`/`lauf`/`angriff`.
- Mauer Stufe 3 gebaut → Textur `gebaeude-paper/mauer/stand.s3.png`.
- Monarch beritten → kombinierte Animation `schritt`/`trab`/`galopp` des Monarch-Assets (Pferd im Bild).

## Ausnahme- und Fehlerfälle

- Manifest oder PNG fehlt → Ladefehler mit Dateiname wie bisher.
- Rolle ohne Papier-Asset → alte Grafik bleibt (bis CU1), Zuordnung in `data/paper.json`.
- `-check-data` findet eine im Index genannte Datei nicht → Fehler, alter Stand bleibt aktiv.

## Akzeptanzkriterien

- **AC-01** Das Spiel lädt jede Figur, jedes Gebäude und jeden Hintergrund aus den Gruppen-Indizes (Test in `tests/projectRules.test.ts`: kein Lade-Pfad auf `atlas/` oder `grafik-db32/`).
- **AC-02** `k3c-server -check-data` meldet fehlende Dateien aus einem Gruppen-Index, der alte Stand bleibt aktiv (Go-Test); der Watch ignoriert `*.paper.json` (Go-Test).
- **AC-03** `task check:all` grün.
- **AC-04** CLAUDE.md, `docs/arbeitsweise.md` und Codemaps beschreiben Papierschnitt, Workbench-Bereiche und den Wegfall des Studios.

## Offene Fragen

keine (Defaults als D-US1-xx in B-701)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| US1.1 | `US1.1-loader.md` | Umsetzung | autonom | fertig |
| US1.2 | `US1.2-server-check-watch.md` | Umsetzung | autonom | fertig |
| US1.3 | `US1.3-tasks-ci.md` | Umsetzung | autonom | fertig |
| US1.4 | `US1.4-doku-codemaps.md` | Umsetzung | autonom | fertig |
| US1.5 | `US1.5-review.md` | Review | autonom | fertig |

## Abnahme

2026-11-07, autonom (Claude). AC-01 US1.1, AC-02 US1.2 und US1.5, AC-03 US1.3 (`task check:all` Exit 0; Race lokal ohne gcc übersprungen), AC-04 US1.4.
Behoben aus dem Review: W1–W4, K5, K10; zurückgestellt D-US1-06. Defaults D-US1-01 bis D-US1-06 in B-701.
Neue Tickets: keine. Sichtprüfung am PC-Browser und Xbox in PS1.
