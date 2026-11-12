# B-824 · Nach dem Papierschnitt verweist kein Code, Test und keine Doku mehr auf alte Pixel-Grafik

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

CU1 (Projekt PAP, Review CU1.6 offen) hat alte Packs, Atlas, `public/sprites/`, `data/assets.json`, `data/sprites.json` und die Generatoren entfernt; MG8 (Natur, Objekte, Icons) ist erledigt, Gerüste und Plantage sind seit CU1.1 Papier. Die Suche vom 2026-11-12 (`git grep -E "grafik-db32|atlas/atlas\.json|tools/buildings|data/assets\.json|data/sprites\.json|public/sprites|sprites/[a-z]+/idle\.png|-32\.png|sCoins"`, ohne erledigte Sprints, Archiv, Entscheidungen, Codemaps) findet noch:

| Rest | Fundstelle | Folge |
|---|---|---|
| Alte Pixel-Pfade im Balancing-Bericht der Workbench | `tools/k3c-workbench/internal/balance/report_html.go:33-42` (`grafik/gebaeude/burg-1-32.png`, `sprites/skeleton/idle.png` …) | `TestHTMLEigenstaendig` (`internal/balance/hints_test.go:50`) rot in `task check:all` (B-817 AC-05, B-787 Notizen). Die Kopie `engine/balance/report_html.go` hat CU1 (#543) schon auf Papier-Pfade umgestellt, die Workbench-Kopie (seit RS2) nicht. |
| Test-Glob auf alte 32-px-Gebäude | `src/scenes/render/hubGrowth.test.ts:10` (`public/grafik/gebaeude/*-32.png`), Testname `buildingSprites.test.ts:25` | Test prüft gegen einen leeren Ordner |
| Kommentare „aus `tools/buildings`“, „32 px“ | `src/scenes/render/buildingSprites.ts:13,67`, `natureSprites.ts:6`, `worldSprites.ts:102`, `scaffoldView.ts:32` | irreführend (laut CU1.5-Ergebnis offen) |
| Task-Quelle | `tools/Taskfile.yml:69` (`soundatlas` › `sources: data/assets.json`) | Neubau erkennt Änderungen an `data/sounds.json` nicht |
| Lizenz- und Release-Texte auf `public/sprites/CREDITS.md` | `LICENSE:5`, `LICENSE-ASSETS:3-4`, `README.md:173`, `docs/release/LIES-MICH.txt:52`, `docs/release/itch-io.md:4,65` | Verweis ins Leere |
| Doku auf `data/sprites.json`, `data/assets.json` | `docs/game-design.md:187`, `docs/rules/monarch.md:69`, `docs/assets/sounds.md:3`, `public/audio/README.md:4,14`, `docs/glossar.md:23,24,228,232`, `docs/arbeitsweise.md:522` | veraltete Quelle |
| Hinweis „Natur, Objekte und Gerüste bleiben bis MG8 Pixel-Grafik“ | `CLAUDE.md` (Abschnitt Struktur, Papierschnitt-Grafik) | veraltet |

Bereits anderswo geplant: `tools/assetkit` (Katalog, Grafik-Slots, Pixel-Credits) in **B-760** (DEV, PAP); `site/img/` (Pixel-Bilder der Pages-Seite) hängt an **B-823** (Frage 4).

## Ziel

Kein Code, Test, Task und keine gültige Doku verweist mehr auf alte Pixel-Grafik; `TestHTMLEigenstaendig` ist grün, der Balancing-Bericht zeigt Papier-Bilder.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Review); 🧑 Ralf liest Balancing-Berichte und Lizenzhinweise.

## Anforderungen

- `internal/balance/report_html.go` der Workbench nutzt dieselben Papier-Pfade und Rahmen wie `engine/balance/report_html.go`; fehlende Grafiken erscheinen weiter als Platzhalter.
- `hubGrowth.test.ts` und `buildingSprites.test.ts` prüfen nur Papier-Assets; Kommentare in `src/scenes/render/` nennen die Papier-Quelle statt `tools/buildings`.
- `tools/Taskfile.yml` › `soundatlas` hat `data/sounds.json` als Quelle.
- Lizenz-, Release- und Fach-Doku nennen statt `public/sprites/CREDITS.md`, `data/sprites.json`, `data/assets.json` die heutigen Quellen (Sound-Herkunft laut `public/audio/README.md` und Seite „Lizenzen & Danksagung“, `data/paper.json`, `data/sounds.json`) oder der Satz entfällt; `CLAUDE.md` ohne den MG8-Satz.
- Die Suche aus „Beispiele“ steht samt Ergebnis im Session-Ergebnis.

## Nicht-Ziele

