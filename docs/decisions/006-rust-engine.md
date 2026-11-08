# 006 · Rust-Engine: Simulationskern zuerst, Server bleibt vorerst Go

- **Status:** angenommen (Ralf, 2026-11-07, vorläufig: „erstmal“)
- **Datum:** 2026-11-07
- **Projekt:** RST · Ticket: B-725
- **Vorgänger:** 001 (Engine auf Go-Server)

## Anlass

Ralf will die Engine nach Rust bringen. Gründe: Timing-Probleme schon auf einem schnellen PC ohne Spiellast,
Simulationen und Balancing dauern zu lange, die Welten werden größer.

**Hauptgrund laut Ralf (2026-11-07): der Raspberry Pi.** Ziel aus B-042: Pi 3 (64-Bit, 4× Cortex-A53, 1 GB),
2 Räume × 3 Spieler, Tick-Dauer p99 < 10 ms. Die Handmessung vom 2026-10-03 lag schon bei 8,0–10,3 ms, also am Ziel,
und die Welten wachsen weiter. Gleich wichtig sind Balancing- und Simulationsläufe; deshalb kommt Variante 1 auf jeden
Fall in Frage.

Ralf hat am 2026-11-07 entschieden: Die Timing-Messung läuft **nicht jetzt**, sondern im Lauf der Umsetzung
(Session RS1.1). Die Befunde unten sind deshalb eine kurze Erstmessung, kein vollständiges Profil.

## Messbefunde (Erstmessung 2026-11-07, Go 1.27.3, Core Ultra 7 165H, Energieschema „Ausbalanciert“)

Rechner während der Messung: **100 % CPU-Last** durch parallele Agenten-Sessions. Alle Werte gelten unter dieser Last.

| Messung | Ergebnis | Bewertung |
|---|---|---|
| `time.Sleep(1 ms)` | p50 1,55 ms, p99 8,3 ms | Go nutzt unter Windows hochauflösende Timer. Die 15,6-ms-Auflösung ist **widerlegt**. |
| `time.Sleep(16 ms)` | p50 16,5 ms, p99 27,9 ms | Ausreißer kommen vom OS-Scheduler unter Last, nicht vom Timer. |
| Ticker 30 Hz, leer | p50 33,3 ms, p99 40,7 ms, max 45 ms | Takt stimmt, Ausreißer bis +12 ms |
| Ticker 30 Hz, 8 ms Arbeit + Allokation je Tick | p99 71 ms, max 156 ms, 39 von 300 Abständen > +5 ms | Unter Volllast fallen ganze Ticks aus. Ein Rust-Thread bekommt dieselbe CPU-Zeit vom OS. |
| Leerer Spielserver (0 Räume) | 8,9 % eines Kerns, 20 Threads, 22 MB | Grundlast ohne Spiel |
| Datei-Watch-Scan (`engine/watch`, 1936 Dateien in `public/` + `data/`) | **150 ms je Durchlauf, jede Sekunde** | erklärt die Grundlast fast ganz; allokiert je Scan eine neue Map mit ~2000 Einträgen |

Nicht gemessen (auf Ralfs Wunsch verschoben): pprof CPU/Heap von `engine/sim`, Tick-Dauern im laufenden Raum,
GC-Pausen (`GODEBUG=gctrace=1`), Lock-Contention (`mutex`-Profil), Kosten von Snapshot/Delta, `sim_test` online.

### Baseline PC (RS1.1, 2026-11-07, Stand develop 9d8b2353, Rezept `docs/messung/rst-baseline.md`)

Rechner: Core Ultra 7 165H, Go 1.27.3. **Last:** 68 % CPU (`LoadPercentage`) durch parallele Agenten während der Messung;
Wandzeiten (Takt, GC) sind damit nur als obere Schranke zu lesen, `ns/op` der Benchmarks schwankt etwa ±20 %.

