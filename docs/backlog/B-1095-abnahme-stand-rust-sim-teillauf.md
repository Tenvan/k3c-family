# B-1095 · Abnahme-Teillauf Rust-Sim: Stand der ignorierten Tests und Go-Seite für RU10, RU11, RU14, RU15

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU47
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Abnahme-Tickets B-1081 (RU11), B-1084 (RU15), B-1086 (RU10) und B-1093 (RU14) verlangen Sim-, Balance-, Gen- und Race-Läufe samt Freischalten der ignorierten Tests. Der größte Teil hängt an RU18 (Wirtschaft, Bau, `create_world`/`new_hub_site`, `step_players`) und RU19 (Events, `decode_world_story`, `emit`); die Sprints laufen auf dem Hauptrechner. Heute sinnvoll ist ein Teillauf: Stand der `#[ignore]`-Marken in Rust (Stand main 2026-11-18: 52 Tests „wartet auf RU10“, 3 RU15, 1 RU17, 114 RU18, 136 RU19, 3 RU21, 2 RU22), Grünstand von `cargo test` und die Go-Seite (Sim-Tests, Golden unverändert). `gen:paper` und `check:race` liefen nach RU14 bereits in RU45.1 (grün, kein DATA RACE) und werden nicht wiederholt. Lane: zweit.

## Ziel

Ein belegter Stand je Marke und je Abnahme-Ticket liegt vor: welche ignorierten Tests heute an echten Platzhaltern (RU18, RU19) hängen und welche mit Marke RU10, RU15 oder RU17 schon freischaltbar wären, dazu der Grünstand von Rust und Go. Die Notizen der vier Abnahme-Tickets nennen ihn; die Tickets bleiben offen, bis RU18 und RU19 stehen.

## Beteiligte und Zielgruppen

Der Second Worker fährt die Läufe (Lane zweit); der Hauptrechner legt aus den Befunden Tickets an und entscheidet über das Freischalten nach RU18.

## Anforderungen

- Nur lesende Läufe: kein Code, kein Golden, keine Marke wird geändert (RU18 ändert dieselben Dateien).
- Ausgaben der Läufe nach `.work/abnahme-stand/` (ignoriert), danach löschen.

## Nicht-Ziele

Freischalten oder Reparieren von Tests, `gen:paper`, `check:race` (liefen in RU45.1), Balance-Läufe gegen Go (brauchen RU18/RU19, bleiben in B-1081, B-1084, B-1086, B-1093), Gate-Messung B-1074.

## Regeln und Einschränkungen

Second-Worker-Regeln (`.claude/skills/second-worker/SKILL.md`): keine Tickets anlegen, Befunde im Ergebnis. Go-Läufe nacheinander (B-284). `task go:golden` und `gen:paper -update` sind verboten.

- **Bereich:** `docs/backlog/B-1081-*.md`, `docs/backlog/B-1084-*.md`, `docs/backlog/B-1086-*.md`, `docs/backlog/B-1093-*.md` (nur Notizen, über MCP)
- **Abhängig von:** RU10, RU11, RU14, RU15
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –

## Beispiele

`cargo test --release -p k3c-sim` meldet z. B. „375 passed; 0 failed; 296 ignored“ (Stand RU14); nach RU18 sinkt die Zahl der ignorierten.

## Ausnahme- und Fehlerfälle

Rote Tests: Name, Ausschnitt der Ausgabe und vermutete Ursache im Ergebnis (Vorschlag Ticket Typ Problem), nichts beheben. Kein C-Compiler ist hier ohne Belang (kein Race-Lauf).

## Akzeptanzkriterien

- **AC-01** `cargo test --release --workspace` (Verzeichnis `rust/`) gelaufen: Zahlen grün/ignoriert/rot je Crate im Ergebnis, 0 rot oder jeder rote Test benannt.
- **AC-02** Tabelle der ignorierten Tests nach Marke (RU10, RU15, RU17, RU18, RU19, RU21, RU22, lang, Messung) und je Crate; Abgleich mit den Angaben in B-1081 (RU10 12, RU14 40, RU15 2, RU18 11, RU19 3, RU21 3, lang 1) und B-1093 (296 ignoriert) mit Erklärung der Abweichung.
- **AC-03** `cargo test --release -p k3c-sim -- --ignored` und `-p k3c-game -- --ignored` gelaufen; je ignoriertem Test mit Marke RU10, RU15 oder RU17 das Ergebnis (grün, Panic `unimplemented` mit Platzhalter-Name, Diff). Gruppiert nach Ursache; die grünen sind die Kandidaten zum Freischalten (Vorschlag für den Hauptrechner).
- **AC-04** Go-Seite: `check_run go:test` (alle Go-Tests der Engine) grün; `git status --short testdata` leer (Golden unverändert); Rust-Referenz-Fixtures `testdata/rust/**` sind die der Go-Seite (keine Neuerzeugung).
- **AC-05** Die Notizen von B-1081, B-1084, B-1086 und B-1093 tragen je einen Absatz „Stand 2026-11-18 (RU47)“ mit den für sie relevanten Zahlen und dem Hinweis, welche Kriterien (Race, gen:paper: RU45.1) schon belegt sind und was an RU18/RU19 hängt; Status der Tickets bleibt `offen`.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

**Stand 2026-11-18 (RU47)**: Ergebnisse in RU47.1 (Cargo-Stand 1381 grün / 326 ignoriert / 4 rot nur `still*`, Marken je RU, `--ignored`-Läufe) und RU47.2 (`go:test` grün, `testdata/` unverändert); Notizen in B-1081, B-1084, B-1086, B-1093 geschrieben (Status unverändert `offen`).

Vorschlag für den Hauptrechner: (1) 39 grüne ignorierte Tests freischalten (26 `k3c-sim`, 3 `k3c-game`, 10 `k3c-room`, Liste im Ergebnis RU47.1), (2) Marke RU10/RU15 an den Insel-Helfern auf RU18 umstellen, (3) Test `island::worlds_tests::eingebettete_welten_gueltig` anpassen (`rom.json` ist eingebettet), (4) Tickets für `overlay_two_players_learn` (Testdaten) und den Atlas-Test (RU17, Ausschnitte in `k3c-platform`), (5) rust-fmt/`still*`-Altlasten aus dem Ergebnis RU45.