- `tools/assetkit` und seine Tests (B-760).
- `site/` und `site/img/` (B-823).
- Historische Doku ohne Gebrauchswert als Quelle: `docs/plan-weiterentwicklung.md`, `docs/fragenkatalog.md`, `docs/funde/`, erledigte Sprints, Archiv, `docs/decisions/`.
- Codemaps und `.codemap/state.json` (entstehen neu mit dem Skill `codemap`).
- Offene Tickets, die noch `tools/buildings` oder `data/assets.json` planen (B-435, B-462, B-570, B-571, B-641, B-685, Sprint GRJ2): nur auflisten, kein Statuswechsel.
- Zusammenführen der Kopien `engine/balance` und `tools/k3c-workbench/internal/balance` (Vorschlag als neues Ticket, siehe Offene Fragen).

## Regeln und Einschränkungen

Domäne INF als Aufräum-Arbeit über Domänengrenzen (Grenzfall „Alt-Engine löschen“, `docs/arbeitsweise.md` › Domänen): nur Pfade, Testnamen, Kommentare und Verweise nachziehen, keine neue Funktion. `docs/rules/monarch.md` und `docs/game-design.md` bekommen nur den korrigierten Datei-Verweis, keine Regeländerung. `docs/glossar.md` ist Sammeldatei: Änderung nur im letzten Commit des Sprints. Kein `gen:paper`-Golden in der Session (B-742).

- **Bereich:** `tools/k3c-workbench/internal/balance/report_html.go`, `tools/k3c-workbench/internal/balance/*_test.go`, `src/scenes/render/hubGrowth.test.ts`, `src/scenes/render/buildingSprites.ts`, `src/scenes/render/buildingSprites.test.ts`, `src/scenes/render/natureSprites.ts`, `src/scenes/render/worldSprites.ts`, `src/scenes/render/scaffoldView.ts`, `tools/Taskfile.yml`, `LICENSE`, `LICENSE-ASSETS`, `README.md`, `docs/release/**`, `docs/game-design.md`, `docs/rules/monarch.md`, `docs/assets/sounds.md`, `public/audio/README.md`, `docs/arbeitsweise.md`, `CLAUDE.md`
- **Abhängig von:** CU1
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

- `git grep -n -E "grafik-db32|atlas/atlas\.json|tools/buildings|data/assets\.json|data/sprites\.json|public/sprites|sprites/[a-z]+/idle\.png|-32\.png|sCoins" -- ':!docs/sprints' ':!docs/backlog' ':!docs/decisions' ':!docs/plan-weiterentwicklung.md' ':!docs/fragenkatalog.md' ':!docs/funde' ':!**/codemap.md' ':!codemap.md' ':!.codemap' ':!graphify-out' ':!site' ':!tools/assetkit'` → keine Treffer (außer Kommentaren, die ausdrücklich die Entfernung in CU1 beschreiben).
- `check_run` › `go test ./internal/balance/...` in `tools/k3c-workbench` nach `task gen:paper` → `TestHTMLEigenstaendig` grün.

## Ausnahme- und Fehlerfälle

- Ein Papier-Asset für eine Berichtsgrafik fehlt (z. B. vor `gen:paper`) → der Bericht zeigt den Platzhalter wie heute, kein Absturz.
- Ein Verweis erweist sich als noch gebraucht → bleibt, Fundstelle und Grund im Ergebnis.

## Akzeptanzkriterien

- **AC-01** `tools/k3c-workbench/internal/balance/report_html.go` nutzt die Papier-Pfade aus `engine/balance/report_html.go`; `TestHTMLEigenstaendig` grün (nach `task gen:paper`, Lauf im Ergebnis belegt).
- **AC-02** `hubGrowth.test.ts` und `buildingSprites.test.ts` ohne alte 32-px-Pfade und grün; Kommentare in `src/scenes/render/` ohne `tools/buildings`.
- **AC-03** `tools/Taskfile.yml` › `soundatlas` mit Quelle `data/sounds.json`.
- **AC-04** Lizenz-, Release- und Fach-Doku sowie `CLAUDE.md` ohne Verweise auf `public/sprites/`, `data/sprites.json`, `data/assets.json` und den MG8-Satz; die Suche aus „Beispiele“ ohne Treffer.
- **AC-05** Das Session-Ergebnis listet die offenen Tickets und Sprints, die noch alte Werkzeuge planen (Nicht-Ziele), für die Ralf-Queue.

## Offene Fragen

Autonom mit Default entschieden (Claude, 2026-11-12, Meldung im Nachgang):

- **Doppelte Balancing-Pakete:** `engine/balance` (75 Dateien) und `tools/k3c-workbench/internal/balance` (49 Dateien) laufen auseinander. Default: hier nur die Pfade angleichen; das Zusammenführen schlägt die Review-Session als eigenes Ticket vor.
- **Glossar:** Einträge „Asset-Slot“ und „Slot-Register“ auf Sound-Slots (`data/sounds.json`) umschreiben statt löschen; „Spielerfarbe“ auf `data/paper.json`.

## Notizen

Beschluss 🧑 2026-11-12 (Altlasten-Sprint CU2). Abgrenzung zu CU1 und B-760 per Suche und CU1.5-Ergebnis geprüft; der Teil `src/scenes/render` aus dem CU1.5-Ergebnis („gehört zu B-760“) wandert hierher, weil B-760 nur `tools/assetkit` beschreibt.