| Messung | Ergebnis |
|---|---|
| Insel-Tick 3 Stufen × 4 Spieler (`IslandStep3Stages4Players`) | 40,2 µs/Tick (≈ 24 900 Ticks/s), 3 666 B/op, 39 allocs/op |
| Insel-Tick Endboss, 4 Spieler | 22,9 µs/Tick, 0 allocs |
| `sim_run` 100 000 Ticks (`SimRun100k`, Wald, 1 Monarch) | 0,125 s = 802 000 Ticks/s, 15,4 MB, 373 620 allocs |
| Sim-CPU-Profil (Insel-Tick) | `collectCoins` 51,7 % flat / 77,4 % kumulativ, `World.values` 12,0 %, `purseMax` 6,2 %, `math.Abs` 3,3 % |
| Sim-Heap (alloc_space) | `stepSpawns` 40,4 %, `chooseTarget` 27,5 %, `bytes.Clone` 19,2 % |
| Raum-Tick mit 4 Geräten (`TickDevices4`) | 1,81 ms/Tick, 232 920 B, 5 571 allocs |
| Raum-Tick mit/ohne Monitor (`TickMonitor`) | 1,31 ms / 1,10 ms |
| Stufen-Raum 2 Spieler (`StufenZweiSpieler`) | 419 ms/op, 82,7 MB, 72 918 allocs (Mehr-Tick-Lauf) |
| Net/Raum-CPU | `json.Unmarshal` im Messaufbau 39 %, `json.Marshal` des Zustands 8,7 % (Spielbetrieb); Rest Runtime (`preemptM`, `semasleep`) |
| Net/Raum-Heap | `reflect.mapassign` (Testclient-Entpacken) 45,7 %, `bytes.Clone` 11,1 %, `addActions`/`actionsOf` 8,7 % |
| Mutex-Profil (`net`, `room`) | nur Runtime (`runtime.unlock`, GC-Worker `enlistWorker`); keine Wartezeit an Raum-Sperren im Profil |
| GC (`gctrace`, `TickDevices4`, 5 s) | 894 GCs; STW je GC p50 ≈ 0 ms, p99 9,6 ms, max 21 ms (unter Last); Live-Heap max 102 MB |
| Level-Generator (`GenerateWith`) | 28–65 µs/Aufruf (forest 63 µs, cave 34, mine 28, ironhold 65, crystal 55, geplant 40), 93–137 allocs |
| Meilenstein-Lauf `sim_test` coop2/standard/40 Tage, 101 Seeds | 38 min 41 s (Pass); ein Seed (`k3c-dev`): 50 s |
| Online `sim_test` perf (1 Raum × 4 Bots, 5 min) | Tick-Dauer p99 max 135,7 ms über 61 Proben (Ziel ≤ 10 ms; Last 68 %), 35 310 Zustände, 0 Fehler |
| Tick-Abstand p99 (`TaktAbstand4`, 4 Geräte, 20 s) | p50 33,3 ms, **p99 65,7–84,3 ms**, max 154–263 ms (Last 68 %) |
| Leerer Spielserver (10 s) | **3,3 % eines Kerns**, 13 Threads (Kindprozess; Supervisor 0 %) |

Lesart: 77 % der Sim-CPU im Insel-Tick liegen in einer einzigen Funktion (`collectCoins`: Münzen × Spieler mit
`w.values()` in der inneren Schleife). Das ist zuerst ein Go-Fix und zugleich der naheliegende Spike-Kandidat für RS1.6.
Der Netz-Raum-Tick (1,8 ms) liegt weit unter 10 ms; die Ausreißer im Online-Lauf (p99 136 ms) und im Takt-Abstand
(p99 66–84 ms) stammen von Scheduler und GC unter 68 % Last, nicht von der Rechenarbeit (wie in der Erstmessung).

### Go-Fixes (RS1.3, 2026-11-07): vorher/nachher

Gleicher Aufbau wie die Baseline (Rezept `docs/messung/rst-baseline.md`). **Last:** vorher 68 %, nachher 3–30 % (Agenten
ruhig); Zeilen mit Wandzeit sind deshalb nur grob vergleichbar, die Benchmark-Zeilen (feste Iterationszahl) gut.

