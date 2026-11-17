# B-1059 · Abnahme RU13: Gen, Bildtests, Race und Sichtprüfung der Standbilder

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU13
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU13 portiert `world` und `actors` nach Rust und liefert `k3c-game still` samt Bildvergleich an fünf Ständen. Die Bildtests laufen nur mit gerenderter Grafik (`K3C_GRAFIK_DIR`) und werden sonst übersprungen; Gen, Race und die Sichtprüfung sind bis zum Sprint-Abschluss gesperrt (B-742).

## Ziel

Die in RU13 aufgeschobenen Läufe sind einmal gelaufen und von Ralf abgenommen.

## Beteiligte und Zielgruppen

Claude fährt die Läufe, Ralf prüft die fünf Standbilder.

## Anforderungen

- Bildvergleich Rust gegen Go ohne übersprungene Tests.
- Race-Lauf der Go-Pakete ohne Befund.
- Sichtprüfung der Standbilder durch Ralf.

## Nicht-Ziele

Fluss-Shader (B-1055), Multiply-Blend der Lichtkarte (B-1056), Canvas-Sortierung nach Textur (B-1058).

## Regeln und Einschränkungen

Golden werden nie zugunsten von Rust neu geschrieben. Sim und Balance bleiben gesperrt, solange der Beschluss vom 2026-11-11 gilt.

- **Bereich:** `rust/crates/k3c-game/**`, `testdata/rust/bild/**`
- **Abhängig von:** RU13.7
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: Abnahme-Lauf ohne neues Verhalten.

## Ausnahme- und Fehlerfälle

Weicht ein Standbild ab, gilt Go: Befund als Ticket, nie ein geändertes Referenzbild oder Golden.

## Akzeptanzkriterien

- **AC-01** Sim: gesperrt (Beschluss 2026-11-11), verschoben bis Widerruf.
- **AC-02** Balance: nicht berührt, entfällt.
- **AC-03** Gen: `task gen:paper`, danach `K3C_STILL_REQUIRE=1 cargo test -p k3c-game still` grün (fünf Bildtests laufen, keiner übersprungen), `testdata/golden` unverändert.
- **AC-04** Race: `check_run task:check:race` grün (ohne C-Compiler als übersprungen nennen).
- **AC-05** Ralf prüft die fünf Standbilder (`k3c-game still …`: morgen, mittag, abend, nacht, nacht-gegner) gegen das Go-Spiel; Tageszeit und Zeichenreihenfolge sehen gleich aus.

## Offene Fragen

keine

## Notizen

Aus RU13: `Actors.lib` (`Rc<RefCell<Library>>`) und `world::Parts` (`Rc<Library>`) bleiben getrennt, Hinweis für RU16/RU26. Nicht portierte Go-Tests mit Grund: Shader-Tests, Alloc-Tests, `LevelSheets(nil)`. Review-Befunde niedrig: `still/raster.rs:146` clamp bei 0-Textur, f32 statt f64 in der Kachel-Affine (`world/layers.rs`).
