# B-787 · Workbench-Tests bleiben mit -short unter 3 s je Test, der volle Lauf ist grün

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ42
- **Projekt:** WZG
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Messung 2026-11-11 (develop a806320f+, `go test -short -count=1 ./...` in `tools/k3c-workbench`, Entwicklungs-PC): 150 s Wanduhr, 865 s Paketzeit, **103 Tests brauchen trotz `-short` 3 s oder mehr** (zusammen 2021 s Testzeit). Langsamste Pakete: `internal/mcpsrv` 131 s, `internal/planning` 125 s (allein 33 s), `internal/paperassets` 88 s, `internal/review` 67 s, `internal/assetsui` 61 s. Im vollen Lauf waren vier Pakete rot: `simexe` (TestBotZeitlimit), `botdev` (TestBotGeraeteSteuernMonarchen, TestBeobachterOhnePlatz), `balance` (TestHTMLEigenstaendig: Grafik fehlt ohne gen:paper), `mcpsrv` (TestRunProcessExitUndAusgabe). In den Logs der Workbench war `wb-gotest` der teuerste Schritt von `check:diff` (Median 181 s, bis 273 s); 183 von 322 `check:diff`-Läufen endeten rot.

## Ziel

Ein geändertes Workbench-Paket prüft `check:diff` in Sekunden, und der volle Lauf (`check:dev`) ist verlässlich grün.

## Beteiligte und Zielgruppen

Claude und andere Agenten (jede Session wartet auf diese Tests), 🧑 Ralf (Laufzeit, rote Läufe).

## Anforderungen

- Kein Test der Workbench braucht mit `-short` länger als etwa 3 s (Regel B-619); lange Tests (echte Git-Repos, `go build`, CLI-Aufrufe von `tools/paper`, Server-Starts) beginnen mit `testing.Short()`-Skip oder teilen sich ihre Vorbereitung je Paket.
- Die vier roten Tests sind behoben oder mit Grund übersprungen (Last, fehlende Grafik).

## Nicht-Ziele

Neue Funktionen der Workbench; die gesperrten Sim- und Balance-Läufe (Beschluss 2026-11-11).

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › Tempo im Lauf (Lange Tests, B-619), B-704 (lastabhängige Tests). Klasse A (Problem, DEV).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `tools/k3c-workbench/internal/assetsui/**`, `tools/k3c-workbench/internal/paperassets/**`, `tools/k3c-workbench/internal/review/**`, `tools/k3c-workbench/internal/simrun/**`, `tools/k3c-workbench/internal/botdev/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –

## Beispiele

Änderung in `internal/planning` → `task check:diff` fährt `go test -short ./internal/planning` in unter 10 s statt 33 s.

## Ausnahme- und Fehlerfälle

Ein Test braucht die echte CLI oder ein echtes Git-Repo → er läuft nur ohne `-short` (`check:dev`, Review, CI).

## Akzeptanzkriterien

- **AC-01** `go test -short -count=1 -json ./...` in `tools/k3c-workbench`: kein Test mit `Elapsed` ≥ 3 s, Messung in den Notizen.
- **AC-02** Jedes Paket der Workbench läuft allein mit `-short -count=1` in höchstens 10 s.
- **AC-03** `check_run task:check:dev` zweimal nacheinander grün.

## Offene Fragen

Warum brauchen einzelne Tests in `assetsui` und `paperassets` 40 bis 57 s (vermutet: jeder Test startet die CLI von `tools/paper` oder kopiert ein Test-Repo; ungeprüft)? Klärt die Umsetzung (WZ42.2), nicht blockierend.

## Notizen

Zum Vergleich Wurzel-Modul (`-short -count=1`): 57 s Wanduhr; `engine/net` 29 s (TestStateGleichOrakelAlleBiome 17,8 s), `engine/balance` 27 s, `engine/level` TestRustWieGo 4,2 s (fällt mit B-779 weg). `tools/assetkit` 10 s. Gemessen von Claude beim Umbau von `check:diff` (`tools/checkdiff`).

2026-11-11, Abnahme-Ticket B-817 (`task check:all`, 462 s): einziger roter Test `TestHTMLEigenstaendig` (`internal/balance`). Ursache belegt: `report_html.go` bettet Pixel-Grafik aus Pfaden ein, die mit CU1 entfernt wurden (`grafik/gebaeude/burg-1-32.png`, `mauer-1-32.png`, `turm-1-32.png`, `grafik/16x16-small-and-medium-coin-animation/…`, `sprites/skeleton/idle.png`); auch nach `gen:paper` fehlen sie. Vorschlag: auf Papier-Assets aus `public/grafik` umstellen oder fehlende Bilder auslassen. Die drei übrigen roten Tests der ersten Messung (`simexe`, `botdev`, `mcpsrv`) waren in diesem Lauf grün (vermutet: lastabhängig, B-704).

Messwerte vorher/nachher (WZ42.2, `go test -short -count=1 ./...` in `tools/k3c-workbench`): Tests ≥ 3 s 97 → 9, Wanduhr 123 → 60 s, Bereichspakete (`assetsui`, `paperassets`, `review`) allein ≤ 3,2 s, `check:diff` 162 → 32 s. Rest außerhalb des Sprint-Bereichs: 5 Tests `mcpsrv`, 4 Tests `planning`, Pakete 58 s und 40 s allein → B-830.

Voller Lauf (WZ42.4, `task check:dev` auf frischem Worktree nach `task install` und `gen:paper`): Lauf 1 grün in 665 s (`mcpsrv` 52 s, `planning` 145 s, `services` 4 s), Lauf 2 rot nur in zwei Katalog-Vitests unter Last (→ B-837), Lauf 3 und 4 siehe WZ42.4 › Ergebnis.
