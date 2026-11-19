# B-1130 · RU55 Abnahme: Aufnahmen gegen K2C-Referenz, Bildvergleich und Frame-Zeit (Lane zweit)

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU64
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

**Lane: zweit** (Second Worker). Sprint RU55 bringt Gehlinie 58 %, Ufer 7 %, Weite ≈ 5 M, Spiegel-Fluss, Tönung, Lichthöfe und Wald nach Rodung ins Rust-Spiel und schließt auf dem Hauptrechner ohne Aufnahme-, Mess- und Race-Läufe ab (B-742). Der Standard `docs/standards/bildaufbau.md` ist das Vorbild (16 K2C-Screenshots von Ralf, nicht im Repo).

## Ziel

Die gesperrten Läufe zu RU55 sind gelaufen: Aufnahmen der Referenz-Szenen in einer und zwei Ansichten, Pixelmessung der Verhältnisse, Bildvergleich gegen das Go-Spiel im Profil `go`, Frame-Zeit und Allokationen.

## Beteiligte und Zielgruppen

Second Worker (Läufe), 🧑 (Blick am TV und gegen die K2C-Screenshots).

## Anforderungen

- **Referenz-Szenen** (je eine Ansicht 1920 × 1080 und zwei Ansichten, mit den vorhandenen Aufnahme-Werkzeugen `tests/still*.rs`, Wiedergabe `replay`, `K3C_LOOK=k2c`): Hub am Tag; Hub in der Nacht mit Lagerfeuer und Mond (Spiegelung); Blutmond; dichter Wald mit Fackel; gerodeter Hub mit Leitmotiv; Lava-Stufe; Abend und Morgengrauen. Aufnahmen liegen in `.work/` (nicht eingecheckt); der Bericht `docs/messung/look-k2c.md` nennt Szene, Seed, Befehl und Messwerte.
- **Pixelmessung:** Gehlinie bei 58 % ± 1 %, Ufer 7 %, Fluss Rest; M ≈ 12 % der Ansicht (eine Ansicht) und ≈ 16 % je Hälfte (zwei Ansichten); Spiegel-Helligkeit 60 bis 70 % der Welt.
- **Bildvergleich Go:** `cargo test -p k3c-game --test e2e_compare` und `e2e_images` im Profil `go` bleiben grün (`K3C_LOOK=go`), `testdata/` unverändert.
- **Frame-Zeit:** schlechtester Frame mit zwei Ansichten auf dem Mini-PC unter 16,7 ms (`tests/messung_cpu.rs`, `native/measure.ps1`), Wert im Bericht; `alloc_je_bild` mit unveränderter Obergrenze grün.
- `task check:race` und `cargo test --workspace` grün.

## Nicht-Ziele

Neue Funktionen, Werte ändern (Befunde als Tickets; die Weite 4 / 4,5 / 5 M entscheidet 🧑 am TV).

## Regeln und Einschränkungen

- **Bereich:** `docs/messung/look-k2c.md`
- **Abhängig von:** RU55.1, RU55.2, RU55.3, RU55.4, RU55.5, RU55.6
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –
Nur auf dem Second Worker (Sim, Race, Gen und Messläufe erlaubt, Beschluss 2026-11-18). Golden werden nie zugunsten von Rust geändert.

## Beispiele

Hub in der Nacht, zwei Ansichten: Gehlinie je Hälfte bei 58 %, Lagerfeuer und Mond hell im Spiegel, Waldrand dunkel.

## Ausnahme- und Fehlerfälle

Weicht die Messung ab, gilt der Standard: Abweichung als Ticket melden, Werte nicht im Bericht beschönigen.

## Akzeptanzkriterien

- **AC-01** Bericht `docs/messung/look-k2c.md` mit allen Referenz-Szenen (eine und zwei Ansichten) und den Pixel-Verhältnissen im Zielbereich.
- **AC-02** Bildvergleich gegen Go im Profil `go` grün, `testdata/` unverändert.
- **AC-03** Schlechtester Frame (zwei Ansichten) < 16,7 ms, `alloc_je_bild` und `task check:race` grün, Werte im Bericht.
- **AC-04** Manuell (🧑): Am TV wirken Nacht, Blutmond, Wald und Hub wie das K2C-Vorbild; 🧑 wählt die endgültige Weite (4, 4,5 oder 5 M).