| Messung | vorher | nachher |
|---|---|---|
| Datei-Watch im Leerlauf | Scan alle 1 s, 150 ms, neue Map mit ~2 000 Einträgen je Scan | Scan nur nach Datei-Ereignis, bei Frist oder alle 30 s; `BenchmarkRunLeerlaufMitWake`: **0 allocs/op**, ~1 µs je Takt |
| Leerer Spielserver (10 s) | 3,3 % eines Kerns (Erstmessung 8,9 %), 13 Threads | **0,00 %**, 14 Threads, Priorität „über normal“ |
| Insel-Tick 3 Stufen × 4 Spieler, 100 000 Iterationen | 31,5–42,3 µs, 3 549 B, 37 allocs | **20,4–21,5 µs**, 650 B, 10 allocs |
| Insel-Tick Endboss | 22,9 µs | 12,1 µs (andere Iterationszahl, nur Richtung) |
| Tick-Abstand p99, 4 Geräte, 20 s (`TaktAbstand4`) | 74,6–76,9 ms, max 108–223 ms (Last 25 %) | mit Priorität/EcoQoS-Aus (`K3C_TAKT_TUNE=1`): **35,8–36,0 ms**, max 42–45 ms |
| Online `sim_test` (1 × 4, 5 min) Tick-Dauer p99 max | 135,7 ms (Last 68 %) | 2,5 ms (Last ~30 %) |
| Meilenstein-Lauf, ein Seed `k3c-dev` | 50 s | 57 s (Lauf und Bericht unverändert, 8 von 16 Zielen wie vorher) |

Maßnahmen: (1) Datei-Watch ereignisgesteuert (`engine/watch/notify.go`, fsnotify, Rückfall auf Abfragen im Takt) und
Stände ohne Neuanlage der Map; (2) Server-Kind setzt unter Windows Priorität „über normal“ und schaltet Power-Throttling
(EcoQoS) ab (`cmd/k3c-server/priority_windows.go`); (3) drei Sim-Fixes ohne Verhaltensänderung: `collectCoins` holt
Reichweite und Beutelgrenze einmal je Aufruf, `stepSpawns` baut Listen nur bei fälligen Spawns, `chooseTarget` gibt
Zeiger in die Kandidatenliste zurück statt `&c` der Schleifenvariable (Heap-Objekt je Durchlauf). Der dritte große
Heap-Posten der Baseline (`bytes.Clone` aus `json.Marshal`) kommt vom Benchmark-Aufbau, nicht aus dem Tick.
Golden-Dateien unverändert, `engine/sim`-Tests grün.

Folgerung: Die Takt-Ausreißer waren überwiegend Prozess-Scheduling, nicht Go: Priorität/EcoQoS brachte den Tick-Abstand
p99 von ~75 ms auf ~36 ms (Ziel ≤ 40 ms), ohne eine Zeile Rechenlogik. `collectCoins` bleibt der heißeste Sim-Pfad.

### Zustand ohne JSON-Roundtrip (RS2.3, B-731 Stufe 1, 2026-11-08)

`net.stateOf` baut die Map direkt aus `sim.World` und `sim.EconomyOf` (`engine/net/state.go`), der alte Weg bleibt als
Test-Orakel `stateOfJSON`. Vorher = `HEAD` (2b55090e) als Kopie, nachher = Arbeitsstand, beide nacheinander auf demselben
PC (Core Ultra 7 165H), `go test ./engine/net -run XXX -bench 'TickDevices4$' -benchmem -benchtime 100000x`, je zwei Läufe.
**Last:** 91 % vor dem ersten Paar (parallele Agenten), 2–65 % danach.

| Messung | vorher | nachher | Faktor |
|---|---|---|---|
| `BenchmarkTickDevices4` ns/op | 893 794 / 914 131 | **225 281 / 215 459** | 4,0–4,2× |
| allocs/op | 5 647 / 5 648 | **1 229** | 21,8 % |
| B/op | 239 621 / 240 025 | 151 325 / 151 341 | 63 % |
| Online `sim_test` (1 × 4, 5 min, perf) Tick-Dauer p99 max | 2,5 ms (RS1.3) | 2,5 ms (run-28) | gleich |

