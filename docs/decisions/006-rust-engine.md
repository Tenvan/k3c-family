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
(Session RST-M1 unten). Die Befunde unten sind deshalb eine kurze Erstmessung, kein vollständiges Profil.

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

### Was die Erstmessung zur Vermutung sagt

- **Windows-Timer (15,6 ms): widerlegt.** Go 1.27 schläft auf etwa 0,5 ms genau.
- **Scheduler/OS unter Last: bestätigt** als Ursache der Takt-Ausreißer. Rust ändert daran nichts; helfen
  Prozess-Priorität, Abschalten von Power-Throttling (EcoQoS) und Kern-Affinität bei der Hybrid-CPU (P-/E-Kerne).
- **Go selbst:** offen. Ob GC oder Allokationen im Tick die Dauer treiben, zeigt erst das Profil (RST-M1).
- **Eigene Grundlast:** der Datei-Watch kostet 15 % Wandzeit je Sekunde und erzeugt Müll für den GC. Das ist ein
  Go-Fix (Ereignis-Watch per `ReadDirectoryChangesW` oder Scan nur im Dev-Modus), keine Sprachfrage.

Folgerung: Für das **Timing** bringt Rust nach heutigem Stand nichts. Für den **Durchsatz** von Simulation und
Balancing kann Rust etwas bringen (keine GC, bessere Datenlayouts, SIMD); wie viel, ist ungeprüft und wird in RST-M1
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

Wichtig für jede Teil-Umstellung: Die Workbench importiert `k3c/engine/sim` an 36 Stellen (`sim_run`, Balancing,
`studioview`), dazu `net`, `room`, `store`. Was nach Rust wandert, braucht für die Workbench eine Brücke oder bleibt
als Go-Kopie doppelt.

## Varianten

Aufwand: 1 Session ≈ ½ Tag Agentenarbeit plus Review durch Ralf. Schätzungen ungeprüft, ±30 %.

| # | Umfang | Aufwand | Nutzen (messbar) | Risiken |
|---|---|---|---|---|
| 1 | Simulationskern (`engine/sim`, `level`, `rng`) in Rust, Go ruft ihn auf | 28–36 Sessions (14–18 Tage) | Sim-Durchsatz für `sim_run`/`sim_test`/Balancing: Ziel ≥ 2× Ticks/s gegenüber optimiertem Go (vermutet erreichbar, ungeprüft); Tick-Dauer p99 im Raum | Brücke je Tick: Zustand für Snapshot/Delta muss über die Grenze, Serialisierung kann den Gewinn auffressen; cgo braucht einen C-Compiler, der lokal fehlt (Ausweg: DLL über `syscall.LoadDLL` ohne cgo oder eigener Prozess); Determinismus Go↔Rust (Float-Reihenfolge, FMA auf ARM/Pi B-042); zwei Toolchains in CI |
| 2 | Server komplett (`engine/*`, `cmd/k3c-server`) | 50–65 Sessions (25–33 Tage) | zusätzlich: keine GC-Pausen im Netzpfad, ein Prozess ohne Brücke | Workbench bleibt Go und braucht `sim`/`room`/`net` weiter → Engine doppelt oder Workbench ohne `sim_run`; Supervisor, Datei-Watch, Spielstände, Protokoll v2 neu; WebSocket-Stack neu |
| 3 | Game komplett (Server + `k3c-load` + `k3c-tui`) | 56–72 Sessions (28–36 Tage) | wie 2; Lasttest und TUI im selben Stack | wie 2; `k3c-load`/`k3c-tui` sprechen nur das Protokoll, ihr Umzug bringt keine Leistung |
| 4 | Alles inkl. Workbench, `assetkit`, Go-CLIs, Starter | 130–170 Sessions (65–85 Tage) | eine Sprache, kein doppelter Code | Wails → Tauri (Frontends bleiben TS, Bindings neu); MCP-Server neu; Workbench ist das Werkzeug für alle anderen Projekte → monatelang zwei halbe Workbenches; PAP und Balancing stehen still |

## Gemeinsamer Code bei einer Teil-Umstellung

| Gemeinsam | Variante 1 | Variante 2/3 | Umgang |
|---|---|---|---|
| Protokoll `docs/protocol.md` (Go `engine/net/protocol.go`, TS `src/online/`) | bleibt Go, nicht doppelt | dritte Implementierung (Rust) neben TS und Go-Tests der Workbench | Protokoll als Schema (JSON Schema) und Typen daraus erzeugen |
| Daten-Schemas `data/*.json` | **doppelt**: Go-Structs (Server, Workbench) und Rust-Structs (Sim) | doppelt (Workbench) | ein JSON-Schema je Datei, Typen für Go/Rust/TS erzeugen; Rust bekommt die Werte als JSON von Go |
| Golden-Tests `testdata/golden/` (22 Dateien) | gemeinsam, sprachneutral | gemeinsam | **das Orakel der Umstellung**: Rust muss dieselben Dateien bitgleich erzeugen |
| `engine/rng` (85 Zeilen, `golden/rng.json`) | doppelt, solange Go-Code würfelt | doppelt | gefahrlos portierbar, Golden prüft Bitgleichheit |
| Spielstände (`store`, `saves/`) | Format folgt dem Sim-Zustand → Rust-serde muss Go-JSON exakt treffen | neu in Rust | laut Ralf keine Rücksicht auf alte Spielstände; trotzdem ein Format-Test je Seite |
| Workbench-Nutzung von `engine/sim` (36 Importe) | Brücke nötig | Brücke oder Go-Kopie | Variante 1: dieselbe Brücke wie der Server |

