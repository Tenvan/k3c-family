# Port-Leitfaden Go → Rust

Quelle: B-1018 (Abschnitt Anforderungen). Jede Port-Session liest diesen Leitfaden zuerst.

Dieser Text ist der Port-Leitfaden. RU1.1 legt ihn wörtlich als `docs/standards/rust-port.md` ab; jede Port-Session liest ihn zuerst.

### § 1 Grundsatz

- **Go ist das Orakel für Verhalten, nicht für Code.** Lies jede Go-Datei der Session ganz, samt Test, bevor du schreibst. Namen in `snake_case`, Typen in `CamelCase`, deutsche Kommentare übernehmen.
- **Idiomatisches, performantes Rust ist erwünscht** (Entscheidung 012): keine Pflicht zur Go-Rechenreihenfolge, zu `k3c-det`-Mathematik und zu Go-Datenstrukturen (Maps, lineare Suche, Slices). Hinweise in Sprints und Sessions wie „wie Go“ oder „Go-Reihenfolge“ gelten nur für das, was unten verbindlich ist.
- **Verbindlich bleiben:** Reihenfolge der Ereignisse und der rng-Aufrufe, Ganzzahl-Werte, Kampagnen-Ergebnisse und Determinismus innerhalb von Rust (gleicher Seed ergibt bitgleich dasselbe Spiel). Fällt ein Fehler in Go auf: Ticket anlegen; ob Rust ihn nachbaut, entscheidet das Ticket.
- **Bitgleich** (gleicher JSON-Baum, jede Zahl als `f64` mit denselben Bits) gilt nur für den rng. Golden werden nie geändert.
- **Gleiches Ergebnis** gilt für den Level-Generator (RU6): gleiche Chunks und Objekte (Art, Anzahl, Reihenfolge), Fließkomma mit Toleranz 1e-6 relativ (absolut 1e-9 nahe 0).
- **Variantenströme im Level-Generator sind bitgleich (Toleranz 0):** Die Variante einer Entität wird im rng-Strom `<biom>:<seed>:variant:<gruppe>:<go_float(x)>` gewürfelt. Der Text hängt an den bitgleichen Entity-x und an Gos Textform (`go_float`, `%v`); ein anderes Format (etwa `{x}` oder ein anderes Runden) oder ein um ein Bit abweichendes x ergibt eine andere Variante, ohne dass die 1e-6-Toleranz es auffängt. Rust darf den Strom-Text darum nicht anders formatieren; wer die Formatierung beschleunigt (`write_go_float` in `k3c-level` schreibt ohne `String`), belegt den gleichen Text gegen `go_float` im Test und im Level-Vergleich.
- **Gleiches Verhalten** gilt für die Sim (RU22, G3): gleiche Ereignisse in gleicher Reihenfolge, gleiche Ganzzahl-Werte (Gold, Tag, IDs, Zählungen), gleiche Kampagnen-Ergebnisse; Fließkomma mit grober Toleranz, Standard 1e-4 relativ (absolut 1e-9 nahe 0). Zweck des Abgleichs: nur Port-Fehler finden. Alte Sim- und Balancing-Ergebnisse zählen nicht; Balancing und Sim-Läufe werden nach der Umsetzung komplett neu gemacht.

### § 2 Ablage

- `engine/sim/gruppe_name.go` → `rust/crates/k3c-sim/src/gruppe/name.rs`; `gruppe.go` → `gruppe/mod.rs`. Pakete ohne Gruppen: `paket/name.go` → `src/paket/name.rs`. Den Zielordner nennt die Sprint-README.
- Tests: `name_test.go` → `name_tests.rs` daneben, eingebunden mit `#[cfg(test)] mod name_tests;`. Testnamen in `snake_case`, sonst gleich.
- Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Wird eine Rust-Datei länger als ihre Go-Datei erlaubt, teile sie an derselben Stelle, an der ein Abschnittskommentar in Go steht.

### § 3 Typen

| Go | Rust |
|---|---|
| `int` | `i64` |
| `float64` | `f64` (nie `f32`) |
| `string` | `String`, Parameter `&str` |
| `[]T`, `[]*T` | `Vec<T>` in derselben Reihenfolge |
| `*T` als „kann fehlen“ | `Option<T>` |
| `*int` als Verweis | `Option<i64>` |
| `map[K]V` | `BTreeMap<K, V>`; `HashMap` ist in `k3c-sim`, `k3c-level`, `k3c-room`, `k3c-balance` verboten |
| Konstanten-Gruppe aus Strings | `enum` mit `#[serde(rename = "…")]` je Wert |
| `error` | `Result<T, Fehler>` mit einem `enum Fehler` je Crate; Meldungstext wie in Go |
| `panic(…)` | `panic!(…)` mit demselben Text |
| `interface` | `enum`, wenn die Varianten feststehen; sonst `trait` |