B-731/AC-01 erfüllt (≥ 3×, allocs ≤ 25 %), Stufe 2 nicht nötig. Snapshot und Delta sind in jedem Takt byte-gleich zum
Orakel (alle Biome × 660 Takte, Bosswelle, Golden-Läufe, Vorschau); der Reflection-Test fängt neue `sim.World`-Felder.
Kniffe gegen Allokationen: ganze Zahlen 1–4095 und Texte aus festen Mengen liegen vorab als `any` bereit.

### Brücke Go → Rust ohne cgo (RS1.4, 2026-11-07)

`rust/k3c-core` (cdylib, C-ABI) wird von `engine/core` geladen: Windows über `syscall.LoadDLL` + `SyscallN`, Linux über
purego (`dlopen`, nur mit `-tags k3ccore`). Aufrufkosten auf dem PC (Core Ultra 7 165H, Windows, `go test ./engine/core -bench .`):

| Aufruf | Kosten | allocs |
|---|---|---|
| `k3c_add(a, b)` (zwei Zahlen, eine Zahl zurück) | **41 ns** | 0 |
| `k3c_sum_bytes` über 4 KiB (Zeiger + Länge, Summe im Rust-Teil) | 881 ns (davon fast alles Rechnen, ~0,2 ns/Byte) | 0 |

Folgerung: Der reine Aufruf kostet etwa 40 ns, bei 30 Hz und einigen hundert Aufrufen je Tick unter 0,1 % des 33-ms-Takts.
Teuer wird nur, was über die Grenze kopiert oder serialisiert wird (Zustand für Snapshot/Delta); das misst RS1.6. Pi (arm64):
ungemessen (RS1.2, Ralf). Linux-arm64 prüft lokal nur `cargo check`; Linken und Laden läuft in der CI (Job `rust`, nur Push).
**Docker:** Das Image bleibt `distroless/static` und lädt den Kern nicht; mit `-tags k3ccore` hängt das Go-Binary dynamisch
an glibc, das Image bräuchte dann `distroless/cc` und die `.so` (Umstellung erst bei bestandenem Spike, RS1.6).

### rng und Level-Generator in Rust (RS1.5, 2026-11-07, Last ~2 %)

`rust/k3c-core/src/rng.rs` und `src/level/` (9 Module) erzeugen `rng.json` und alle sechs `level-*.json` (forest, cave, mine,
ironhold, crystal, planned) **bitgleich** (Go-Vergleichstests `TestRustBitgleich`, `TestRustWieGo`: alle Biome × 9–12 Fälle × 40 Seeds,
dazu 31 Fehlerfälle mit identischem Text). Aufruf aus Go über die Brücke (`engine/core/level.go`, JSON rein und raus; Rust liest
`data/` nicht selbst). arm64: Vergleichstests laufen im CI-Job `rust` (ungeprüft bis zum Merge).
`BenchmarkGenerate` (Go) gegen `BenchmarkGenerateRust` (inklusive Brücke), `-benchtime 20000x -count 3`, Minimum in ns/op:

| Biom | Go | Rust + Brücke | Verhältnis |
|---|---|---|---|
| forest | 54 852 | 44 064 | 1,24× |
| cave | 39 889 | 31 526 | 1,27× |
| crystal | 32 644 | 34 731 | 0,94× |
| ironhold | 40 274 | 43 298 | 0,93× |
| mine | 17 606 | 34 554 | 0,51× |

Rust braucht 1 Allokation je Aufruf (Go 93–117). Der Kern selbst ist 2–4× schneller (Einzelmessung Rust: Erzeugen 8–25 µs),
aber Biom-JSON einlesen (6–13 µs) und Layout-JSON schreiben (4–17 µs) fressen den Gewinn: Mit Brücke liegt Rust bei 0,5–1,3× Go.
Folgerung: Ein Generator, der JSON-in/JSON-out über die Grenze spricht, ist kein Gewinn; ein Gewinn braucht ein Handle mit einmal
gelesenem Biom und binäre Ausgabe. Level-Erzeugung ist selten (je Insel einmal) und kein Engpass.

