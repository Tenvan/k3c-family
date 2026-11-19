# B-1120 · RU32 Abnahme (Lane zweit): Aufnahme beider EXE, Bildvergleich Go gegen Rust, Bericht, Gen

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU32
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

**Lane: zweit** (Second Worker, Rechner ohne die Sperren des Hauptrechners). Sprint RU32 (Ende-zu-Ende-Test) schließt auf dem Hauptrechner ohne Bild-, Gen- und Race-Läufe ab (B-742): `rust/crates/k3c-game/tests/e2e_images.rs` und `e2e_compare.rs` sind fertig und überspringen ohne Umgebung, haben aber nie mit echten Bildern gelaufen. Der Ablauf selbst (`e2e_flow.rs`, Welt-Hash gegen `testdata/rust/e2e/flow.json`) läuft grün; bekannte Abweichung ab Station `gebaut`: B-1106 (Skills lernen fehlt).

## Ziel

Die 14 Stationen sind aus beiden EXE aufgenommen und verglichen, `docs/messung/rust-bildvergleich.md` trägt die Messwerte, jede Station über der Schwelle hat ein Ticket in dem zuständigen RU-Sprint, die gesperrten Läufe des Sprints sind nachgeholt.

## Beteiligte und Zielgruppen

Second Worker setzt um; 🧑 prüft die Sichtprüfung im Spiel (RU36).

## Anforderungen

1. Grafik rendern: `task gen:paper`; Go-EXE und Rust-EXE bauen (`task native:game:exe`, `cargo build --release -p k3c-game`).
2. Go-Bilder aufnehmen: in `native/k3c-game` `K3C_GAME_EXE=<repo>/native/k3c-game/bin/k3c-game.exe K3C_E2E_OUT=<repo>/.work/e2e/go go test ./e2e -run TestFlowImages` (14 Bilder, Läufe a und b pixelgleich).
3. Rust-Bilder aufnehmen: `K3C_RUST_EXE=<repo>/rust/target/release/k3c-game.exe K3C_E2E_OUT=<repo>/.work/e2e cargo test -p k3c-game --test e2e_images` (14 Bilder je Lauf unter `.work/e2e/rust/{a,b}`). Hinweis: `K3C_E2E_OUT` für Go zeigt auf `.work/e2e/go`, für Rust auf `.work/e2e` (der Test hängt `rust/` an). Kennt die Rust-EXE einen Schalter nicht (`--flow --input --shots --at --size`), Ticket im Sprint RU30, nicht umgehen.
4. Vergleich: `K3C_E2E_OUT=<repo>/.work/e2e cargo test -p k3c-game --test e2e_compare -- --nocapture` schreibt `.work/e2e/vergleich.json` und die Bildpaare `.work/e2e/paare/NN-<station>.png`. Schwellen wie `tests/still.rs` (mittel höchstens 0,5, Anteil höchstens 0,005, Pixeltoleranz 24); nie anheben.
5. Bericht `docs/messung/rust-bildvergleich.md`: Tabelle füllen (Rechner, Datum, Befehle, je Station Mittel, Anteil, Ergebnis, Ticket); je Station über der Schwelle ein Ticket mit Projekt RSU und Sprint (Welt und Figuren RU13, Gebäude RU16, Effekte RU17, HUD RU20 und RU21, Menüs RU9 und RU12) und dem Pfad des Bildpaars. Erwartung (ungeprüft): Stationen mit Licht (Nacht) weichen wegen fehlender Lichtkarte ab (B-1056), Stationen nach `gebaut` zusätzlich wegen B-1106.
6. Gesperrte Läufe des Sprints: `task check:race` (Ablauf liegt in Go unverändert; Rust hat nur Tests). Keine Sim-, Balance- oder Lasttest-Läufe (Sperre 2026-11-11 gilt auch für den Second Worker nur dort, wo CLAUDE.md sie aufhebt).
7. Optional, falls Zeit: `cargo test --release -p k3c-game --test e2e_flow` (Laufzeit im Release statt Debug, Debug etwa 15 s je Lauf).

## Nicht-Ziele

Zeichen-Fehler beheben (eigene Tickets), Schwellen anheben, Bilder einchecken, Referenz `flow.json` neu schreiben.

## Regeln und Einschränkungen

Schreiben nur im Repo (`.work/`), danach `.work/e2e` löschen. Golden nie neu schreiben. Bilder nicht einchecken.

## Beispiele

nicht relevant: Abnahme-Ticket; das Ergebnis ist die Tabelle im Bericht.

## Ausnahme- und Fehlerfälle

Fehlt die gerenderte Grafik, überspringen `e2e_images` und `e2e_compare` still: Mit `K3C_STILL_REQUIRE=1` ist das bei `still.rs` ein Fehler, hier prüft der Lauf selbst, dass 14 Bilder je Seite entstanden sind. Weicht die Bildgröße ab, ist das ein Fehler, kein Ticket pro Station.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

**Go-Seite entfällt als Live-Lauf (Ralf 2026-11-19, RU58/RU37):** Das Go-Spiel wird entfernt. Die 14 Go-Stationsbilder (Läufe a und b) nimmt RU58.2 einmal auf und legt sie nach `testdata/rust/e2e/go/a/` (oder begründet nur im Tag); `e2e_compare` liest sie von dort ohne Go-EXE. Schritt 1 (Go-EXE bauen) und Schritt 2 (Go-Bilder aufnehmen) dieses Tickets sind damit durch RU58.2 ersetzt; die Schritte 3 bis 5 (Rust-Bilder aufnehmen, Vergleich, Bericht) laufen weiter hier, gegen die gesicherte Referenz. Bauquelle für Nachaufnahmen: Tag `go-referenz-final`, `docs/standards/go-referenz.md`.
