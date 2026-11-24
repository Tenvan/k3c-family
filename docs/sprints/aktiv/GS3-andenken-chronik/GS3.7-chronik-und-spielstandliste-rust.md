# GS3.7 · Chronik im Spielstand und Spielstand-Liste in Rust (ersetzt GS3.1 und GS3.4)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs3
- **Abhängig von:** RU37, RU60
- **Tickets:** B-966, B-978
- **Kriterien:** AC-01, AC-04

## Ziel

Die Sim hält beim Ablegen Insel, Tage und Monarchen fest, Speichern und Laden erhalten die Chronik, und die Liste der Spielstände liefert Route, aktuelle Insel, abgeschlossene Inseln und Chronik.

## Kontext

Spec: `B-966 › Anforderungen` (AC-01 bis AC-03 der Sim-Seite) und `B-978 › Anforderungen` (AC-01), Regeltext `docs/rules/welten.md` § 5/§ 6.1. Die Go-Sessions GS3.1 und GS3.4 sind verworfen (Go-Stopp); diese Session ersetzt sie in Rust (Entscheidung 013): Sim in `rust/crates/k3c-sim` (Insel-Speicherung neben `island/`), Spielstand-Speicher in `rust/crates/k3c-room/src/store/`. Port-Regeln: `docs/standards/rust-port.md`.

Das Spielstand-Format ändert sich: Version erhöhen, Fixture, keine Rücksicht auf alte Spielstände. Keine Simulation, kein Balancing.

## Erlaubte Dateien