JSON: `json:"name"` → `#[serde(rename = "name")]`; `omitempty` → `skip_serializing_if` (0, leerer Text, `false`, leere Liste, `None` fehlen wie in Go); `json:"-"` und kleingeschriebene Felder → `#[serde(skip)]`. Felder in derselben Reihenfolge wie in Go. Go schreibt eine `nil`-Liste als `null` und eine leere als `[]`; Rust schreibt immer `[]`. Der Golden-Vergleich (`k3c_det::golden`) setzt deshalb `null` und `[]` gleich, sonst nichts.

### § 4 Zustand und Besitz

- Der Zustand folgt `docs/standards/rust-sim-zustand.md` (RU8): Listen als `Vec<T>`, Verweise als IDs, keine Rückverweise.
- Verboten in den Sim-Crates: `unsafe`, `Rc`, `RefCell`, `Cell`, `Mutex`, Threads, `async`, Lebenszeit-Parameter an öffentlichen Typen, `macro_rules!`.
- Zwei Dinge zugleich ändern: Schleife über Indizes, oder die innere Liste mit `std::mem::take` herausnehmen und danach zurücklegen. Kein `clone()` ganzer Listen im Schritt.
- Löschen wie in Go an derselben Stelle: `remove(i)`, `retain`, `swap_remove` nur, wo Go tauscht.

### § 5 Determinismus

- Gewürfelt wird nur mit `k3c_rng::Rng`. Jeder Würfel-Aufruf steht an derselben Stelle der Reihenfolge wie in Go, auch wenn das Ergebnis verworfen wird.
- Determinismus innerhalb von Rust bleibt Pflicht: gleicher Seed ergibt bitgleich dasselbe Spiel (Wiedergabe, Spielstände, Koop).
- Gleitkomma im rng: dieselben Rechenschritte wie in Go (bitgleich), kein `mul_add`. Im Level-Aufbau und in der Sim ist die Rechenreihenfolge frei, solange Toleranz (§ 1) und Ereignis-Abgleich halten.
- Sim und Level dürfen std-Funktionen (`powf`, `powi`, `exp`, `ln`, `sin`, `cos`, `f64::max`, `f64::min`) und schnellere Datenstrukturen nutzen; `k3c_det`-Mathematik ist nur im rng Pflicht. Die Reihenfolge der Ereignisse und der rng-Aufrufe bleibt Pflicht.
- Nach RU41 (Go-Engine entfernt) schreibt Rust eigene Golden; die Go-Golden werden dann ersetzt.
- Erlaubt, weil exakt gleich: `abs`, `floor`, `ceil`, `trunc`, `round` (wie `math.Round`), `sqrt`, `%` (wie `math.Mod`), `copysign`.
- Ganzzahlen: `/` und `%` runden in beiden Sprachen zur Null. Überlauf nur im rng, dort `wrapping_*`. `int(f)` → `f as i64`.
- Sortieren: `sort.SliceStable`, `slices.SortStableFunc` → `sort_by`. `slices.Sort` auf Zahlen oder Texten → `sort_unstable` (Gleitkomma: `sort_by(f64::total_cmp)`). `slices.SortFunc` und `sort.Slice` sind nicht stabil: Zeige im Kommentar, dass der Vergleich keinen Gleichstand kennt (zum Beispiel weil die ID mitvergleicht), und nimm `sort_by`. Ist Gleichstand möglich: anhalten, Ticket.
- Keine Uhr (`std::time`) und keine Umgebung in den Sim-Crates.

### § 6 Tests

- Jeder Go-Test wird portiert, Tabellen-Tests als Feld aus Fällen mit Schleife. Hilfen der Tests kommen nach `testutil.rs`.
- Golden und Fixtures liest `k3c_det::golden`; Pfade ab `env!("CARGO_MANIFEST_DIR")` nach `../../../testdata`.
- Ein Test, der noch nicht laufen kann, bekommt `#[ignore = "wartet auf RUnn"]`. Ohne Sprint-Nummer ist `ignore` verboten. Tests, die Go mit `-short` überspringt: `#[ignore = "lang"]`.
- Kein Test fehlt ohne Zeile im Ergebnis der Session (Name und Grund).

### § 7 Was noch fehlt

Ruft eine Funktion etwas aus einem späteren Sprint, kommt in `pending.rs` des Moduls ein Platzhalter mit derselben Signatur: `pub fn name(…) -> T { unimplemented!("RU15: combat_hit") }`. Der Sprint, der die Funktion liefert, löscht den Platzhalter. RU22 prüft, dass keiner übrig ist.

### § 8 Client

- Kein Fachmodul importiert den Zeichen-Stack; gezeichnet wird nur über `platform::Canvas`, gehört nur über `platform::AudioOut`, Eingaben kommen aus `platform::InputState`.
- `ebiten.Image` als Ziel → `&mut dyn Canvas`; als Quelle → `TextureId`. `DrawImage` mit `GeoM` → `canvas.sprite(tex, src, transform, tint)`; `DrawTriangles` → `canvas.mesh`; `vector.*` → `canvas.rect`, `circle`, `line`, `path`; `text.Draw` → `canvas.text`; `inpututil.IsKeyJustPressed` → `input.just_pressed(…)`.
- Die Zeichen-Reihenfolge bleibt die von Go. Logik hängt nie am Ergebnis eines Zeichen-Aufrufs, denn headless zeichnet ins Leere.
- Jede Mechanik läuft mit zwei Spielern; B geht in Menüs zurück; nur Xbox-Symbole.

