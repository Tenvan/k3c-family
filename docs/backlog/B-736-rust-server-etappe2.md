# B-736 · Etappe 2: Server und Simulation laufen komplett in Rust, Go-Server und Brücke entfallen

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RS2
- **Projekt:** RST
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-07, Chat (Ralf), Revision 1, durch 🧑 (mit RS2)

## Ausgangslage

Entscheidung 006 (Variante 2): Der Server läuft komplett in Rust. Ralf hat diese Stufe am 2026-11-07 ausdrücklich gewählt, obwohl RS1 (PR #510) gezeigt hat, dass die Zeit nicht in der Sim liegt.

**Messbefunde, die Ralf kennt (006 › Spike RS1.6):** Die Sim ist ~2 % der Raum-Tick-Zeit (~20 µs von 1,1 ms), 97,7 % liegen im JSON-Roundtrip `stateOf` (`engine/net/protocol.go`). Der Spike `collectCoins` blieb bei realen 27 Münzen bei 1,1× (PC). Teilumstellung mit Zustand in Go ist netto negativ (Packkosten 4–8× langsamer). Die Takt-Ausreißer waren Prozess-Scheduling (Tick-Abstand p99 75 → 36 ms durch Priorität/EcoQoS). Die RS1-Empfehlung lautete: kein Vollumbau, zuerst B-731. Pi-Werte fehlen (RS1.2, Ralf).

**Wichtig zum Umfang:** Etappe 1 (Sim in Rust, Go-Server) wurde nie umgesetzt, nur gespikt. Etappe 2 heißt deshalb praktisch **Sim (~25 000 Zeilen Go) + room (~7 600) + net (~7 400) + store (~1 400) + watch/devrun/Server-`cmd`** nach Rust: 006 schätzt 50–65 Sessions (25–33 Tage) plus B-726. Bestand Rust: `rust/k3c-core` (rng, level, coins als C-ABI), Brücke `engine/core`.

Vorarbeiten, die auch ohne Etappe 2 Gewinn bringen: **B-726** (Workbench entkoppeln, freigegeben) und **B-731** (Zustand ohne JSON-Roundtrip, Entwurf).

## Ziel

Ein Rust-Server (`rust/k3c-server`) ersetzt `cmd/k3c-server` und `engine/{sim,room,net,store,watch,devrun}`: gleiches Protokoll (`docs/protocol.md`), gleiche Golden-Dateien, gleiche Spielstände-Struktur; die Go↔Rust-Brücke (`engine/core`) und der Go-Server entfallen. Entscheidung 001 wird im Wortlaut von „Go“ auf „Rust“ angepasst. Auf dem Pi 3 hält der Server Tick p99 < 10 ms bei 2 Räumen × 3 Spielern (B-042), `sim_run`/`sim_test` laufen schneller als optimiertes Go.

Umsetzung in Sprints (Skizze, nur RS2 ist detailliert; die Folgesprints werden je einzeln bereit gemacht und von Ralf freigegeben):

| Sprint | Inhalt | Sessions (grob) |
|---|---|---|
| RS2 | Vorarbeit: B-726, B-731 Stufe 1, Inventar + Entscheidung 007 (Entwurf), Pi-Messung | 6 |
| RS3 | Orakel und Skelett: Protokoll-Trace als Golden (Go-Server aufzeichnen), JSON-Schema aus Protokoll, Rust-Server-Skelett (HTTP, `/ws`, Handshake, Hello gegen den TS-Client) als **Spike mit Abbruchkriterium** | 5–6 |
| RS4–RS7 | Sim-Port in Etappen (Welt/Daten/Chunks → Monarch/Bewegung/Münzen → Gegner/Kampf/Truppen → Wirtschaft/Bau/Bürger → Stufen/Insel), je Etappe Golden bitgleich | 20–26 |
| RS8 | `room`: Raum-Schleife, Takt, Geräte, Pause, Stufenwechsel | 6–8 |
| RS9 | `net`: WebSocket, Protokoll v2, Snapshot/Delta direkt aus Rust-Typen | 8–10 |
| RS10 | `store`, `watch`, Supervisor/Neustart, Spielstand-Format-Test | 5–7 |
| RS11 | Server-`cmd`, Lasttest gegen beide Server, Parallelbetrieb | 4–5 |
| RS12 | Cutover: Go-Server und `engine/core` löschen, 001 ändern, Docker, CI, Pi-Abnahme (Mensch) | 3–4 |

Summe 57–72 Sessions, ±30 % (ungeprüft, 006: 50–65 + 3–5 für B-726). Jeder Sprint endet mit Review und PR; Go-Server und Rust-Server laufen bis RS12 parallel, der Go-Server bleibt das Orakel.

## Beteiligte und Zielgruppen

Ralf (🧑) entscheidet über jede Freigabe, jeden Gate-Punkt und misst auf dem Pi 3. Agenten setzen um. Nutzen: Spieleabende auf dem Pi, Balancing-/`sim_test`-Läufe, spätere Welten mit mehr Inseln (B-103).

## Anforderungen

**Vorarbeit (RS2, vor jedem Rust-Server-Code):**
1. B-726: Workbench ruft Sim/Balancing/Level per Kindprozess (`cmd/k3c-sim`) statt per Import.
2. B-731 Stufe 1: Zustand ohne JSON-Roundtrip. Dient als **schneller Go-Referenzweg** und als Vergleichswert für den Rust-Server (siehe Kriterium AC-02).
3. Inventar: Zustand, Protokoll-Nachrichten, Spielstand-Felder, Daten-Schemas, Nebenläufigkeit (Locks, Goroutinen), externe Abhängigkeiten (`coder/websocket`, `fsnotify`) als Tabelle; dazu Entscheidung 007 (Entwurf): Laufzeit (tokio oder Threads), WebSocket- und HTTP-Bibliothek, serde, Schema-Erzeugung, Verzeichnis `rust/`, Fehlerbehandlung, Logging mit Emojis (`engine/conlog`).

**Umsetzung (Folgesprints):**
- Orakel: Der Go-Server bleibt bis zum Cutover lauffähig. Protokoll-Traces (Eingaben → Snapshots/Deltas) fester Seeds werden als Golden aufgezeichnet; der Rust-Server muss sie **byte-gleich** reproduzieren, dazu die 22 Dateien in `testdata/golden/`. Kein Golden-Neuschrieb zugunsten von Rust.
- Port bottom-up in Etappen mit je grünem Orakel; nach jeder Etappe läuft `check:diff`, Rust-Tests, clippy.
- Der Rust-Server hält den Zustand dauerhaft in Rust-Typen; Snapshot und Delta entstehen daraus ohne Zwischenformat (kein `map[string]any`-Äquivalent).
- Determinismus auf amd64 und arm64 (kein FMA durch Rust, Golden auf beiden).
- Datenquelle bleibt `data/*.json` (eingebettet oder `K3C_DATA_DIR`), Datei-Watch und Supervisor-Neustart (Exit 75) bleiben im Verhalten gleich (Entscheidung 004).
- Build: `task rust:*`, EXE mit festem Pfad unter `bin/`; Pi: linux/arm64-Binary, Docker-Image bleibt klein (statisch, musl oder `distroless/cc`, Entscheidung in 007).

## Nicht-Ziele

- Lasttest (`k3c-load`), TUI, Workbench, `assetkit`, Starter in Rust (006 Varianten 3–4); `k3c-load`/`k3c-tui` sprechen nur das Protokoll und bleiben Go.
- Protokolländerung, neues Wire-Format (Binary), Regel- oder Balance-Änderung. Etappe 2 ist ein reiner Port.
- Migration alter Spielstände (Ralf: keine Rücksicht); trotzdem ein Format-Test je Seite, solange beide Server laufen.
- Client-Änderung, außer die Protokollversion bleibt unverändert.
- Client-Leistung (Projekt LST).
- Entfernen des Go-Servers vor RS12.

## Regeln und Einschränkungen

Entscheidung 001, 004, 006; Entwurf 007 (RS2.4). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen, Verschachtelung ≤ 4, Komplexität ≤ 15 auch in Rust (Werkzeug für Rust: clippy-Lints und ein Test, wie `tests/projectRules.test.ts` für Dateilänge, im ersten Rust-Sprint zu klären). Nur `rust/…`-RNG würfelt (`rng.rs`), nie eine fremde Crate. **Keine neue Abhängigkeit ohne Zustimmung von Ralf mit der Spec-Freigabe** (Komplexitäts-Budget): Kandidaten tokio, axum oder tungstenite, serde, serde_json, notify, tracing stehen in 007 und gelten erst nach der Freigabe von 007. Nur 2 Spieler gleichzeitig prüfen. Lange Läufe nur über `sim_test`/`check_run`. Ein Sprint bleibt in seiner Domäne. Lokal fehlt ein C-Compiler (Race-Detektor, cgo): Rust-Tests laufen ohne ihn; arm64 prüft die CI (Job `rust`) und der Pi.

## Beispiele

- Orakel: Seed `k3c-dev`, zwei Geräte, 600 Ticks Eingaben aus `testdata/trace/…` → Rust-Server liefert dieselben Snapshot- und Delta-Frames Byte für Byte wie der Go-Server.
- `sim_test` Meilenstein-Lauf (coop2, standard, 40 Tage, 101 Seeds): Bericht wie bei Go, Dauer ≤ Go.
- Pi 3: `bin/k3c-server` (Rust, arm64) mit 2 Räumen × 3 Spielern → Tick p99 < 10 ms (B-042).

## Ausnahme- und Fehlerfälle

- Rust weicht von Golden oder Trace ab: Etappe gilt als nicht bestanden, kein Neuschrieb; Abweichung ist Befund (Fließkomma-Reihenfolge, Map-Reihenfolge, Zahlenformat).
- Ein Abbruchkriterium greift (siehe Akzeptanzkriterien AC-09): Projekt RST pausiert, Ralf entscheidet; bis dahin bleibt der Go-Server unverändert produktiv.
- Pi nicht verfügbar: Pi-Gates blockieren nur die Mensch-Sessions (Hardware entkoppelt), nicht den Port.
- Beide Server laufen parallel: Ein Regel- oder Protokollwunsch während der Parallelphase wird zuerst in Go umgesetzt und dann portiert, oder die Parallelphase pausiert (Entscheidung Ralf).

## Akzeptanzkriterien

- **AC-01** B-726 ist umgesetzt: `go list -deps ./tools/k3c-workbench/...` enthält `k3c/engine/sim`, `room`, `net`, `level` nicht mehr (Ausnahme `studioview`); Ergebnisse von `sim_run`, `sim_test` offline und `level_generate` unverändert (`B-726/AC-01–03`).
- **AC-02** B-731 Stufe 1 ist umgesetzt, `BenchmarkTickDevices4` mindestens 3× schneller, allocs/op ≤ 25 % der Baseline (`B-731/AC-01`); die Zahlen stehen in 006 und dienen als Maßstab für den Rust-Server.
- **AC-03** Das Inventar (Zustand, Protokoll, Spielstand, Daten-Schemas, Nebenläufigkeit, Abhängigkeiten) und Entscheidung 007 (Entwurf) liegen vor; jede neue Rust-Abhängigkeit ist mit Grund aufgeführt.
- **AC-04** Die Pi-3-Baseline nach B-731 (Tick p99, 2 Räume × 3 Spieler) steht in 006 (RS1.2-Nachholung, Ralf).
- **AC-05** Protokoll-Trace-Golden: Aufzeichnung fester Seeds und Eingaben (alle sechs Biome, 2 Spieler, ≥ 600 Ticks, Zeitraffer, `saved`, Game Over) als Golden, vom Go-Server byte-stabil reproduzierbar.
- **AC-06** Der Rust-Server (Skelett, RS3) liefert Handshake und Hello gegen den TS-Client; Tick-Schleife und Snapshot aus Rust-Typen stehen als Spike gemessen auf PC und Pi (Gate G2).
- **AC-07** Sim-Port: Alle 22 Dateien in `testdata/golden/` und die Traces aus AC-05 reproduziert der Rust-Server bitgleich auf amd64 und arm64; `engine/core`-Vergleichstests sind überflüssig und entfallen.
- **AC-08** Cutover: Der Rust-Server ersetzt `cmd/k3c-server` (Supervisor, Neustart 75, Datei-Watch, `saves/`, `reports/`, `data/`-Override); `k3c-load` und TUI laufen unverändert gegen ihn; Docker-Image und CI (Rust-Jobs, arm64) grün; Go-Server, `engine/{sim,room,net,store,watch,core}` gelöscht; 001 angepasst.
- **AC-09** Abbruchkriterien (Gates, Entscheidung jeweils Ralf): **G1** nach RS2: erreicht der Go-Server nach B-731 auf dem Pi Tick p99 ≤ 5 ms bei 2 × 3 Spielern, fragt Claude Ralf, ob Etappe 2 trotzdem weiterläuft (der Treiber Pi ist dann erfüllt). **G2** nach RS3: braucht das Rust-Skelett auf dem Pi für Tick + Snapshot (1 Raum, 3 Geräte, fester Seed) mehr als 50 % der Zeit von Go nach B-731, oder lässt sich der Trace aus AC-05 nach der Sim-Port-Probe (Welt + Monarch) nicht bitgleich treffen → Pause. **G3** Port-Aufwand: überschreiten RS4–RS7 zusammen 1,5× der Schätzung (36 Sessions), Neubewertung. **G4** vor RS12: Pi-Tick p99 im Rust-Server schlechter als im Go-Server nach B-731 → kein Cutover.
- **AC-10** Jeder Sprint endet mit Review, Spec-Freigabe je Sprint durch Ralf; Pi-Abnahmen macht Ralf.

## Offene Fragen

Abgeglichen mit `docs/decisions/007-rust-server.md` › Offene Entscheidungen für Ralf (RS2.4, 2026-11-08); Nummer in 007 in Klammern.

1. **Umfang bei Pause:** Soll Etappe 2 nach RS2 automatisch weiterlaufen, auch wenn G1 erfüllt ist (B-731 erreicht das Pi-Ziel)? Default: Nein, Ralf entscheidet an G1 (Claude fragt, Sprint RS3 bleibt `geplant`).
2. **Reihenfolge Sim vor Server oder Server-Skelett zuerst:** Default: Skelett zuerst (RS3, Spike), damit das Pi-Risiko früh geprüft wird, dann Sim-Port. 007 übernimmt das.
3. **Laufzeit und Bibliotheken** (007: 1): tokio + axum oder blockierende Threads + tungstenite? Entwurf 007: hybrid, Räume auf OS-Threads, Netz auf tokio + axum; Rückfall `tiny_http` + `tungstenite`, falls G2 an Speicher oder Binärgröße scheitert.
4. **Parallelbetrieb:** Gilt „Regel zuerst in Go, dann portieren“ oder friert die Regelentwicklung ein? Default: Regeländerungen laufen weiter in Go (SIM-Domäne) und werden in der Portphase in Rust nachgezogen; das kostet Zeit und ist Treiber für G3.
5. **Docker** (007: 4): statisches musl-Binary (`distroless/static`) oder glibc (`distroless/cc`)? Default: musl, Wechsel nur, wenn RS3 den musl-Allokator als Nachteil misst.
6. **Schema-Erzeugung** (007: 9): Default bestätigt: kein JSON-Schema, Trace-Golden ist das Orakel.
7. **Rust-Lints für Komplexität** (007: 10): clippy `too_many_lines` (60), `cognitive_complexity` (15), `excessive_nesting` (4) auf `deny`, dazu Dateilänge ≤ 400 im Regeltest für `rust/**/*.rs`.
8. **Crate-Liste** (007: 2): `tokio`, `axum`, `notify`, `windows-sys` neu; `serde`, `serde_json` schon da. Einzeln zustimmen oder streichen.
9. **HTTPS** (007: 3): Wird der HTTPS-Port (`K3C_CERTS_DIR`) noch gebraucht? Default: nein, Cargo-Feature `tls` bleibt aus (rustls bringt C-Code mit).
10. **Balancing in Rust** (007: 5): `engine/balance` + `cmd/k3c-sim` (~8 300 Zeilen) hängen an `engine/sim`. Portieren (neuer Sprint RS7b, 10–14 Sessions) oder Go-Sim fürs Balancing behalten (doppelte Regelpflege, AC-08 nicht erreichbar)? Default: portieren.
11. **Umfang anpassen** (007: 6): `devrun`, `conlog` bleiben Go (Workbench); dazu `balance`, `k3c-sim`, `studioview`. Summe 66–86 statt 57–72 Sessions (ungeprüft, ±30 %).
12. **Spielstand nur v9** (007: 7): Migrationen v1–v8 nicht portieren? Default: ja.
13. **Logging** (007: 8): eigenes `conlog` in Rust mit identischem JSON-Format (Default) oder `tracing`?

## Notizen

**Einordnung:** Diese Spec folgt Ralfs ausdrücklicher Wahl (Chat, 2026-11-07), obwohl 006 empfiehlt, zuerst B-731 zu messen. Deshalb ist B-731 hier Vorarbeit mit Gate G1: Zeigt der Pi nach B-731 keinen Bedarf mehr, ist das ein Grund, Etappe 2 zu pausieren, kein Grund, ihn zu überspielen. Der Port-Gewinn auf dem Pi (kein GC, ein Prozess, Rust-Typen statt Map) ist **ungemessen**; der Spike RS1.6 hat nur gezeigt, dass Teil-Umstellung nichts bringt. Spec-Entwurf, Freigabe nur durch Ralf. Zugeordnet: Projekt RST, Sprint RS2 (Vorarbeit), Folgesprints noch nicht angelegt.