### Spike heißer Sim-Pfad `collectCoins` (RS1.6, 2026-11-07, nur PC; Pi ausstehend)

Pfad: `collectCoins` (Münzen × Spieler), in der Baseline 77 % der Sim-CPU. Rust: `rust/k3c-core/src/coins.rs`, Aufruf über die
Brücke mit flachen Feldern (Go liefert x, Sperre, Gold, Lebend-Flag; Rust liefert den Nehmer je Münze, Gold wird erhöht).
Vergleichstest `TestRustCollectCoinsBitgleich` (Nehmer-Reihenfolge und Gold, 27 / 327 / 3 027 Münzen): **bitgleich**.
`go test ./engine/sim -run XXX -bench CollectCoins -benchtime 20000x` (Go = optimiertes Go aus RS1.3):

| Münzen | Go | Rust, Eingabe flach (Aufruf + Rechnung) | Rust, je Aufruf aus Go-Strukturen gepackt |
|---|---|---|---|
| 27 (realer Spielstand nach 6 000 Ticks) | 257 ns | 227 ns (1,1×) | 2 089 ns (0,12×) |
| 327 | 3 810 ns | 1 207 ns (3,2×) | 16 842 ns (0,23×) |
| 3 027 | 17 939 ns | 3 899 ns (4,6×) | 78 476 ns (0,23×) |

Befunde:

1. **Der Pfad ist im Spielbetrieb gar nicht heiß.** Im Benchmark wachsen die Münzen unbegrenzt (nach 100 000 Ticks 538 / 2 078 /
   3 495 je Stufe, weil Spieler auf anderen Stufen sind und nichts einsammeln); echte Stände haben Dutzende. Mit 27 Münzen kostet
   `collectCoins` 0,26 µs je Tick, ein Insel-Tick insgesamt 12–21 µs.
2. **Die Sim ist nicht der Takt-Kostenträger.** CPU-Profil von `Room.Tick` (4 Geräte, `TickDevices4`): 97,7 % in
   `pushStatesSaved` → `stateOf` (Zustand als JSON serialisieren und wieder in `map[string]any` einlesen, zweimal je Gerät-Zustand),
   **1,9 % in `sim.StepIsland`**. Ein Tick dauert 1,1 ms, davon sind ~20 µs Simulation. Rust für die Sim kann also höchstens
   ~2 % Tick-Zeit sparen; die Pi-Tick-Zeit liegt in `engine/net/protocol.go` (`stateOf`, Delta), nicht in `engine/sim`.
3. **Brückenkosten:** Der reine Aufruf kostet ~40 ns (RS1.4). Packen aus Go-Strukturen (`[]*Coin`, `[]*Player`) frisst den ganzen
   Gewinn (4–8× langsamer als Go). Teilumstellung einzelner Pfade mit Zustand in Go lohnt nie; ein Gewinn entsteht nur, wenn der
   Zustand dauerhaft in Rust liegt (voller Umbau der Sim, evtl. mit Serialisierung in Rust, Variante 1 voll / Etappe 2).

Abbruchkriterium aus 006 (weniger als 2× gegen optimiertes Go **oder** Brückenkosten > 20 % der Tick-Dauer): Der Rechenkern
erreicht 3–5×, aber nur bei unrealistisch vielen Münzen (bei realen 27: 1,1×), und mit der Go↔Rust-Übergabe des Zustands
liegen die Brückenkosten weit über 20 %. **Empfehlung (Entwurf, Ralf entscheidet): kein voller Umbau der Sim nach Rust jetzt;
Projekt RST pausiert nach RS1 mit dem Ergebnis „Go optimieren“.** Begründung in Zahlen: (a) die Takt-Ausreißer waren
Prozess-Scheduling (Tick-Abstand p99 75 → 36 ms durch Priorität/EcoQoS), (b) die Tick-Arbeit ist zu 98 % Zustands-Serialisierung
(Go-Fix möglich, Ticket), (c) die Sim ist 2 % des Ticks. **Offen bis zur Pi-Messung (RS1.2, Ralf):** liegt der Pi-Tick weiter bei
8–10 ms, entscheidet sich, ob die Serialisierung in Go (kein Roundtrip) reicht oder Etappe 2 (Server in Rust) gebraucht wird.
Nicht belegt: Pi-Werte für alle Zeilen oben, Rust-Gewinn für die Serialisierung.