### § 9 Ablauf einer Port-Session

1. Session-Datei und Sprint-README lesen, dann diesen Leitfaden.
2. Die Go-Dateien der Session und ihre Tests ganz lesen.
3. Rust schreiben, Datei für Datei; nach jeder Datei `cargo build -p <crate>` in `rust/`.
4. Tests portieren, `task rust:test -- -p <crate>`.
5. `task rust:lint`, dann einmal `task check:diff`.
6. Ergebnis in die Session schreiben (`plan_section`), Status `fertig` (`plan_set`), ein Commit mit Domäne im Titel.

Nur die Erlaubten Dateien ändern. Kommst du dreimal am selben Fehler nicht vorbei: anhalten, den Stand ins Ergebnis schreiben, Status `blockiert`. Im Zweifel ein Ticket, keine Vermutung.

### § 10 Log

Jede Log-Meldung beginnt mit einem Emoji aus der Liste in `CLAUDE.md` (ein Codepunkt, kein U+FE0F).

### § 11 Leistung und Stabilität

Ziel: ruckelfrei und absturzfrei (Leitplanken RU42, Ralf 2026-11-18). Die Lints in `rust/Cargo.toml` erzwingen den Teil Stabilität, `k3c-game/tests/alloc_je_bild.rs` den Teil Allokationen.

- **Kein Allokieren im Bild- und Takt-Pfad** (`step`, `draw`, Sim-Takt): Puffer anlegen, wiederverwenden (`clear()` statt neuer `Vec`), kein `format!`, `to_string()`, `clone()` von `String`/`Vec` je Bild. Der Test zählt Allokationen je Bild mit `k3c-alloc-probe` und schreibt eine Obergrenze fest; sinkt der Wert, die Grenze mitsenken.
- **Keine `Rc`/`RefCell`-Geflechte im Zustand:** Besitz klar, Daten flach und per Index verweisen; geteilt wird nur Unveränderliches.
- **Draw-Calls bündeln:** gleiche Textur und gleicher Blend nacheinander, Sprites in einem Rutsch (`canvas.sprite`-Folgen statt Einzelaufrufe mit Zustandswechsel).
- **`Result` statt `panic`:** `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!` sind im Workspace `deny` (in Tests erlaubt). Fehler mit `?` weiterreichen oder mit begründetem Fallback abfangen; `unimplemented!` nur als Platzhalter nach § 7 mit gezieltem `allow` und Sprint-Nummer.
- **Bewusste Schwachstellen markieren:** `// PERF(B-xxxx): Grund` bei bekannter Leistungsschuld, `// STAB(B-xxxx): Grund` mit gezieltem `#[allow(clippy::…)]` bei bekanntem Panic-Risiko; beides mit Ticket im Sprint RU43. Ohne Ticket keine Markierung, neuer Code fügt keine hinzu.
- **Messung und Feinoptimierung am Ende** (Ralf 2026-11-18): erst bauen, dann messen (`messung_cpu`, `alloc_je_bild`), dann nach Messung optimieren (RU43); keine Optimierung ohne Zahl.

### § 12 Abhängigkeiten und unsafe

Fremde Crates stehen mit fester Version (`=`) in `rust/Cargo.toml` › `[workspace.dependencies]`, mit Kommentar zu Datum und Freigabe; neue nur mit Freigabe von Ralf. Der Workspace verbietet `unsafe` (`unsafe_code = forbid`).

| Crate | Version | Zweck | Freigabe |
|---|---|---|---|
| `windows` (Microsoft) | fest gepinnt in RU56.2 (aktuelle stabile Version, mindestens zwei Wochen alt), `default-features = false`, nur die begründeten Features | Win32-Fenster des Cheat-Fensters `k3c-devwin` (RU56, B-1132) | Ralf, 2026-11-19 (Auftrag RU56) |

- **Einzige Ausnahme für `unsafe`:** `rust/crates/k3c-devwin/src/win32/` (Win32-Aufrufe). Das Crate setzt eigene `[lints]` (`unsafe_code = "deny"`, die clippy-Lints des Workspace), das Modul `#![allow(unsafe_code)]`, jeder `unsafe`-Block trägt einen `// SAFETY:`-Kommentar. Ein Test in `k3c-devwin/tests/no_unsafe.rs` hält `unsafe` aus dem restlichen Rust-Workspace heraus.
- `k3c-game` und `k3c-devproto` bleiben `forbid`: Das Spiel spricht das Cheat-Fenster über eine Named Pipe als Client mit `std::fs` an, ohne `unsafe` und ohne `windows`.
