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
