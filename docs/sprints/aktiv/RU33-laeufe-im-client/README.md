# RU33 · CLI · Läufe im Game-Client: Sim, Level, Welten und Hub, headless oder mit Fenster per Parameter

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-1017
- **Start-Commit:** 00382aa2d
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Nach G4 ist das Rust-Spiel das Spiel; die Workbench rechnet Sim, Level, Welten-Prüfung und Hub-Planer aber noch mit der Go-Engine im eigenen Prozess (`engine/simcli`, B-924). Beschluss 🧑 2026-11-16: Die Läufe gehören in den Game-Client.

## Ziel

Die Spiel-EXE führt Sim-, Level-, Welten- und Hub-Läufe aus, per Parameter headless oder mit Fenster. Am Ende sichtbar: `k3c-game sim run --headless --input lauf.json` schreibt JSON-Zeilen; derselbe Aufruf mit `--window` zeigt den Lauf im Spiel.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1017 › Anforderungen` (Schnittstelle, headless und mit Fenster, MCP-Tools).

Welle 20 von 26, 4 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell: `opus` für RU33.1 (Schnittstelle festschreiben) und RU33.3 (neue App für Läufe mit Fenster), sonst `sonnet`. Die Sessions laufen als Kette auf `sprint/ru33`.

### Sessions

| Session | Inhalt | Go-Quellen | Zeilen |
|---|---|---|---:|
| 1 | Schnittstelle `docs/standards/k3c-game-laeufe.md`; `sim run`, `sim level` headless; Vergleich mit Go (AC-01, AC-02) | `engine/simcli/`: simcli (175), input (165, ohne balance-Teile); `engine/enginetools/`: run (135), level (70) | 545 |
| 2 | `sim world`, `sim hub` headless (AC-02) | `engine/simcli/`: world (276, T), hub (175, T) | 451 |
| 3 | `sim run --window`: Lauf sichtbar, Zeitraffer, gleiches Ergebnis (AC-03) | – (neue `RunApp`) | – |
| 4 | Review | – | – |

Nicht portiert: `engine/simcli/call.go` (Aufruf im Prozess der Workbench; ab RU34 startet die Workbench die EXE). `balance.go`, `targets.go`, `tune.go`, `sensitivity.go` und `balance_test.go` portiert RU40.1. Neu dazu kommt `engine/enginetools` (run, level), weil `sim run` und `sim level` dessen Text ausgeben.

### Vorentscheidungen

- Die Schnittstelle ist die von `cmd/k3c-sim/main.go` (Kopfkommentar), mit Präfix `sim` und den Schaltern `--headless` (Standard) und `--window`. Neu: `--speed N` (1–64, nur mit `--window`), `--data ORDNER` (Daten-Ordner, die Workbench gibt ihn immer), `--stop-stdin` (Abbruch, wenn stdin endet). JSON-Zeilen und Exit-Codes 0, 1, 2, 130 bleiben gleich.
- JSON-Zeilen sind byte-gleich zu Gos `json.Marshal` (Feldfolge, sortierte Map-Schlüssel, Zahlen über `k3c_det::golden::go_float`, HTML-Escape wie Go).
- `--window` gibt es nur bei `run` und `balance`; sonst Exit 2.
- `sim …` läuft headless über `platform::run_headless`; `--window` nutzt `platform::run_window` mit derselben `App` und demselben `Stepper`.
- Mit `--window` gilt Zeitraffer 1 als Standard; `--speed N` rechnet N Takte je Bild. Das Ergebnis hängt nicht von `--speed` ab.
- `balance` startet mehrere Läufe: headless parallel wie in Go (`engine/balance/parallel.go`), mit Fenster nacheinander (RU40).
- Vergleich mit Go über Fixtures: `engine/simcli/rustref_test.go` (`-write`) schreibt `testdata/rust/runs/<fall>.json`, `rust/crates/k3c-game/tests/runs_ref.rs` vergleicht im Prozess.

### Hinweise für die Umsetzung

- Die Schnittstelle ist der Kopfkommentar von `cmd/k3c-sim/main.go`; sie wird übernommen, nicht neu erfunden.
- Headless ist der Standard für `sim …`; ohne Unterbefehl startet die EXE wie bisher das Spiel mit Fenster.
- Sim-Sperre: nur kurze Testläufe mit festen Eingaben aus den Go-Tests von `engine/simcli`.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Einzige Go-Änderung: die neue Testdatei `engine/simcli/rustref_test.go` (Sessions 1 und 2).

- **Bereich:** `rust/crates/k3c-game/src/runs/**`, `rust/crates/k3c-game/src/lib.rs`, `rust/crates/k3c-game/src/main.rs`, `rust/crates/k3c-game/tests/runs_*.rs`, `engine/simcli/rustref_test.go`, `testdata/rust/runs/**`, `docs/standards/k3c-game-laeufe.md`
- **Abhängig von:** RU36
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

`k3c-game sim run --headless --input lauf.json` → `{"event":"result",…}` und Code 0 ohne Fenster. Derselbe Aufruf mit `--window` → der Lauf ist im Spiel sichtbar, der Bericht ist gleich.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** `docs/standards/k3c-game-laeufe.md` beschreibt die Schnittstelle: Unterbefehle `sim run|level|balance|world|hub`, `--input`, `--headless` (Standard), `--window`, JSON-Zeilen auf stdout und Exit-Codes 0, 1, 2, 130 wie im Kopfkommentar von `cmd/k3c-sim/main.go`.
- **AC-02** `sim run`, `level`, `world` und `hub` laufen headless ohne Grafikkontext und geben für dieselbe Eingabe dieselben JSON-Zeilen aus wie `bin/k3c-sim.exe` (Vergleichstest mit festen Eingaben).
- **AC-03** Mit `--window` läuft `sim run` sichtbar im Spiel (Zeitraffer wählbar) und liefert bei gleicher Eingabe dasselbe Ergebnis wie headless.

## Offene Fragen

Die Sim-Sperre gilt weiter: Die Sessions bauen und testen die Läufe nur mit kurzen Testläufen; echte Sim- und Balancing-Läufe erst nach Widerruf der Sperre (🧑).

Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Abbruch:** über `--stop-stdin` (stdin-Ende setzt den Abbruch, Zeile `aborted`, Exit 130). CTRL_BREAK/Strg+C beendet die EXE ohne Zeile `aborted`, weil std keinen Signal-Handler hat und `unsafe` verboten ist. Will 🧑 die Zeile auch bei Strg+C, braucht es die Abhängigkeit `ctrlc` (Freigabe 🧑).
- **Neue Schalter:** `--speed`, `--data`, `--stop-stdin` (siehe Vorentscheidungen); `--window` bei `level`, `world`, `hub` ist Exit 2.
- **Mit Fenster:** Zeitraffer 1 (Echtzeit) als Standard (B-1017 Frage 3); nach dem Ende bleibt das Fenster 60 Bilder stehen.
- **Ohne Bildschirm:** `--window` endet mit Zeile `error` und Exit 1.
- **Daten-Ordner:** `world` und `hub` brauchen einen Ordner (sie listen `biomes/` und `worlds/`), eingebettete Daten reichen nicht.
- **Abweichung vom Schnitt:** `call.go` wird nicht portiert, `engine/enginetools` (run, level) kommt in Session 1 dazu; Bereich um `main.rs`, `lib.rs`, `tests/runs_*.rs`, `rustref_test.go` und `testdata/rust/runs/**` erweitert.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU33.1 | `RU33.1-schnittstelle-sim-run-level.md` | Umsetzung | autonom | fertig |
| RU33.2 | `RU33.2-sim-world-hub.md` | Umsetzung | autonom | fertig |
| RU33.3 | `RU33.3-sim-run-window.md` | Umsetzung | autonom | fertig |
| RU33.4 | `RU33.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