### Was die Erstmessung zur Vermutung sagt

- **Windows-Timer (15,6 ms): widerlegt.** Go 1.27 schläft auf etwa 0,5 ms genau.
- **Scheduler/OS unter Last: bestätigt** als Ursache der Takt-Ausreißer. Rust ändert daran nichts; helfen
  Prozess-Priorität, Abschalten von Power-Throttling (EcoQoS) und Kern-Affinität bei der Hybrid-CPU (P-/E-Kerne).
- **Go selbst:** offen. Ob GC oder Allokationen im Tick die Dauer treiben, zeigt erst das Profil (RS1.1).
- **Eigene Grundlast:** der Datei-Watch kostet 15 % Wandzeit je Sekunde und erzeugt Müll für den GC. Das ist ein
  Go-Fix (Ereignis-Watch per `ReadDirectoryChangesW` oder Scan nur im Dev-Modus), keine Sprachfrage.

Folgerung: Für das **Timing** bringt Rust nach heutigem Stand nichts. Für den **Durchsatz** von Simulation und
Balancing kann Rust etwas bringen (keine GC, bessere Datenlayouts, SIMD); wie viel, ist ungeprüft und wird in RS1.1
gegen ein optimiertes Go gemessen.

## Codeumfang (Go, Stand origin/develop c81160d4)

| Bereich | Code | Tests |
|---|---:|---:|
| `engine/sim` | 10 374 | 15 053 |
| `engine/level` + `engine/rng` | 1 823 | 1 842 |
| `engine/room`, `net`, `store`, `watch`, `conlog`, `devrun`, `internal` | 7 443 | 11 104 |
| `cmd/k3c-server` | 1 050 | 687 |
| `cmd/k3c-load` + `cmd/k3c-tui` | 1 716 | 958 |
| `tools/k3c-workbench` | 32 093 | 17 838 |
| `tools/assetkit` | 4 501 | 2 133 |
| übrige Go-CLIs (`buildings`, `ruletable`, `cleanup`, `atlas`, `k3c-starter`) | 2 290 | 1 787 |

Die Workbench importiert `k3c/engine/sim` an 36 Stellen (`sim_run`, Balancing, `studioview`), dazu `net`, `room`,
`store`. **Ralf, 2026-11-07:** Diese Importe dürfen wegfallen; Sim und Balancing ruft die Workbench dann als Task bzw.
Kindprozess auf (JSON rein, Bericht raus). Die Workbench ist damit kein Hindernis mehr für eine Teil-Umstellung
(Ticket B-726). Die Generatoren (`tools/buildings`, `ruletable`, `atlas`) entfallen mit PAP ohnehin und zählen nicht.

## Varianten

Aufwand: 1 Session ≈ ½ Tag Agentenarbeit plus Review durch Ralf. Schätzungen ungeprüft, ±30 %.

