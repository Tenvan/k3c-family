# BAL1.1 · Szenario-Matrix, zwei Bot-Profile, Kennzahlen als JSON

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** bal1/1-bots-kennzahlen
- **Abhängig von:** –
- **Tickets:** B-099
- **Kriterien:** AC-01, AC-02

## Ziel

Ein Befehl spielt eine Szenario-Matrix (Seeds × Spielerzahl × Bot-Profil × Biom/Tiefe × Tage) headless mit mindestens zwei deterministischen Bot-Profilen und schreibt die Kennzahlen je Lauf als JSON; gleiche Eingaben ergeben byte-gleiche Ausgabe.

## Kontext

- **Ort geklärt (🧑 2026-10-04, Spec-Freigabe):** (b) Tool in k3c-dev unter `tools/k3c-dev/internal/` (Domäne SRV, Ausnahme mit der Spec freigegeben). Paket `tools/k3c-dev/internal/balance/`, Befehl `internal/balance/cmd` (EXE `bin/k3c-balance`), Task `balance:run`. Die Alternative (a) `engine/balance` + `cmd/k3c-balance` entfällt.
- Anforderungen: B-099 › Anforderungen (Szenario-Matrix, Bot-Profile, Kennzahlen je Lauf); Bericht und Korridore folgen in BAL2, weitere Profile und Sensitivität in BAL3.
- **Simulation:** `engine/sim` (`CreateIsland(seed, depths, cycleSpeed)` und `StepIsland(isl, commands, dt)` in `island.go`; einzelne Welt `CreateWorld`/`Step` in `world.go`). Takt wie der Server: 30 Hz, `dt = 1/30` (`tools/k3c-dev/internal/enginetools/run.go` › `TickHz`). Ereignisse stehen je Tick in `w.Events` (z. B. `built`, `wave`, `destroyed`, `recruited`, `gathered`, `chest`) und sind die Quelle der Kennzahlen.
- **Bots nur über `PlayerCommand`** (`engine/sim/types.go`: `MoveX`, `Sprint`, `Pay`; nach S1 auch Schlag/Skill). Ein Bot liest den Zustand der Welt (Positionen, Bauplätze, Gold) und entscheidet daraus seine Eingabe, er verändert nie die Welt direkt.
- **Profile (mindestens zwei):** Pflichtliste Q19: passiv, sparsam, Mauern zuerst, Wirtschaft zuerst, Koop 2/4; BAL1 nimmt zwei davon (Vorschlag: „passiv“ = steht am Hub, zahlt nichts, und „sparsam“ = zahlt erst Mauern, dann Bauern, wenn Gold ≥ Preis), die übrigen kommen in BAL3.
- **Kennzahlen (B-099):** Tick der ersten Gold-Schwelle, Gold und Material zu Dämmerungsbeginn je Tag, Zeit bis zur ersten Mauer und zum ersten Bogen, Verluste und Überleben je Welle, Tick des Burgfalls, Einnahmen/Ausgaben je Tag, erreichte Tiefe. Fehlt eine Quelle im Zustand oder in den Ereignissen, Kennzahl als `null` und Ticket (SIM), nicht im Sim-Code nachrüsten.
- Determinismus: Seeds als feste Liste, `engine/rng` für jede Bot-Entscheidung mit Zufall, keine Wanduhr, keine Map-Iteration ohne Sortierung (Check aus F2/B-138 beachten). Ausgabe-JSON mit fester Feldreihenfolge (Structs, keine Maps).
- Abbruch eines Laufs (Panic/Fehler) → Lauf `ungültig` mit Seed in der Ausgabe, der Rest läuft weiter.

## Erlaubte Dateien

- `tools/k3c-dev/internal/balance/` (neu, mit Tests, Befehl unter `cmd/`), nach Klärung (b) statt `engine/balance/` und `cmd/k3c-balance/`
- `Taskfile.yml` (nur ein Task zum Starten, z. B. `balance:run`; EXE mit festem Pfad, kein `go run`)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Replay-Format (BAL1.2), k3c-dev (BAL1.3), Zielkorridore und Ampel-Bericht (BAL2), weitere Profile und Sensitivität (BAL3), Änderungen an `engine/sim` oder `data/`.

