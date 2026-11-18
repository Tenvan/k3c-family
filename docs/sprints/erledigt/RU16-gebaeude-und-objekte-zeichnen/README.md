# RU16 · CLI · Gebäude und Objekte zeichnen

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** be144f5d7
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

`props` (2521 Zeilen, 18 Importe der Sim) zeichnet Gebäude, Baustellen, Münzen und Objekte.

## Ziel

Gebäude und Objekte eines Zustands erscheinen im Rust-Spiel. Am Ende sichtbar: Bildvergleich einer ausgebauten Basis.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 6 von 26, 7 Sessions (B-1015; eine mehr als geplant: der Bildvergleich bekommt eine eigene Session, weil AC-01 einen Bildvergleich verlangt und `still` samt Go-Referenzbildern mehr ist als ein Nebenschritt).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/props/` → `rust/crates/k3c-game/src/props/`; Dateinamen nach Leitfaden § 2. Die Sessions laufen als Kette auf dem Branch `sprint/ru16` (eine nach der anderen, ein Commit je Session). Die Reihenfolge weicht von der ursprünglichen Tabelle ab, weil `decor`, `decor_glow` und `paint` Methoden und Typen des Planers aus `plan.go` brauchen und `listen` die Texte und Regeln aus `sites.go` (Rust kann `impl Planner` nicht vor dem Typ schreiben). `felling_test.go` gehört zu `plan_objects.go`.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/props/`: assets (45, T), data (126), buildings (243, T), coin_fountain (23, T); dazu die Naht `SimView` (neu, `simview.rs`, `pending.rs`) | 437 |
| 2 | `native/k3c-game/props/`: nature (215, T), sites (267, T) | 482 |
| 3 | `native/k3c-game/props/`: plan (350, T), listen (206, T) | 556 |
| 4 | `native/k3c-game/props/`: plan_combat (130, T), plan_objects (88, T über `felling_test.go`), plan_sites (314) | 532 |
| 5 | `native/k3c-game/props/`: decor (118, T), decor_glow (130), paint (214), props (52) | 514 |
| 6 | Standbild `k3c-game still` (Ebene `props`) und Bildvergleich an fünf Ständen (Go-Test `props/rustref_test.go`) | – |
| 7 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2521 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, Session 6 liefert den Bildvergleich, das Review prüft alle.

**Naht `SimView`.** Die Sim-Funktionen `EconomyOf`, `SiteGoldLeft`, `SitePayable`, `SiteNeedsClearing`, `ShieldRack`, `CombatSpots`, `AtLair` fehlen in `k3c-sim`, bis RU15 und RU18 (Wellen 10 und 12) sie bringen. `props` (und `hud` in RU20) rufen sie über `trait SimView` (RU16.1); der Bildvergleich bekommt die Antworten als Orakel aus Go (`*.simview.json`). Siehe Offene Fragen.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Einzige Go-Änderung: die neue Testdatei `native/k3c-game/props/rustref_test.go` (Session 6). `k3c-platform` und `k3c-sim` bleiben unverändert.

- **Bereich:** `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/still/**`, `rust/crates/k3c-game/src/lib.rs`, `rust/crates/k3c-game/src/main.rs`, `rust/crates/k3c-game/tests/**`, `native/k3c-game/props/rustref_test.go`, `testdata/rust/bild/**`
- **Abhängig von:** RU13
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Gebäude und Baustellen sehen an fünf Ständen aus wie im Go-Spiel.
- **AC-02** Objekte und Natur stehen an den Positionen des Zustands; die Paket-Tests sind portiert.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Naht `SimView` statt Platzhalter:** Die Sim-Funktionen, die `props` ruft, kommen erst in Welle 10 bis 12. Ein `unimplemented!`-Platzhalter nach Leitfaden § 7 verhindert den Bildvergleich. Default: `trait SimView` mit `PendingSimView` (Platzhalter) und im Bildvergleich `FixedSimView` aus Go-Antworten; RU15 und RU18 löschen die Naht. Betrifft auch `hud` (RU20) und eventuell `actors` (RU13: prüfen, ob dort dieselbe Naht nötig war).
- **Bildvergleich:** eigene Session 6 (7 statt 6 Sessions); `still` aus RU13.6 wird erweitert, kein zweites Werkzeug.
- **Reihenfolge:** `plan` und `listen` vor `decor` und `paint` (Typ-Abhängigkeiten), siehe Anforderungen.
- **Tests mit Level-Generator:** Welten per Struct-Literal wie in Go; wo Go `sim.NewWorld` ruft (`decor_test.go`), `#[ignore = "wartet auf RU10"]` nur wenn Literal nicht geht.

**Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Das Go-Spiel wächst nicht weiter). Diese Arbeiten entstehen direkt im Rust-Client, sobald die Sim-Seite steht. Spec steht im Ticket:

- SI1.4 Ladeplätze und Fracht an Deck (B-997)
- SI2.4 Doppel-Truhe im Spiel (B-998)
- SI5.4 Hütte, Figur und Bauwerk im Spiel (B-1001)
- WE6.2 Skins der Welt für Gegner, Objekte, Gebäude und Schiff (B-994)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU16.1 | `RU16.1-props-assets-data-buildings-coin.md` | Umsetzung | autonom | fertig |
| RU16.2 | `RU16.2-props-nature-sites.md` | Umsetzung | autonom | fertig |
| RU16.3 | `RU16.3-props-plan-listen.md` | Umsetzung | autonom | fertig |
| RU16.4 | `RU16.4-props-plan-combat-objects-sites.md` | Umsetzung | autonom | fertig |
| RU16.5 | `RU16.5-props-decor-paint-props.md` | Umsetzung | autonom | fertig |
| RU16.6 | `RU16.6-standbild-bildvergleich.md` | Umsetzung | autonom | fertig |
| RU16.7 | `RU16.7-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-18, Review RU16.7. AC-01 und AC-02 geprüft: Ergebnisse RU16.1 bis RU16.6, Bildvergleich an sieben Ständen (`still_props`, Text maskiert, B-1063), `cargo test -p k3c-game` grün (313 + Integrationstests, 9 ignoriert: acht `wartet auf RU10` in `props`), `task check` und `task check:go` grün; `engine/`, `testdata/golden`, `k3c-platform` unverändert, Go nur `rustref_*_test.go`.
Review k3c-reviewer: keine HOCH-Befunde; MITTEL: `new_pending` mit `unimplemented!()` (nicht verdrahten bis RU15/RU18), Allokationen je Frame in `label` (RU39), Bildtest nur mit Grafik. `SimView` (inkl. `spot_near`, `spot_state`, `site_visible`) bestätigt, in B-1053 (RU30) genannt.
Neue Tickets: B-1061 (erledigt), B-1063 (Text 2 px), Abnahme-Ticket B-1064 (Sim, Balance, Gen, Sichtprüfung). Nachzüge B-997, B-998, B-1001, B-994 nicht Teil des Sprints, Stand unverändert offen.
