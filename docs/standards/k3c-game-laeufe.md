# Läufe im Game-Client (`k3c-game sim`)

Quelle: Sprint RU33 (B-1017). Die Spiel-EXE führt Sim-, Level-, Welten- und Hub-Läufe aus, per Parameter headless oder mit
Fenster. Die Schnittstelle ist die des Go-Altstands `cmd/k3c-sim/main.go` (Kopfkommentar), mit Präfix `sim` und neuen Schaltern; das Go-CLI `k3c-sim`
ist entfernt (RU40.4, RU41). Umsetzung: `rust/crates/k3c-game/src/runs/`.

## Aufruf

```
k3c-game sim <run|level|balance|world|hub|save> [--input DATEI|-] [--headless|--window] [--speed N] [--data ORDNER] [--stop-stdin]
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
| `save` | `{"seed","saves"}`: Seed `^[a-z0-9-]{1,27}$`, Ordner der Spielstände (Abschnitt `save`) | `result` mit `text` (`voll-<seed>`) | RU40.2 |

`world` braucht einen Daten-Ordner (`--data`, es listet `worlds/`; eingebettete Daten genügen nicht): ohne ihn Exit 1. `hub` liest nur `buildings.json` und läuft auch mit eingebetteten Daten. `world`, `hub` und `save` kennen `--window` nicht (Exit 2). `balance` läuft headless seit RU40.1 und mit Fenster seit RU40.2 (siehe Abschnitte `balance` und `balance --window`). Biome von `run` und `level`: `forest` (Standard), `cave`, `mine`; `ticks` 1 bis 100000 (30 Ticks je Sekunde),
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

`--window` gilt für `run` (RU33.3) und `balance` (RU40.2) und nutzt `platform::run_window` mit der `RunApp`
(`runs/window.rs`); derselbe `Stepper` rechnet auch headless (`run_headless`), darum hängt das Ergebnis weder vom Modus noch von
`--speed` ab. Die App zeichnet mit den Zeichnern des Spiels (Welt, Bauten, Figuren, Effekte, HUD; Kamera folgt dem Monarchen,
ab zwei Monarchen zwei Zellen übereinander). `--speed N` (1 bis 64) rechnet N Sim-Takte je Schritt der Plattform (30 Hz):
1 ist Echtzeit. Nach dem letzten Takt bleibt das Fenster 60 Bilder stehen; die Zeile `result` folgt, wenn es sich geschlossen
hat. Schließt jemand das Fenster vor dem Ende, folgen `aborted` und Exit 130. Ohne Bildschirm oder Grafiktreiber endet
`--window` mit Zeile `error` und Exit 1, ohne stillen Wechsel auf headless. `balance` startet mehrere Läufe: headless parallel,
mit Fenster nacheinander.

## balance (RU40.1)

Headless wie `k3c-sim balance` (Go-Altstand `engine/simcli/balance.go`): Matrix (`--seeds`, `--seed-list`, `--players`, `--bots`, `--depths`,
`--days`, `--gold-threshold`, `--out`, `--replay-dir`), `--play DATEI`, Ziele (`--targets` mit `--baseline`, `--write-baseline`,
`--depth`, `--bot`, `--grade`, `--world`, `--helps`, `--progress`), `--insights DATEI`, Sensitivität (`--sensitivity`,
`--vary`, `--curves`) und Abstimmung (`--tune sweep|search`, `--param`, `--steps`, `--budget`). Schalter wie Gos `flag`-Paket
(`runs/flags.rs`: `-x`, `--x`, `--x=wert`, `--x wert`, Schalter ohne Wert, `--vary` und `--param` wiederholbar); die Fehltexte
des Pakets sind nicht wörtlich gleich (Exit 2 und Zeile `error` zählen). `--input DATEI|-` liest ein JSON-Objekt mit Flag-Namen
als Schlüsseln (sortiert, vor den Schaltern der Befehlszeile).

Unterschiede zu Go:

- **Zeit in UTC:** `balance-<Zeit>.*`, `tuning-<Zeit>.json`, `sensitivity-<Zeit>.*` tragen die Zeit in UTC (Go: Ortszeit).
- **`--legacy`** (nur Vergleich): rechnet Matrix und `--play` mit den Regeln des Go-Orakels, ohne die nativen Regeln des Rust-Spiels
  (Hafenstart, Wächter-Bäume, Besorgung). `runs_ref.rs` hängt es an jeden `balance`-Fall; `--targets`, `--tune` und
  `--sensitivity` kennen es nicht (sie rechnen mit den nativen Regeln, nicht gegen Go vergleichbar).
- **Endzustand-Hash** von `--play` (`endHash`) gleicht nicht dem von Go (Rust hasht die `Debug`-Ausgabe der Insel, die Rust-Insel
  hat kein JSON); der Vergleich in `runs_ref.rs` lässt dieses Feld aus.
- **Abbruch** (`--stop-stdin`): zwischen zwei Läufen (Matrix, `--targets`, Sensitivität). `--tune` prüft ihn erst nach jedem Punkt,
  nicht nach jedem Lauf (`TuneOptions` hat keinen Rückruf je Lauf). Danach `aborted`, Exit 130, kein Bericht.
- **`--data`** gilt für alle Läufe (Daten im Speicher über `k3c_balance::vary::with_data`).

## balance --window (RU40.2)

`--window` zeigt die Läufe von Matrix, `--targets` und `--play DATEI` nacheinander (nicht parallel) in einem Fenster
(`runs/balance_window.rs`, `BalanceApp`; die Läufe in Takte zerlegt: `runs/balance_run.rs`, die Wahl des Laufs
`runs/balance_windowed.rs`). Jedes Szenario läuft mit dem Bot-Schritt von `k3c_balance::run` (dieselben öffentlichen Bausteine,
daher derselbe Bericht wie headless, bei jedem `--speed`: Test `balance_window_tests.rs`, Vergleich mit der Fixture
`balance-matrix-klein`). Gezeigt wird die erste Stufe mit einem Monarchen; ab zwei Monarchen zwei Zellen übereinander. Ergebnis,
Berichtsdateien, `--out`, `--replay-dir`, `--baseline`, Exit-Codes wie headless; `--progress` meldet nach jedem Lauf.
`--tune`, `--vary`, `--curves`, `--insights` und `--param` kennen kein Fenster (Exit 2, `--window gilt für …`): sie rechnen sehr viele
Läufe oder gar keinen. Fenster zu vor dem Ende: `aborted`, Exit 130, kein Bericht; ohne Bildschirm Zeile `error`, Exit 1.

## save (RU40.2)

`k3c-game sim save` legt den Spielstand „voll ausgebaut“ an, den „Im Spiel ansehen“ der Workbench braucht (Port von
`room.Manager.FullSave` im Dev-Modus, `k3c-room`, `full_save`). Eingabe `{"seed":"…","saves":"<Ordner>"}`; der Stand liegt als
`<Ordner>/voll-<seed>.json` (ein gleichnamiger wird überschrieben), Ausgabe `{"event":"result","text":"voll-<seed>"}`. Der Seed
muss `^[a-z0-9-]{1,27}$` entsprechen, sonst Exit 2 (wie `--window`, fehlender Ordner, unbekanntes Feld). `savedAt` ist die
Wanduhr; gleicher Seed ergibt denselben Stand bis auf dieses Feld. Daten aus `--data` (sonst wie das Spiel).

## Berichtsdateien

`balance` schreibt nach `--report-dir` (Workbench: `reports/simtest-<id>/`), Dateien und Inhalt wie bei `k3c-sim`; sie entstehen
erst nach der Rechenphase.

## Unterschiede zum entfernten Go-CLI `k3c-sim` (seit RU40.4 gelöscht, letzter Stand im Git-Verlauf)

- Präfix `sim`, neue Schalter `--headless`, `--window`, `--speed`, `--data`, `--stop-stdin`.
- Abbruch über stdin-Ende statt Signal; kein Aufruf im Prozess von Workbench oder Studio (sie starten die EXE).
- Rechnet mit der Rust-Sim (Crate `k3c-sim`, `k3c-level`); gleiche Eingabe gibt dieselben Zeilen wie früher Go, geprüft von
  `rust/crates/k3c-game/tests/runs_ref.rs` gegen die eingefrorenen Fixtures `testdata/rust/runs/*.json` (Go schrieb sie bis
  RU40.4; seitdem kein Neuschreiben). Weicht Rust ab, ist das ein Ticket, kein neues Golden.
