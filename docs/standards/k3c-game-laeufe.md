# Läufe im Game-Client (`k3c-game sim`)

Quelle: Sprint RU33 (B-1017). Die Spiel-EXE führt Sim-, Level-, Welten- und Hub-Läufe aus, per Parameter headless oder mit
Fenster. Die Schnittstelle ist die von `cmd/k3c-sim/main.go` (Kopfkommentar), mit Präfix `sim` und neuen Schaltern; `k3c-sim`
(Go) bleibt, bis RU41 die Go-Engine entfernt. Umsetzung: `rust/crates/k3c-game/src/runs/`.

## Aufruf

```
k3c-game sim <run|level|balance|world|hub> [--input DATEI|-] [--headless|--window] [--speed N] [--data ORDNER] [--stop-stdin]
```

Der Unterbefehl steht direkt nach `sim`, die Schalter davor oder dahinter in beliebiger Folge (`-x`, `--x`, `--x=wert`,
`--x wert`). Ohne `sim` startet die EXE wie bisher das Spiel mit Fenster.

| Schalter | Bedeutung |
|---|---|
| `--input DATEI` | Eingabe-JSON aus der Datei; `-` (Standard) liest stdin |
| `--headless` | ohne Fenster und ohne Grafikkontext (Standard); zusammen mit `--window`: Exit 2 |
| `--window` | Lauf sichtbar im Spiel; nur bei `run` (RU33.3) und `balance` (RU40.2), sonst Exit 2 |
| `--speed N` | Zeitraffer 1 bis 64 (Standard 1), nur mit `--window`, sonst Exit 2; das Ergebnis hängt nicht davon ab |
| `--data ORDNER` | Daten-Ordner; Vorrang vor `K3C_DATA_DIR`, Ordner neben der EXE und den eingebetteten Daten. Die Workbench gibt immer `--data <Checkout>/data` |
| `--stop-stdin` | Abbruch, wenn stdin endet (siehe Abbruch); braucht `--input DATEI` |

## Unterbefehle und Eingabe

Unbekannte JSON-Felder sind Exit 2 (`json: unknown field "x"`), die Namen gelten ohne Groß- und Kleinschreibung wie in Go.

| Unterbefehl | Eingabe-JSON | Ergebnis | Stand |
|---|---|---|---|
| `run` | `{"biome","seed","ticks","inputs":[{"player","fromTick","toTick","moveX","sprint","pay"}]}` wie MCP `sim_run` | `result` mit `text` | RU33.1 |
| `level` | `{"biome","seed"}` wie MCP `level_generate` | `result` mit `text` | RU33.1 |
| `world` | `{"op":"level"\|"values"\|"check",…}` für den Welten-Editor | `result` mit `report` | RU33.2 |
| `hub` | `{"op":"plan","raw":…}` für den Gebäude-Planer | `result` mit `report` | RU33.2 |
| `balance` | JSON mit Flag-Namen als Schlüssel (`{"targets":true,"seed-list":"k3c-dev"}`), Flags der Befehlszeile stehen danach | `progress`, `result` mit `report` oder `report` mit Dateien | RU40 |

Bis ihr Sprint liefert, antworten `world`, `hub` und `balance` mit `{"event":"error","message":"sim <name> folgt mit …"}`
und Exit 2. Biome von `run` und `level`: `forest` (Standard), `cave`, `mine`; `ticks` 1 bis 100000 (30 Ticks je Sekunde),
höchstens 100 Segmente, `player` 0 bis 3.

## Ausgabe

stdout trägt nur JSON-Zeilen, je Zeile ein Objekt mit `event`:

```
{"event":"progress","done":3,"total":6}     nach jedem fertigen Lauf (balance)
{"event":"result","text":"…"}               Ergebnis von run und level
{"event":"result","report":{…}}             world, hub, balance ohne Berichtsdateien
{"event":"report","files":["…json","…md"]}  balance --targets: geschriebene Berichtsdateien
{"event":"error","message":"…"}             vor Exit 1 oder 2
{"event":"aborted"}                         vor Exit 130
```

Die Zeilen sind byte-gleich zu Gos `json.Marshal`: Felder in Go-Reihenfolge, Map-Schlüssel sortiert, `<`, `>`, `&`, U+2028 und
U+2029 als `<`, `>`, `&`, ` `, ` `, Zahlen in Texten über `k3c_det::golden::go_float` (`1`, nicht
`1.0`). Meldungen für Menschen gehen nach stderr mit Emoji (`❌ k3c-game sim: …`, `💥 k3c-game sim: Panik: …`,
`🛑 k3c-game sim: abgebrochen`).

## Exit-Codes

| Code | Bedeutung |
|---:|---|
| 0 | geschafft |
| 1 | Lauf gescheitert oder Panik der Engine (Zeile `error`, `💥 Absturz in der Engine: …`) |
| 2 | Aufruf falsch: Unterbefehl, Schalter, JSON, unlesbare Eingabe |
| 130 | abgebrochen (Zeile `aborted`) |

## Abbruch

`--stop-stdin` startet einen Thread, der stdin bis zum Ende liest; das Ende (die Workbench schließt die Pipe) setzt ein
Zeichen. `run` prüft es vor jedem Takt, `balance` zwischen zwei Läufen; dann folgen die Zeile `aborted` und Exit 130.
Der Aufrufer muss stdin als offene Pipe halten (Windows: nicht `NUL`, in Go `StdinPipe()`), sonst endet jeder Lauf sofort mit Exit 130. Strg+C und CTRL_BREAK beenden die EXE ohne Zeile `aborted` (std kennt keinen Signal-Handler, `unsafe` ist verboten); die
Workbench wertet ein Ende nach eigenem Stop als Abbruch. Eine Zeile bei Strg+C bräuchte die Abhängigkeit `ctrlc` (Freigabe 🧑).

## Mit Fenster

`--window` gilt für `run` und `balance` und nutzt `platform::run_window` mit derselben App und demselben Stepper wie
`--headless`. Zeitraffer 1 (Echtzeit) ist der Standard; nach dem Ende bleibt das Fenster 60 Bilder stehen. Ohne Bildschirm endet
`--window` mit Zeile `error` und Exit 1. `balance` startet mehrere Läufe: headless parallel, mit Fenster nacheinander.

## Berichtsdateien

`balance` schreibt nach `--report-dir` (Workbench: `reports/simtest-<id>/`), Dateien und Inhalt wie bei `k3c-sim`; sie entstehen
erst nach der Rechenphase.

## Unterschiede zu `k3c-sim`

- Präfix `sim`, neue Schalter `--headless`, `--window`, `--speed`, `--data`, `--stop-stdin`.
- Abbruch über stdin-Ende statt Signal; kein Aufruf im Prozess der Workbench (`engine/simcli/call.go` entfällt, ab RU34 startet
  die Workbench die EXE).
- Rechnet mit der Rust-Sim (`k3c-sim`, `k3c-level`); gleiche Eingabe gibt dieselben Zeilen wie Go, geprüft von
  `rust/crates/k3c-game/tests/runs_ref.rs` gegen `testdata/rust/runs/*.json` (Go schreibt sie:
  `go test ./engine/simcli/ -run RustRef -write`). Weicht Rust ab, gilt Go (Ticket statt neues Golden).