- `docs/rules/welten.md`
- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-room/**`

## Nicht-Ziele

Anzeige (GS3.8). Assets (GS3.2). Änderungen an `engine/**` (die Go-Engine bekommt keine neue Mechanik).

## Schritte

1. Chronik-Eintrag beim Ablegen in `k3c-sim` (Insel, Tage, Monarchen), im Spielstand speichern und laden; Version erhöhen, Fixture, Test.
2. Spielstand-Liste in `k3c-room/src/store/`: Route, aktuelle Insel, abgeschlossene Inseln, Chronik je Stand; Test.
3. Regeltext in `docs/rules/welten.md` abgleichen (nur wenn eine Zeile fehlt).

## Fertig, wenn

- [ ] AC-01: Die Sim hält beim Ablegen Insel, Tage und Monarchen fest; Speichern und Laden erhalten die Chronik, Version erhöht, Fixture (B-966).
- [ ] AC-04: Die Liste der Spielstände liefert Route, aktuelle Insel, abgeschlossene Inseln und Chronik (B-978/AC-01).
- [ ] `task check:diff` grün (Rust-Tests der geänderten Crates), kein Sim-, Balance- oder Gen-Lauf.

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

AC-01 geprüft: `island/chronicle.rs` (`ChronicleEntry`, `Island::chronicle_entry`), `Island::chronicle`; `next_island` legt beim Ablegen mit besiegtem Endboss den Eintrag an (Insel-ID, Tag der ersten Stufe, Monarchen = lebend, gekrönt, nicht frei wie bei `all_at_gate`) und zieht die Chronik mit. Spielstand: `IslandSave::chronicle` (optional), `ISLAND_SAVE_VERSION` 13, Fixture `testdata/saves/v13/familie.json` (Rundlauf in `save_tests`), Version-12-Stände laden ohne Chronik. Prüfung `check_chronicle` in `validate_island_save`: Eintrag nur für eine abgeschlossene Insel, jede höchstens einmal, keine negativen Zahlen. Tests `chronicle_tests` (8): Eintrag und Zählung, kein Eintrag ohne besiegten Endboss, zweiter Wechsel hängt an, Speichern und Laden gleich, Feld fehlt ohne Abschluss, Ablehnungen, Version 12.
AC-04 geprüft: `Saves::list` nennt je Stand zusätzlich `islandId`, `completed` und `chronicle`; `Saves::list_with_routes` ergänzt die Route der Welt (`RouteIsland` mit ID, Name, Etappe; Vorgabe Listenplatz + 1) über eine Funktion `world(id) -> Option<WorldDef>` (nur Welt-Datei lesen, kein Insel-Bau, keine Simulation). Kaputte Stände und unbekannte Welten behalten die Route leer, ohne Welt-Angabe gilt `standard`; JSON der alten Felder bleibt unverändert, wenn die neuen leer sind (`saves_route_tests`, 4 Tests). Regeltext `welten.md` § 5 um die Chronik ergänzt.
Geprüft: `cargo test -p k3c-room` grün (242), `task rust:lint` grün. `cargo test -p k3c-sim --lib`: 4 rote Tests, alle nicht aus dieser Session (siehe unten). Review durch k3c-reviewer vor dem Commit (SIM, Spielstand): Zählung der Monarchen und Test für den zweiten Wechsel umgesetzt; offen bleibt nur die Anmerkung, dass die Fixture v13 kein `chronicle` enthält (das Feldformat prüft `chronicle_tests` im Speicher, weil die Fixture-Welt keine abgeschlossene Insel hat) und dass `list_with_routes` auch einen gültigen, aber nicht ladbaren Stand (z. B. Version 99) nur über `error` kennzeichnet (GS3.8 sperrt die Vorschau über den bestehenden Lade-Check).

Abweichungen vom Plan:
- **Feldnamen:** `islandId` und `monarchs` statt `island` und `monarchs` aus dem Ticket-Ausschnitt B-966 (Beispiel nannte `island`): `island` kollidiert mit der Prüfung der Version-8-Tests (`!first.contains("\"island\":")`, der alte Index), `islandId` ist der Name im übrigen Spielstand.
- **Test-Fixture repariert (nicht Teil des Auftrags, aber in `k3c-sim/**`):** `island/testutil.rs` › `one_island_standard` kürzt die Standard-Welt auf eine Insel; Etappen, Wörterbuch und Insel-Geschichte der Fünf-Inseln-Welt (WL2) passten nicht mehr, 334 Tests von `k3c-sim` waren rot („Etappe 2 hat keine Insel“, „Wort pip fehlt im lexicon“). Die Fixture setzt jetzt die Geschichte der Welt auf `null` und entfernt die Story der Insel. Geschichts-Ereignisse im Inselablauf sind dadurch in diesen Tests nicht mehr abgedeckt; `story/*_tests` prüfen die Logik mit eigenen Welten.
- `mod.rs` der Insel hat genau 400 Zeilen (Re-Export von `ChronicleEntry` entfällt, Zugriff über `island::chronicle`).

Nicht aus dieser Session, weiter rot: `k3c-sim` `worlds_tests::eingebettete_welten_gueltig` (eingebettet sind jetzt vier Welten), `worlds_tests::standard_welt_gleich_bisherigen_inseln`, `takt_tests::insel_nach_1000_schritten_wie_go` (Id-Zähler 640 statt 143, vermutet durch den Wald aus WD1.1), `story::mod_tests::standard_welt_erzaehlt_eine_geschichte` (Fixture ohne Geschichte, jetzt erwartbar rot); `k3c-game` `props::plan_tests` (3, gleiche Ursache wie die Fixture, Datei außerhalb der erlaubten Dateien); Go `engine/sim` `TestVorpostenBauNurOhneGefahr` (`task check:diff` Stufe 2 rot, nil-Zeiger, kein Code dieser Session).

### Für den Hauptrechner

- Ticket: Test-Fixtures der Standard-Welt an die Fünf-Inseln-Welt anpassen (`k3c-sim` 4 rote Tests, `k3c-game` `props/plan_tests.rs` 3 rote Tests); `one_island_standard` nimmt jetzt die Geschichte ganz heraus.
- Ticket: Go `engine/sim` `TestVorpostenBauNurOhneGefahr` (nil-Zeiger) macht `task check:diff` Stufe 2 rot.
- Glossar `docs/glossar.md` und Anzeige: GS3.2/GS3.8 (Trophäe, Andenken, Chronik).
- Hinweis für GS3.8: Die Vorschau muss nicht ladbare Stände (gültiges JSON, aber Version 99 oder kaputte Chronik) über den Lade-Check sperren; die Liste nennt dort nur `error` für kaputtes JSON.
- Am TV prüfen (Ralf, bei GS3.8): „Tage · Monarchen“ nach einem Wechsel mit einem toten oder freien Mitspieler.
