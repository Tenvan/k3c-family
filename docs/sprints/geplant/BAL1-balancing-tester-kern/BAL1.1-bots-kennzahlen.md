# BAL1.1 · Szenario-Matrix, zwei Bot-Profile, Kennzahlen als JSON

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** bal1/1-bots-kennzahlen
- **Abhängig von:** –
- **Tickets:** B-099
- **Kriterien:** AC-01, AC-02

## Ziel

Ein Befehl spielt eine Szenario-Matrix (Seeds × Spielerzahl × Bot-Profil × Biom/Tiefe × Tage) headless mit mindestens zwei deterministischen Bot-Profilen und schreibt die Kennzahlen je Lauf als JSON; gleiche Eingaben ergeben byte-gleiche Ausgabe.

## Kontext

- **Vor dem Start mit 🧑 klären** (Offene Frage der Spec, nicht durch Beschlüsse beantwortet): Wo lebt das Werkzeug? (a) Paket `engine/balance` mit Befehl `cmd/k3c-balance` und Task `balance:run` (Vorschlag der Planung: die Logik bleibt in SIM, k3c-dev nutzt das Paket für die Wiedergabe in BAL1.3); (b) Tool in k3c-dev (`tools/k3c-dev/`, Domäne SRV). Ohne Antwort: Session `blockiert`, nicht raten. Die erlaubten Dateien unten gelten für (a); bei (b) entsprechend unter `tools/k3c-dev/internal/`.
- Anforderungen: B-099 › Anforderungen (Szenario-Matrix, Bot-Profile, Kennzahlen je Lauf); Bericht und Korridore folgen in BAL2, weitere Profile und Sensitivität in BAL3.
- **Simulation:** `engine/sim` (`CreateIsland(seed, depths, cycleSpeed)` und `StepIsland(isl, commands, dt)` in `island.go`; einzelne Welt `CreateWorld`/`Step` in `world.go`). Takt wie der Server: 30 Hz, `dt = 1/30` (`tools/k3c-dev/internal/enginetools/run.go` › `TickHz`). Ereignisse stehen je Tick in `w.Events` (z. B. `built`, `wave`, `destroyed`, `recruited`, `gathered`, `chest`) und sind die Quelle der Kennzahlen.
- **Bots nur über `PlayerCommand`** (`engine/sim/types.go`: `MoveX`, `Sprint`, `Pay`; nach S1 auch Schlag/Skill). Ein Bot liest den Zustand der Welt (Positionen, Bauplätze, Gold) und entscheidet daraus seine Eingabe, er verändert nie die Welt direkt.
- **Profile (mindestens zwei):** Pflichtliste Q19: passiv, sparsam, Mauern zuerst, Wirtschaft zuerst, Koop 2/4; BAL1 nimmt zwei davon (Vorschlag: „passiv“ = steht am Hub, zahlt nichts, und „sparsam“ = zahlt erst Mauern, dann Bauern, wenn Gold ≥ Preis), die übrigen kommen in BAL3.
- **Kennzahlen (B-099):** Tick der ersten Gold-Schwelle, Gold und Material zu Dämmerungsbeginn je Tag, Zeit bis zur ersten Mauer und zum ersten Bogen, Verluste und Überleben je Welle, Tick des Burgfalls, Einnahmen/Ausgaben je Tag, erreichte Tiefe. Fehlt eine Quelle im Zustand oder in den Ereignissen, Kennzahl als `null` und Ticket (SIM), nicht im Sim-Code nachrüsten.
- Determinismus: Seeds als feste Liste, `engine/rng` für jede Bot-Entscheidung mit Zufall, keine Wanduhr, keine Map-Iteration ohne Sortierung (Check aus F2/B-138 beachten). Ausgabe-JSON mit fester Feldreihenfolge (Structs, keine Maps).
- Abbruch eines Laufs (Panic/Fehler) → Lauf `ungültig` mit Seed in der Ausgabe, der Rest läuft weiter.

## Erlaubte Dateien

- `engine/balance/` (neu, mit Tests), `cmd/k3c-balance/` (neu)
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

- [ ] AC-01: Test belegt byte-gleiche Kennzahlen bei gleichen Seeds, Parametern und Daten.
- [ ] AC-02: zwei Bot-Profile und die Kennzahlen aus B-099 umgesetzt und getestet (fehlende Quellen als `null` mit Ticket).
- [ ] `task check:go` grün; keine Datei > 400 Zeilen, keine Funktion > 60 Zeilen.

## Prüfen

```bash
task check:go
```

## Ergebnis

–