## Offene Fragen

Weite 4, 4,5 oder 5 M: 🧑 entscheidet am TV; Default 5 M (B-1045).

## Notizen

Aufgeteilt aus den Abnahme-Punkten von B-1043 bis B-1047 (AC-02 und AC-03 dort sind 🧑-Abnahmen am TV).

**Go-Seite aus der Referenz (Ralf 2026-11-19, RU58):** Das Go-Spiel wird entfernt. "Bildvergleich gegen Go" heißt jetzt: `e2e_compare` gegen die gesicherten Go-Bilder `testdata/rust/e2e/go/a/` (RU58.2) und das Rust-Profil `K3C_LOOK=go`; Go-Messwerte (Frame, Start, Speicher) stehen in `docs/messung/go-referenz.json` (RU58.3). `native/measure.ps1` zieht in RU37.1 nach `tools/measure/`.

**Szenenliste (RU55.7, Messungen offen, Lane zweit).** Je Szene eine Ansicht (1920 × 1080) und zwei Ansichten; Profil `K3C_LOOK=k2c`, Vergleich je Szene zusätzlich mit `K3C_LOOK=go`; Aufnahme per Standbild (`k3c-game --shots`, `tests/still*.rs`) oder Wiedergabe, Seed 1, Ablage `.work/`; Bericht `docs/messung/look-k2c.md` mit Szene, Seed, Befehl, Messwert. Voraussetzung: `task gen:paper` (Grafik ist nicht eingecheckt).

| Szene | Zustand | Messziel |
|---|---|---|
| Hub Tag | Phase day, Hub-Stufe 1, Wald im Umkreis | Gehlinie 58 % ± 1 %, Ufer 7 %, Fluss 35 %; M 12 % (eine Ansicht), 16 % je Hälfte |
| Hub Nacht | Phase night, Lagerfeuer, Mond | Hof an Feuer und Mond, Spiegelung 60 bis 70 % der Welt, Lichter hell im Fluss; Tönung bläulich |
| Blutmond | Event bloodMoon | Mond rot und größer, rote Tönung, Figuren lesbar (Split) |
| Dichter Wald | Level forest, 10 und mehr Bäume im Umkreis, Nacht | Dichte 1: Waldwand deckend, Oliv-Tönung, Hof der Fackeln heller |
| Gerodeter Hub | gleiche Welt, Bäume gefällt | Dichte fällt weich (kein Sprung über 0,01 je Bild), Waldwand 35 %, Blick bis zum Horizont |
| Lava | Lava-Stufe (Vulkan) | keine Himmels-Tönung in Höhle und Mine, Lava hell, Spiegel nur mit Himmel |
| Abend | Phase dusk, progress 0,5 | warme Tönung, Dunst fern 0,3 sichtbar |
| Morgengrauen | Phase dawn, progress 0,5 | Übergang Nacht nach Tag stetig |

Messungen je Szene: Frame-Zeit schlechtester Frame (zwei Ansichten, Mini-PC), `alloc_je_bild` unverändert (786), Fenster-Resize mit Spiegel, `task check:race`. Offene Befunde aus dem Review RU55.7 zur Beachtung: Bind-Group des Spiegels je Bild und Größe der Zwischentextur (messen, ggf. cachen); Randsaum des Spiegels (Sampling am Rand des kopierten Streifens); Mond wird über Bäumen gezeichnet; Fackel des Monarchen hat noch keinen eigenen Hof; Leitmotiv (`motif_place`) und `scroll`-Sätze noch ohne Zeichnung (BB1-Assets); Audio-Ersatzbreite im Split (`audio/mix.rs`); Dusk-Ton liegt Richtung Magenta (Standard: lila).