## Gefahrlos komplett migrierbar

Reine Funktionen mit Golden-Abdeckung und ohne Rückwirkung auf Laufzeit oder Werkzeuge:

1. `engine/rng` – 85 Zeilen, `golden/rng.json` prüft jede Zahl.
2. `engine/level` – Seed → Level, `golden/level-*.json` (6 Biome + geplant) prüfen bitgleich.
3. Generatoren `tools/ruletable`, `tools/buildings` – Ausgabe sind Dateien, vor/nach per Diff vergleichbar
   (bringt aber keine Leistung).

Nicht gefahrlos: `engine/sim` (Zustand fließt in Snapshot, Spielstand, Workbench), alles in `engine/net`/`room`
(Nebenläufigkeit, Neustart-Supervisor), die Workbench.

## Entscheidung (angenommen 2026-11-07)

- **Zielrichtung ist Variante 1:** Simulationskern (`engine/sim`, `level`, `rng`) in Rust, Server, Workbench und CLIs
  bleiben Go. Treiber: Pi 3 (Tick p99 < 10 ms bei wachsenden Welten) und die Dauer von Balancing und `sim_test`.
- **Varianten 2–4 sind zurückgestellt.** Sie verdoppeln die Engine wegen der Workbench und lösen weder Pi noch Timing
  besser als Variante 1.
- **Reihenfolge wie unten:** Messen (RST-M1, auch auf dem Pi), billige Go-Fixes (RST-M2), Spike (RST-S1/S2), dann
  die volle Umstellung des Kerns. Der Spike bleibt als Sicherung, weil die Brücke Go↔Rust je Tick auf dem Pi
  teurer ist als auf dem PC (ungeprüft).
- **Pluspunkt für Rust auf dem Pi:** Rust fusioniert `x*y + z` nicht von selbst zu FMA. Go tut das auf arm64
  (deshalb der CI-Job `go-arm64`, B-071). Ein Rust-Kern rechnet auf PC und Pi gleich, wenn die Golden-Dateien
  auf beiden Plattformen bitgleich bleiben.

## Empfehlung

1. **Jetzt kein Server- oder Komplett-Umbau (Varianten 2–4).** Sie lösen das Timing nicht und verdoppeln die Engine
   wegen der Workbench.
2. **Zuerst Go optimieren, im Projekt RST als Sprint RST-M:** Datei-Watch auf Ereignisse oder nur Dev umstellen,
   Prozess-Priorität und EcoQoS-Abschaltung für den Server, dann die verschobene Messung (pprof, gctrace,
   mutex-Profil, `sim_test` perf online, `sim_run`-Durchsatz) als Baseline, dazu Tick p99 auf dem Pi 3.
3. **Danach Variante 1 als Spike:** `engine/rng` + `engine/level` in Rust (gefahrlos, Golden als Orakel),
   angebunden als Bibliothek ohne cgo (Windows-DLL, auf dem Pi `.so`); dazu ein heißer Sim-Pfad aus dem Profil.
   Abbruchkriterium: weniger als 2× Durchsatz gegenüber optimiertem Go auf PC **und** Pi, oder Brückenkosten > 20 %
   der Tick-Dauer → Rust verwerfen, Go behalten.
4. Bei bestandenem Spike Variante 1 voll (Sim in Rust, Server bleibt Go). Variante 4 nur, wenn Go danach
   nirgends mehr gebraucht wird – heute nicht absehbar.

Vorgeschlagene Sessions (noch nicht angelegt, Spec-Freigabe durch Ralf):

| Session | Inhalt | Domäne | Aufwand |
|---|---|---|---|
| RST-M1 | Baseline messen: pprof CPU/Heap Sim und Raum, gctrace, mutex, `sim_test` perf, `sim_run`-Durchsatz | SRV | 1–2 |
| RST-M2 | Go-Fixes: Datei-Watch, Prozess-Priorität/EcoQoS, Allokationen im Tick aus dem Profil | SRV | 2–3 |
| RST-S1 | Rust-Toolchain in Taskfile und CI, `rng` + `level` in Rust gegen Golden, DLL-Brücke ohne cgo | INF/SRV | 3–4 |
| RST-S2 | Spike heißer Sim-Pfad in Rust, Messung gegen RST-M2, Entscheidung Variante 1 ja/nein | SIM | 2–3 |

## Folgen

- 001 bleibt gültig: Die Engine läuft im Go-Server, der Kern darin wird Rust.
- Projekt RST wird aktiv; Sprints und Sessions werden über `plan_*` angelegt, die Spec B-725 gibt Ralf frei.
- Scheitert der Spike: 006 wird mit Zahlen auf „Go optimieren“ geändert, Projekt RST endet nach RST-M2.