| # | Umfang | Aufwand | Nutzen (messbar) | Risiken |
|---|---|---|---|---|
| 1 | Simulationskern (`engine/sim`, `level`, `rng`) in Rust, Go ruft ihn auf | 28–36 Sessions (14–18 Tage) | Sim-Durchsatz für `sim_run`/`sim_test`/Balancing: Ziel ≥ 2× Ticks/s gegenüber optimiertem Go (vermutet erreichbar, ungeprüft); Tick-Dauer p99 im Raum | Brücke je Tick: Zustand für Snapshot/Delta muss über die Grenze, Serialisierung kann den Gewinn auffressen; cgo braucht einen C-Compiler, der lokal fehlt (Ausweg: DLL über `syscall.LoadDLL` ohne cgo oder eigener Prozess); Determinismus Go↔Rust (Float-Reihenfolge, FMA auf ARM/Pi B-042); zwei Toolchains in CI |
| 2 | Server komplett (`engine/*`, `cmd/k3c-server`) | 50–65 Sessions (25–33 Tage), dazu 3–5 für die Workbench-Entkopplung (B-726) | zusätzlich: **keine Brücke je Tick**, keine GC-Pausen im Netzpfad, ein Prozess; auf dem Pi der volle Gewinn | Supervisor, Datei-Watch, Spielstände, Protokoll v2 und WebSocket-Stack neu; lange Parallelphase, in der Go-Server und Rust-Server beide gepflegt werden |
| 3 | Game komplett (Server + `k3c-load` + `k3c-tui`) | 56–72 Sessions (28–36 Tage) | wie 2; Lasttest und TUI im selben Stack | wie 2; `k3c-load`/`k3c-tui` sprechen nur das Protokoll, ihr Umzug bringt keine Leistung |
| 4 | Alles inkl. Workbench, `assetkit`, `cleanup`, Starter (Generatoren entfallen mit PAP) | 125–165 Sessions (63–83 Tage) | eine Sprache | Wails → Tauri (Frontends bleiben TS, Bindings neu); MCP-Server neu; Workbench ist das Werkzeug für alle anderen Projekte → monatelang zwei halbe Workbenches; bringt dem Spiel keine Leistung |

## Gemeinsamer Code bei einer Teil-Umstellung

| Gemeinsam | Variante 1 | Variante 2/3 | Umgang |
|---|---|---|---|
| Protokoll `docs/protocol.md` (Go `engine/net/protocol.go`, TS `src/online/`) | bleibt Go, nicht doppelt | Rust statt Go; Go nur noch, wo die Workbench Protokoll spricht (Bot-Geräte) | Protokoll als Schema (JSON Schema) und Typen daraus erzeugen |
| Daten-Schemas `data/*.json` | **doppelt**: Go-Structs (Server) und Rust-Structs (Sim) | nach B-726 nur Rust; die Workbench liest `data/` nur noch für Anzeige und `vary` | ein JSON-Schema je Datei, Typen für Rust/TS (und Go, solange nötig) erzeugen |
| Golden-Tests `testdata/golden/` (22 Dateien) | gemeinsam, sprachneutral | gemeinsam | **das Orakel der Umstellung**: Rust muss dieselben Dateien bitgleich erzeugen |
| `engine/rng` (85 Zeilen, `golden/rng.json`) | doppelt, solange Go-Code würfelt | doppelt | gefahrlos portierbar, Golden prüft Bitgleichheit |
| Spielstände (`store`, `saves/`) | Format folgt dem Sim-Zustand → Rust-serde muss Go-JSON exakt treffen | neu in Rust | laut Ralf keine Rücksicht auf alte Spielstände; trotzdem ein Format-Test je Seite |
| Workbench-Nutzung von `engine/sim` (36 Importe) | entfällt mit B-726 (Task/Kindprozess) | entfällt mit B-726 | **zuerst** entkoppeln, dann ist die Workbench sprachneutral |

## Gefahrlos komplett migrierbar

Reine Funktionen mit Golden-Abdeckung und ohne Rückwirkung auf Laufzeit oder Werkzeuge:

1. `engine/rng` – 85 Zeilen, `golden/rng.json` prüft jede Zahl.
2. `engine/level` – Seed → Level, `golden/level-*.json` (6 Biome + geplant) prüfen bitgleich.
3. `cmd/k3c-load` – spricht nur das Protokoll, Black-Box-Vergleich gegen den Server möglich (bringt keine Leistung).

(Die Generatoren `tools/ruletable`, `buildings`, `atlas` entfallen mit PAP und werden nicht migriert.)

Nicht gefahrlos: `engine/sim` (Zustand fließt in Snapshot und Spielstand), alles in `engine/net`/`room`
(Nebenläufigkeit, Neustart-Supervisor), die Workbench.

## Entscheidung (angenommen 2026-11-07)

