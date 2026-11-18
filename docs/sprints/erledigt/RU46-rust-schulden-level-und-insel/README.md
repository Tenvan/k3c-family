# RU46 · CLI, SIM · Rust-Schulden in k3c-level und Insel (Second Worker)

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** CLI, SIM
- **Reife:** bereit
- **Tickets:** B-1079, B-1085, B-1083, B-1088
- **Start-Commit:** 048733cfa
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18

## Ausgangslage

Aus den Sprints RU6 (Level-Generator) und RU10 (Sim Insel) in Rust sind vier kleine Schulden offen: zwei Markierungen im Level-Generator (B-1079), Randfälle beim Lesen der Welt-Datei (B-1085), `null`-Felder und Kopf-Typ im Insel-Spielstand (B-1083) und ein Scheindiff im Takt-Vergleich der Insel (B-1088). Sie liegen in `k3c-level`, `k3c-sim/src/island` und `k3c-data/src/world.rs`; RU18, RU21 und RU26 (Hauptrechner) berühren diese Dateien nicht.

## Ziel

Nach dem Sprint lesen Rust-Welt-Datei und Insel-Spielstand die Randfälle wie Go, der Level-Generator trägt keine STAB- und PERF-Marken mehr und der Takt-Vergleich der Insel liefert bei RU19 keinen Scheindiff.

Am Ende sichtbar: Tests je Randfall grün, `grep B-1079` ohne Marken, Level-Vergleich Go gegen Rust `abweichend 0`, B-1079, B-1083, B-1085 und B-1088 erledigt.

## Beteiligte und Zielgruppen

Der Second Worker (Lane `zweit`) setzt um und prüft; der Hauptrechner legt Befund-Tickets an; Ralf entscheidet nichts (kein 🧑-Schritt).

## Anforderungen

B-1079, B-1085, B-1083, B-1088 › Anforderungen und Akzeptanzkriterien. Keine sprint-eigenen Anforderungen.

## Nicht-Ziele

B-1082 (`IslandRef.grade` ist ein von RU18 genutzter Typ), B-1087 (`state/pending.rs`, überschneidet RU18), alles, was an RU18, RU19, RU21, RU22 oder RU26 hängt; Go-Code und Golden ändern; Gate-Messung B-1074.

## Regeln und Einschränkungen

Alle Sessions `Lane: zweit`, Modell sonnet, nacheinander (RU46.1 → .2 → .3 → .4 → .5): gemeinsame Sprint-Branch-Historie und gemeinsame Datei `island/mod.rs`. Vor dem Push `git merge origin/main`. Der Second Worker legt keine Tickets an. Eigene Dateimenge, ohne Kollision mit RU18 (`k3c-sim/src/{building,economy,hub,monarch,skills,merchant}`, `state/pending.rs`, `world/pending.rs`), RU21 (`k3c-room`) und RU26 (`k3c-game/src/game`). Falls RU18 bis dahin `island/` oder `k3c-data/src/world.rs` ändert: vor RU46.2 `git merge origin/main` und Konflikte im Sinne beider.

- **Bereich:** `rust/crates/k3c-level/**`, `rust/crates/k3c-sim/src/island/**`, `rust/crates/k3c-data/src/world.rs`, `docs/standards/rust-port.md`
- **Abhängig von:** RU6, RU10, RU45
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

`cargo test --release -p k3c-level` grün; `"skins": null` in einer Welt-Datei ergibt denselben Fehler wie in Go; `{"version":"9"}` als String ergibt einen Fehler.

## Ausnahme- und Fehlerfälle

Weicht ein Fall von Go ab, den das Ticket nicht nennt, steht er als Befund im Ergebnis (der Hauptrechner legt das Ticket an), kein Fix außerhalb der Erlaubten Dateien. Der Level-Vergleich meldet Abweichungen: Biom und drei Seeds im Ergebnis, kein Golden-Neuschreiben.

## Akzeptanzkriterien

- **AC-01** Kein `STAB(B-1079)` und kein `PERF(B-1079)` mehr in `k3c-level`, Test für den leeren Versteck-Pool, Leitfaden-Absatz, Level-Vergleich `abweichend 0` (`B-1079/AC-01` bis `AC-04`).
- **AC-02** Welt-Datei: `skins: null`, Array statt Objekt und `null`-Werte verhalten sich wie Go (`B-1085/AC-01`, `AC-02`).
- **AC-03** Insel-Spielstand: Kopf-Typ und `null`-Felder wie Go (`B-1083/AC-01` bis `AC-03`).
- **AC-04** `island_fields` bildet leere Listen auf `null` ab, Hilfsfunktion getestet, `testdata/` unverändert (`B-1088/AC-01`, `AC-02`).
- **AC-05** `task check` und `task check:go` grün, PR gemergt.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU46.1 | `RU46.1-level-marker-abbauen.md` | Umsetzung | autonom | fertig |
| RU46.2 | `RU46.2-welt-datei-randfaelle.md` | Umsetzung | autonom | fertig |
| RU46.3 | `RU46.3-insel-save-null-und-kopf.md` | Umsetzung | autonom | fertig |
| RU46.4 | `RU46.4-takt-vergleich-null-leer.md` | Umsetzung | autonom | fertig |
| RU46.5 | `RU46.5-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-18 (Second Worker, autonom): AC-01 bis AC-05 erfüllt, Nachweis in den Ergebnissen RU46.1 bis RU46.5. B-1079 (Marken abgebaut, Level-Vergleich 5000/0), B-1085 (Welt-Datei-Randfälle), B-1083 (Insel-Spielstand) und B-1088 (leer als `null`) umgesetzt. `task check` grün (1950), `check:go` Tests grün (`golangci-lint` lokal fehlt), Rust-Tests grün. Offen für den Hauptrechner (Ergebnis RU46.5): optionale Tickets zu `write_go_float` in `k3c-det` und zu listenfähigen Structs; die End-zu-Ende-Prüfung der Insel (B-1088) folgt mit RU19.
