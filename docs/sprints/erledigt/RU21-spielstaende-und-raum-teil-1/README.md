# RU21 · SRV · Spielstände und Raum Teil 1

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** SRV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

`engine/store` (689 Zeilen) schreibt Spielstände und Berichte, `engine/room` (2919 Zeilen) hält den Raum mit Insel, Spielern und Ereignissen.

## Ziel

Spielstände speichern und laden, Raum-Grundgerüst in Rust. Am Ende sichtbar: Ein Spielstand aus `testdata/saves` lädt.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 12 von 26, 5 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018) und das Zustandsmodell `docs/standards/rust-sim-zustand.md`. Modell-Empfehlung: `sonnet`, Session 2 `opus` (Gerüst des Raummodells ohne Sperre, Uhr und Thread).

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/store/` → `rust/crates/k3c-room/src/store/`, `engine/room/` → `rust/crates/k3c-room/src/room/`; Dateinamen nach Leitfaden § 2. Die Sessions laufen als Kette auf dem Branch `sprint/ru21`. Gegen das Repo geprüft (2026-11-17): alle Dateien und Zeilenzahlen stimmen; der Crate `k3c-room` besteht als Gerüst (`lib.rs`, Abhängigkeiten auf `k3c-sim`, `k3c-level`, `k3c-data`) und wird hier gefüllt (B-1014 sieht ihn vor, kein neuer Crate). Tests ohne eigene Quelldatei gehören zur passenden Session: `rename_test` zu Session 1, `store_test`, `discard_test`, `memstore_test`/`testsave_test` (Hilfen) zu Session 2, `options_test`, `determinism_test`, `safetick_test`, `victory_test`, `testsave_test` zu Session 3, `review_test` zu Session 4; alles, was Stufen, Pause, Dev, Speichern im Takt, Vorschau, Welten oder Metriken braucht (`depths`, `events`, `island*`, `autosave`, `save`, `saver`, `stages*`, `dev*`, `worlds*`), ist RU24.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `engine/store/`: atomic (96), backups (107, T), reports (143, T), restart (50, T) | 396 |
| 2 | `engine/store/`: saves (223), saves_list (70, T); `engine/room/`: room (315, T) | 608 |
| 3 | `engine/room/`: manager (341), run (149, T) | 490 |
| 4 | `engine/room/`: actions (299), logging (64, T), status (148) | 511 |
| 5 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2005 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Protokoll (`docs/protocol.md`) und Spielstand-Format bleiben unverändert (Version 9). Keine neue Abhängigkeit außer vorhandenen Workspace-Crates (`k3c-rng`, `serde`, `serde_json`).

- **Bereich:** `rust/crates/k3c-room/**`
- **Abhängig von:** RU14
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Spielstände unter `testdata/saves` laden; Schreiben ist atomar wie in Go.
- **AC-02** Der Raum nimmt zwei Spieler an und rechnet den Takt der Insel.
- **AC-03** Die Go-Tests der portierten Dateien sind grün.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Crate:** `k3c-room` existiert als Gerüst; B-1014 nennt ihn in der Aufteilung; Session 1 füllt ihn, kein neuer Crate.
- **Spielstand-Format:** Rust liest und schreibt dasselbe Format wie Go (Version 9, Versionen 1 bis 8 migriert die Sim, RU10); `testdata/saves/v1` bis `v9` müssen laden. Die Regel „keine Rücksicht auf alte Spielstände“ bleibt ohne Wirkung, weil sich das Format nicht ändert (Port ohne Formatänderung, B-1014).
- **Nebenläufigkeit:** keine Sperren und Threads. `Manager`/`Room` sind `&mut self`-Strukturen; `Manager::run(ctx)` wird `tick_all` und `sweep_if_due`, der Aufrufer gibt den Takt vor (RU27).
- **Uhr und Zufall:** Uhr als Pflichtfeld/Parameter (`Stamp`), Zeitformate von Hand (kein `chrono`), Raum-Code aus `k3c_rng` mit Seed des Aufrufers (kein `rand`). `std::thread::sleep` bleibt als einzige Ausnahme in `store/atomic.rs` (Wiederholung beim Umbenennen unter Windows). 🧑 kann stattdessen eine eingespritzte Pause verlangen.
- **Peer-Identität:** `Peer` bekommt `conn_id()`, damit der Vergleich von Gos Interface-Werten möglich bleibt.
- **Logging:** eigenes `trait Log` in `k3c-room` statt `slog`/`k3c-platform` (kein Fenster-Stack im Sim-Crate); der Aufrufer verdrahtet `k3c_platform::log`.
- **HTML-Escape in Berichten:** Nachbehandlung der Ausgabe, damit `json.MarshalIndent` und `serde_json` gleich schreiben.
- **Abweichung vom Schnitt:** keine.

**Nachzug aus Go-Sprints:** keine in diesem Sprint.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU21.1 | `RU21.1-store-atomic-backups-reports-restart.md` | Umsetzung | autonom | fertig |
| RU21.2 | `RU21.2-store-saves-room.md` | Umsetzung | autonom | fertig |
| RU21.3 | `RU21.3-room-manager-run.md` | Umsetzung | autonom | fertig |
| RU21.4 | `RU21.4-room-actions-logging-status.md` | Umsetzung | autonom | fertig |
| RU21.5 | `RU21.5-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

- AC-01 geprüft: `testdata/saves/v1` bis `v9` laden und parsen (`cargo test -p k3c-room`, 84 grün); `task check` und `task check:go` grün.
- AC-02 teilweise: Aktionen, Status, Logging, Fristen und Panic-Schutz geprüft; der Inselschritt im Takt fehlt bis RU18 (`step_players`), `tick_rechnet_beide_spieler` ist `ignore = "wartet auf RU18"`.
- AC-03 teilweise: 10 Tests warten mit Grund (4 RU18, 6 RU24), kein Platzhalter und kein `ignore` mit RU21; Platzhalter nur für RU24 in `pending.rs`.
- Review (k3c-reviewer): Befunde behoben (`rotate` prüft den Slot, `Room::join` lehnt geschlossene Räume ab, `Manager::sweep` entfernt in der Schleife); offen als Hinweis in B-1098: `create` bis RU24 `unimplemented!`.
- Abnahme-Ticket für Sim, Balance, Gen, Race: B-1098 (Lane zweit).