## Schritte

1. Klärung (a)/(b) in den Kontext übernehmen. Branch anlegen, `Status: in Arbeit`.
2. Matrix-Typ und Läufer: je Szenario Insel erzeugen, Ticks mit Bot-Eingaben rechnen, Ereignisse einsammeln.
3. Zwei Bot-Profile mit Tests (Profil zahlt bzw. zahlt nicht; 2 Spieler gleichzeitig).
4. Kennzahlen aus Zustand und Ereignissen, Ausgabe JSON je Lauf.
5. Test: zwei Läufe mit gleichen Seeds und Parametern → byte-gleiche Ausgabe; ungültiger Lauf erscheint mit Seed.
6. `task check:go`. Ergebnis mit Laufzeit für 10 Seeds × 5 Tage, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-01: Test belegt byte-gleiche Kennzahlen bei gleichen Seeds, Parametern und Daten.
- [x] AC-02: zwei Bot-Profile und die Kennzahlen aus B-099 umgesetzt und getestet (fehlende Quellen als `null` mit Ticket).
- [x] `task check:go` grün; keine Datei > 400 Zeilen, keine Funktion > 60 Zeilen.

## Prüfen

```bash
task check:go
```

## Ergebnis

- **Ort:** `tools/k3c-dev/internal/balance/` (`run.go` Matrix und Läufer, `bots.go` Profile, `metrics.go` Kennzahlen, `cmd/main.go` Befehl), Task `task balance:run -- --seeds 10 --players 2 --bots saver,passive --days 5 [--depths 0] [--gold-threshold N] [--out datei.json]`; EXE fest unter `bin/k3c-balance`.
- **AC-01** umgesetzt, geprüft mit `TestGleicheMatrixGleicheBytes` (zwei Läufe derselben Matrix → byte-gleiches JSON). Ausgabe nur aus Structs, keine Wanduhr, Bots ohne Zufall.
- **AC-02** umgesetzt, geprüft mit `TestProfileZahlenOderNicht` (passiv baut keine Mauer, sparsam baut sie; je 2 Spieler), `TestSparsamZweiSpielerZahlenBeide` (beide Monarchen zahlen gleichzeitig), `TestKennzahlenGefuellt`, `TestUngueltigerLaufMitSeed` (Panic und unbekannte Tiefe → `valid: false` mit Seed, Rest läuft weiter), `TestMatrixPrueft`. Profile `passive` (steht am Hub) und `saver` (nachts an der Burg; tags erst nächste unbezahlte Mauer, dann Landstreicher, nur wenn das Gold den Rest des Preises deckt). Kennzahlen je Lauf: `goldThresholdTick`, `firstWallTick`, `firstBowTick` (erstes `armed`), `castleFallTick`, `reachedDepth`, je Tag `duskTick`/`goldAtDusk`/`stockAtDusk`/`income`/`expense` (Gold-Fluss aus der Bestandsänderung je Tick), je Welle `enemies`, `troopsAtStart`, `troopsLost` (Differenz der Truppen-IDs ohne Landstreicher), `buildingsDestroyed`, `playersDown`, `survived`. Keine Kennzahl musste `null` bleiben; die Bedeutung der Gold-Schwelle ist offen → **B-203** (Frage, REG).
- `task check:go`, `task check:dev`, `task check` grün (2026-10-04, Worktree).
- **Laufzeit:** 10 Seeds × 5 Tage × 2 Spieler × 2 Profile (20 Läufe, je 144 000 Ticks) in 12,9 s, also ca. 6,5 s für 10 Seeds × 5 Tage je Profil (Entwickler-Rechner).
- Beobachtung (nur Bericht, keine Bewertung): sparsam, Seeds 1–10, Burg hält Nacht 1–5 in 5 von 10 Seeds; passiv verliert die Burg in Nacht 1.
