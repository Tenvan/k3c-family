# B-1080 · Abnahme RU6: Level-Generator in Rust (Gen, Race, manuell)

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** RU45
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint RU6 portiert `engine/level` nach `rust/crates/k3c-level`. Sim, Balance, Gen und `check:race` liefen im Sprint nicht (B-742); der Level-Vergleich lief nur als `cargo test` gegen die Golden-Dateien.

## Ziel

Die Abnahme nach dem Sprint: Gen-Lauf (`gen:paper` samt Grafik-Golden), `check:race` auf einem Rechner mit C-Compiler und ein Level-Vergleich Go gegen Rust über mehr Seeds, soweit der Beschluss zu Simulationen (gesperrt bis auf Widerruf) das wieder erlaubt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `task check:race` und `task gen:paper` einmal grün.
- Level-Vergleich Go gegen Rust über viele Seeds je Biom (`level_generate`), sobald Simulationen freigegeben sind.

## Nicht-Ziele

Neue Level-Regeln.

## Regeln und Einschränkungen

Läuft nur auf dem Second Worker (Lane `zweit`; die Sim-Sperre gilt dort nicht, `docs/arbeitsweise.md` › Second Worker). Der Level-Vergleich braucht ein Go-Werkzeug auf der Go-Seite, weil `k3c-sim level` nur Text liefert, kein Baum im Golden-Format.

- **Bereich:** `rust/crates/k3c-level/**`, `engine/level/vergleich_dump_test.go`
- **Abhängig von:** RU6
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –

## Beispiele

`K3C_LEVEL_DUMP=.work/level-vergleich go test ./engine/level -run TestVergleichDump` schreibt je Biom und Seed eine Datei im Golden-Format; `K3C_LEVEL_VERGLEICH=.work/level-vergleich cargo test --release -p k3c-level vergleich_go -- --nocapture` meldet 0 Abweichungen oder nennt Biom, Seed und Pfad der ersten Abweichung.

## Ausnahme- und Fehlerfälle

Fehlt auf dem Rechner ein C-Compiler, läuft `check:race` nicht (still übersprungen): AC-01 gilt dann nur für `gen:paper`, das Ergebnis nennt es, `check:race` bleibt als Ticket offen. Eine Abweichung im Level-Vergleich wird ein Ticket (Typ Problem) und bricht den Lauf nicht ab (alle Biome fertig vergleichen).

## Akzeptanzkriterien

- **AC-01** `task check:race` und `task gen:paper` laufen grün (ohne C-Compiler: `gen:paper` grün, `check:race` als übersprungen genannt, Ticket).
- **AC-02** Level-Vergleich Go gegen Rust über mindestens 1000 Seeds je Biom (alle Biome aus `data/biomes/`) ohne Abweichung (oder Befund als Ticket). Fließkomma 1e-6 relativ, sonst gleich (Entscheidung 012).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Verschoben von RU6 nach RU45 (2026-11-18, Hauptrechner). Umgebung: Second Worker. Sessions RU45.1 (AC-01) und RU45.2 (AC-02).
