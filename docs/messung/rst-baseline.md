# Mess-Rezept RST (Entscheidung 006, B-725)

Jede Messung läuft vor und nach einer Änderung mit **demselben Aufbau**. Ergebnisse stehen als Tabelle in
`docs/decisions/006-rust-engine.md` › Messbefunde. Parallele Agenten werden notiert (Last siehe Schritt 0).
Die Werte schwanken unter Last stark; Vergleiche nur bei ähnlicher Last und mit demselben Rechner.

## Aufbau

- Rechner: Core Ultra 7 165H (22 Threads), Windows 11, Go 1.27.3, Energieschema „Ausbalanciert“.
- Profil-Ausgaben liegen in `.rs1prof/` (nicht eingecheckt).

## Schritte

0. **Last:** `powershell -c "Get-CimInstance Win32_Processor | Select LoadPercentage"`, Wert in die Tabelle schreiben.
1. **Sim (CPU/Heap):**
   `go test ./engine/sim -run XXX -bench IslandStep -benchmem -benchtime 100000x -cpuprofile .rs1prof/sim.cpu -memprofile .rs1prof/sim.mem -o .rs1prof/sim.test`
   (feste Iterationszahl, nie `3s`: die Welt im Benchmark wächst, ns/op hängt sonst von der Zahl der Läufe ab)
   dann `go tool pprof -top -nodecount=15 .rs1prof/sim.test .rs1prof/sim.cpu` und `-sample_index=alloc_space` für das Heap-Profil.
2. **Raum/Netz (CPU, Heap, Mutex):**
   `go test ./engine/net -run XXX -bench . -benchmem -benchtime 3s -cpuprofile .rs1prof/net.cpu -memprofile .rs1prof/net.mem -mutexprofile .rs1prof/net.mutex -o .rs1prof/net.test`;
   `go test ./engine/room -run XXX -bench . -benchmem -benchtime 2s -cpuprofile .rs1prof/room.cpu -mutexprofile .rs1prof/room.mutex`.
   Hinweis: In den `net`-Benchmarks dominiert das Entpacken der Zustände (`stateOf`/`json.Unmarshal`) die CPU; das ist
   Messaufbau, nicht Spielbetrieb. Echte Raum-Arbeit: Tick-Dauer `BenchmarkTickDevices4` (ns/op).
3. **GC:** `GODEBUG=gctrace=1 go test ./engine/net -run XXX -bench TickDevices4 -benchtime 5s 2>&1 | grep '^gc '`;
   STW-Pause je GC = erste + dritte Zahl der Zeile `a+b+c ms clock`, Heap = letzte Zahl von `x->y->z MB`. p50/p99 aus allen Zeilen.
   (Abweichung von der Session-Vorgabe: Der Online-Server hat keine gctrace-Umgebung über die Workbench; dieser Benchmark
   spielt dieselbe Raum-Tick-Arbeit mit 4 Geräten.)
4. **Level:** `go test ./engine/level -run XXX -bench . -benchmem -benchtime 2s`.
5. **Durchsatz:** `go test ./engine/sim -run XXX -bench SimRun100k -benchtime 1x -benchmem` (bildet `sim_run` nach: 100 000 Ticks,
   Wald, Seed `k3c-dev`, ein Monarch) → `ticks/s`. Der Insel-Tick (`IslandStep3Stages4Players`, ns/op) ist der schwerere, realistische Wert.
6. **Meilenstein-Lauf:** `sim_test {action: start, bots: coop2, world: standard, days: 40, wait: true}` (ohne `seedList` 101 Seeds,
   fast 40 min); für Vorher/Nachher schnell: zusätzlich `seedList: k3c-dev` (ein Seed, ~50 s).
7. **Online (Takt):** Spielserver des Checkouts mit `svc_start Spielserver` (vorher `check_run task:build`), dann
   `sim_test {action: start, mode: online, rooms: 1, players: 4, duration: 5m, focus: [perf], wait: true}`; Ergebnis
   „Tick p99 max“ (Tick-Dauer).
8. **Tick-Abstand:** `K3C_TAKT_TUNE=1` schaltet Priorität „über normal“ und EcoQoS aus wie der Server (Windows); ohne = Standard. `K3C_TAKT_SECONDS=20 go test ./engine/net -run XXX -bench TaktAbstand4 -benchtime 1x`
   (echter 30-Hz-Takt, 4 Geräte; Abstand der Zustandslieferungen an Gerät 0: gapP50/gapP99/gapMax in ms). Zweimal laufen lassen.
9. **Leerer Server:** `svc_start Spielserver`, 5 s warten, dann in PowerShell 10 s lang die CPU-Zeit des Kindprozesses
   (`k3c-server.exe` mit Eltern-PID = Supervisor) messen:
   `$a=Get-Process k3c-server|Select Id,CPU; Start-Sleep 10; $b=Get-Process k3c-server|Select Id,CPU` → (Δ CPU / 10) × 100 %.
10. **Datei-Watch:** `go test ./engine/watch -run XXX -bench . -benchmem`.

## Pi 3 (RS1.2, Mensch)

Abschnitt wird mit RS1.2 gefüllt: Server-Build arm64 auf dem Pi, Lasttest `k3c-load` 2 Räume × 3 Spieler 10 min (Tag und
Nacht), Tick-Dauer p99 und Tick-Abstand p99 aus `/api/status`; Sim-Benchmark als arm64-Binary
(`GOOS=linux GOARCH=arm64 go test -c ./engine/sim -o sim.test`, auf dem Pi `./sim.test -test.bench IslandStep -test.benchmem`).
