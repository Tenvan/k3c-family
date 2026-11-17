# RU3.2 · Go-gleiche pow, exp, fmax, fmin mit Vektoren aus Go

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru3
- **Abhängig von:** RU3.1
- **Tickets:** B-1014
- **Kriterien:** AC-02

## Ziel

`k3c_det::pow`, `exp`, `fmax` und `fmin` liefern auf je 10 000 Vektoren dieselben Bits wie Go.

## Kontext

Die Sim ruft `math.Exp` in jedem Schritt (`engine/sim/economy.go:61`) und `math.Pow` beim Erzeugen von Gegnern und Bossen (`enemies.go:36`, `enemies.go:39`, `boss.go:41`). Rust-Bordmittel rufen die C-Laufzeit und können in der letzten Stelle abweichen. Die Go-Quelle steht unter `$(go env GOROOT)/src/math`: `pow.go`, `exp.go`, dazu `exp_asm.go`/`exp_noasm.go` und Dateien je Rechnerart. **Ungeprüft:** ob Go auf amd64 für `Exp` Assembler nutzt; das klärt Schritt 1. Maßgeblich ist amd64 (Zielrechner PC). `math.Max`/`math.Min` stehen in `dim.go`: NaN gewinnt, `+0` schlägt `-0` bei `Max`. Port-Leitfaden: `docs/standards/rust-port.md` (bis RU1.1 gemergt ist: `plan_get B-1018`, Abschnitt Anforderungen).

## Erlaubte Dateien

- `rust/crates/k3c-det/**`
- `engine/internal/detvec/**`
- `testdata/rust/**`

## Nicht-Ziele

Andere Funktionen als die vier (plus die Hilfen, die `pow.go` braucht). Änderungen an `engine/sim`.

## Schritte

1. `go env GOROOT`; in `src/math` lesen, welche Fassung von `Exp` und `Pow` amd64 nutzt (`haveArchExp`, Build-Zeilen). Ergebnis ins Ergebnis der Session schreiben.
2. Go-Test `engine/internal/detvec/detvec_test.go`: erzeugt mit `engine/rng` (Seed `detvec`) je Funktion 10 000 Eingaben. `pow`: Basis 0,5 bis 3, Exponent 0 bis 40, dazu Randfälle (0, 1, negativ, ganzzahlig, NaN, ±Inf). `exp`: −50 bis 50 und Randfälle. `fmax`/`fmin`: auch NaN und ±0. Je Zeile `{"f":"pow","a":"<16 Hex-Stellen>","b":"…","r":"…"}` (Bits als Hex). Mit `-write` schreibt er `testdata/rust/det-vectors.json.gz`, ohne vergleicht er.
3. `pow`: `pow.go` Zeile für Zeile übertragen, samt `isOddInt` und den Hilfen `frexp`, `ldexp`, `modf` aus der Go-Quelle; `sqrt` darf `f64::sqrt` sein.
4. `exp`: die Fassung übertragen, die amd64 nutzt (reines Go: `exp` und `expmulti` aus `exp.go`; Assembler: Schritt für Schritt mit denselben Konstanten und derselben Reihenfolge).
5. `fmax`, `fmin` nach `dim.go`.
6. Rust-Test liest die Vektoren und vergleicht `to_bits()`; bei Abweichung Funktion, Eingaben und beide Ergebnisse ausgeben.
7. Treffen die Bits bei `exp` nach ehrlichem Versuch nicht: anhalten, Status `blockiert`, Ticket mit den abweichenden Vektoren (🧑 entscheidet).

## Fertig, wenn

- [ ] AC-02: `cargo test -p k3c-det` vergleicht 40 000 Vektoren ohne Abweichung.
- [ ] `go test ./engine/internal/detvec/` ist grün (Vergleich ohne `-write`).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- Schritt 1 (Go 1.27.3, windows/amd64, `GOAMD64=v1`): `Exp` → Assembler `exp_amd64.s` (`haveArchExp`), mit Laufzeit-Wahl `useFMA = AVX && FMA`; `Log` → Assembler `log_amd64.s` (`haveArchLog`); `Pow`, `Frexp`, `Ldexp`, `Modf` → reines Go (`haveArchPow` usw. falsch); `Max`/`Min` → `dim_amd64.s` (Regeln wie `dim.go`).
- AC-02 geprüft: `cargo test -p k3c-det` vergleicht 50 081 Vektoren (`pow`, `exp`, `log`, `fmax`, `fmin` je ≥ 10 000) ohne Abweichung, auch mit `--release`; `go test ./engine/internal/detvec/` grün (Vergleich ohne `-write`); `task check:diff` grün.
- Port: `exp` = FMA-Pfad aus `exp_amd64.s` über `mul_add` (einzige Stelle für `mul_add`), `log` = `log_amd64.s` (nicht `log.go`: kein Normalisieren denormaler Zahlen, Vergleich mit √2/2 „nicht kleiner“; Mutationsprobe `<` statt `<=` schlägt bei `log(ldexp(√2/2, k))` fehl), `pow`, `frexp`, `ldexp`, `modf` aus der Go-Quelle, NaN mit Gos Bits `0x7FF8000000000001`.
- Abweichungen vom Plan: zusätzlich `log` (Hilfe von `pow`) mit eigenen Vektoren; die Datei ist eine JSON-Liste der Zeilen `{"f","a","b","r"}`. Gos SSE-Pfad für `Exp` (CPU ohne AVX/FMA) ist nicht portiert: `GODEBUG=cpu.fma=off` wirkt unter Windows nicht, der Pfad ist mit Go hier nicht prüfbar; Go-Test erkennt den Pfad an einem Probe-Wert und überspringt dann `exp`/`pow`. `flate2 =1.1.11` als Test-Abhängigkeit (Spec nennt flate2), `serde_json` mit `float_roundtrip`.
- Neue Tickets: B-1023 (Go-`math.Exp` hängt an AVX/FMA, Entscheidung 🧑, Sprint RU22).