- **Erster Schritt ist Variante 1:** Simulationskern (`engine/sim`, `level`, `rng`) in Rust, Server bleibt vorerst Go.
  Treiber: Pi 3 (Tick p99 < 10 ms bei wachsenden Welten) und die Dauer von Balancing und `sim_test`.
- **Variante 2 ist das mögliche Endziel** (Nachtrag 2026-11-07, nach Ralfs Hinweis zur Workbench): Mit entkoppelter
  Workbench (B-726) verdoppelt Variante 2 nichts mehr und spart die Brücke je Tick, die auf dem Pi das größte Risiko
  von Variante 1 ist. Variante 1 ist dann Etappe 1 von Variante 2, keine Sackgasse: Der Rust-Kern bleibt, später
  wandern `room`/`net`/`store` dazu und die Brücke fällt weg. Ob Etappe 2 kommt, entscheiden die Zahlen aus RS1.6.
- **Varianten 3 und 4 sind zurückgestellt:** Lasttest, TUI und Workbench in Rust bringen dem Spiel keine Leistung.
- **Reihenfolge:** Messen (RS1.1/RS1.2, auch auf dem Pi), billige Go-Fixes (RS1.3), Spike (RS1.4–RS1.6), dann
  die volle Umstellung des Kerns. Der Spike bleibt als Sicherung, weil die Brücke Go↔Rust je Tick auf dem Pi
  teurer ist als auf dem PC (ungeprüft).
- **Pluspunkt für Rust auf dem Pi:** Rust fusioniert `x*y + z` nicht von selbst zu FMA. Go tut das auf arm64
  (deshalb der CI-Job `go-arm64`, B-071). Ein Rust-Kern rechnet auf PC und Pi gleich, wenn die Golden-Dateien
  auf beiden Plattformen bitgleich bleiben.

## Empfehlung

1. **Kein Komplett-Umbau (Varianten 3–4).** Variante 2 erst nach dem Spike, als Etappe 2.
2. **Workbench entkoppeln (B-726):** `sim_run`, `sim_test` offline und Balancing rufen die Sim als Kindprozess/Task
   auf statt per Import. Unabhängig von RS1, kann parallel laufen; Voraussetzung für Etappe 2.
3. **Zuerst Go optimieren, im Projekt RST als Sprint RS1:** Datei-Watch auf Ereignisse oder nur Dev umstellen,
   Prozess-Priorität und EcoQoS-Abschaltung für den Server, dann die verschobene Messung (pprof, gctrace,
   mutex-Profil, `sim_test` perf online, `sim_run`-Durchsatz) als Baseline, dazu Tick p99 auf dem Pi 3.
4. **Danach Variante 1 als Spike:** `engine/rng` + `engine/level` in Rust (gefahrlos, Golden als Orakel),
   angebunden als Bibliothek ohne cgo (Windows-DLL, auf dem Pi `.so`); dazu ein heißer Sim-Pfad aus dem Profil.
   Abbruchkriterium: weniger als 2× Durchsatz gegenüber optimiertem Go auf PC **und** Pi, oder Brückenkosten > 20 %
   der Tick-Dauer → Rust verwerfen, Go behalten.
5. Bei bestandenem Spike Variante 1 voll (Sim in Rust). Zeigt RS1.6, dass die Brücke auf dem Pi > 20 % der
   Tick-Dauer kostet, direkt mit Etappe 2 (Variante 2) weiter statt die Brücke zu optimieren.

Umsetzung: Sprint RS1 (Sessions RS1.1–RS1.7, `docs/sprints/geplant/RS1-messen-go-fixes-rust-spike/`), dazu B-726.

## Folgen

- 001 bleibt gültig: Die Engine läuft im Server, nicht im Client. Etappe 1: Go-Server mit Rust-Kern; Etappe 2
  (falls sie kommt): Rust-Server, dann wird 001 im Wortlaut „Go“ angepasst.
- Projekt RST wird aktiv; Sprints und Sessions werden über `plan_*` angelegt, die Spec B-725 gibt Ralf frei.
- Scheitert der Spike: 006 wird mit Zahlen auf „Go optimieren“ geändert, Projekt RST endet nach RS1.
