# RU25 · SIM · Rust-Balance-Crate Teil 1: Bots

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

`engine/balance` hat 7342 Zeilen in 44 Dateien (Bots, Messungen, Auswertung, Kurven). Balancing ist gesperrt; portiert wird nur der Code mit seinen Tests.

## Ziel

Die Bots spielen in Rust. Am Ende sichtbar: portierte Bot-Tests grün.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 22 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell: `sonnet`. Kette auf `sprint/ru25`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/balance/` → `rust/crates/k3c-balance/src/` (Paket ohne Gruppen: `name.go` → `name.rs`); Dateinamen nach Leitfaden § 2. Der Crate existiert als leeres Gerüst; RU25.1 legt `lib.rs`, `fehler.rs`, `pending.rs`, `testutil.rs` an.

| Session | Go-Dateien | Tests ohne eigene Quelle | Zeilen |
|---|---|---|---:|
| 1 | `engine/balance/`: bots (343) | – | 343 |
| 2 | `engine/balance/`: bot_wirtschaft (334), bot_clear (125, T), bot_abstieg (145, T) | – | 604 |
| 3 | `engine/balance/`: bot_boss (77, T), bot_learn (34, T), crown_bot (36, T), helps (94, T), hints (185, T) | – | 426 |
| 4 | `engine/balance/`: run (190), parallel (92, T), progress (250, T) | balance_test, mix_test | 532 |
| 5 | `engine/balance/`: replay (188, T), island_world (67, T), seedmode (101, T), scope (62); neu `sha256.rs` | profiles_test, bot_perf_test (lang) | 418 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – | – |

Summe: 2323 Zeilen Go (`progress.go` hat heute 250 statt 248 Zeilen). Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle. Weitere Tests ohne eigene Quelle: `economy_test.go` → RU28.3, `fullgame_test.go` und `milestone_test.go` → RU31.4.

### Vorentscheidungen

- `type Bot func(w, p)` wird ein `enum Bot` mit `decide(&self, isl, w, p)`; die Dekoratoren `withDescent`/`withBossFight` sind Varianten mit `inner: Box<Bot>`, die Insel kommt als Parameter (RU25.1).
- Einmal gelesene Daten (Preise, Miniboss-IDs) als `std::sync::OnceLock`, sonst kein globaler Zustand.
- Threads nur in `parallel.rs` (`std::thread::scope`, Ergebnisse nach Index, Fortschritt über `mpsc`); kein `Mutex`.
- `regexp` wird zu kleinen Prüf-Funktionen; `crypto/sha256` zu einer eigenen `sha256.rs` (keine neue Abhängigkeit).
- Was erst RU28/RU31 liefert (Kennzahlen, Auswertung), steht in `pending.rs`; abhängige Tests tragen `ignore = "wartet auf RU28"` bzw. `RU31`.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Balancing-Läufe werden nicht eingeplant; Tests des Pakets laufen.

- **Bereich:** `rust/crates/k3c-balance/**`, `testdata/rust/balance/**`
- **Abhängig von:** RU34
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Jeder Bot aus `engine/balance` trifft in einem festen Lauf dieselben Entscheidungen wie in Go.
- **AC-02** Die Bot-Tests sind portiert und grün; es läuft kein Balancing.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Crate:** `k3c-balance` existiert als leeres Gerüst (B-1014, Stack-Vorschlag); RU25.1 füllt es, kein neuer Crate nötig.
- **SHA-256:** eigene `sha256.rs` statt Crate `sha2` (keine neue Abhängigkeit); 🧑 kann `sha2` freigeben.
- **Bots als `enum`** statt Funktionswerten, Insel als Parameter.
- **Threads** nur in `parallel.rs`; `Crash.stack` bleibt leer.
- **Fixture** `engine/balance/testdata/insights-run7.json` wird nach `testdata/rust/balance/` kopiert (RU25.3), weil RU41 das Go-Paket entfernt.
- **Abweichung vom Schnitt:** Tests ohne eigene Quelle zugeordnet (`balance_test`, `mix_test` → Session 4; `profiles_test`, `bot_perf_test` → Session 5).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU25.1 | `RU25.1-balance-bots.md` | Umsetzung | autonom | fertig |
| RU25.2 | `RU25.2-balance-bot-wirtschaft-clear-abstieg.md` | Umsetzung | autonom | fertig |
| RU25.3 | `RU25.3-balance-bot-boss-learn-crown-helps-hints.md` | Umsetzung | autonom | fertig |
| RU25.4 | `RU25.4-balance-run-parallel-progress.md` | Umsetzung | autonom | fertig |
| RU25.5 | `RU25.5-balance-replay-island-world-seedmode-scope.md` | Umsetzung | autonom | fertig |
| RU25.6 | `RU25.6-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-25: AC-01 und AC-02 erfüllt (Sessions RU25.1 bis RU25.5, Review RU25.6): Bots, Wirtschaft, Boss, Lauf, Replay und Scope in `rust/crates/k3c-balance` portiert, 64 Tests grün, `task rust:lint` grün. Offen: 23 ignorierte Tests (RU28 und lang). Neue Tickets: B-1277, B-1278 (RU28). Keine Sim-, Balance- oder Gen-Läufe; sie liegen in BAL11.
