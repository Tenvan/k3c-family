# B-1018 · Port-Leitfaden: Regeln, nach denen jede Port-Session von Go nach Rust arbeitet

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU1
- **Projekt:** RSU
- **Erstellt:** 2026-11-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Umstellung (B-1014) besteht zu neun Zehnteln aus Port-Sessions: Go-Dateien lesen, in Rust nachbauen, Tests portieren. Ohne feste Regeln entscheidet jede Session Typen, Besitz, Sortierung und Gleitkomma neu, und die Golden treffen nicht.

## Ziel

Ein Leitfaden, mit dem ein Agent mit Modell `sonnet` eine Port-Session ohne eigene Entwurfsarbeit erledigt und dasselbe Ergebnis liefert wie ein stärkeres Modell.

## Beteiligte und Zielgruppen

Agenten aller Port-Sessions des Projekts RSU. 🧑 gibt den Leitfaden mit der Spec frei.

## Anforderungen

Dieser Text ist der Port-Leitfaden. RU1.1 legt ihn wörtlich als `docs/standards/rust-port.md` ab; jede Port-Session liest ihn zuerst.

### § 1 Grundsatz

- **Go ist die Wahrheit.** Lies jede Go-Datei der Session ganz, samt Test, bevor du schreibst.
- Übersetze Funktion für Funktion in derselben Reihenfolge, mit denselben Namen in `snake_case`; Typen in `CamelCase`. Deutsche Kommentare übernehmen.
- Nichts verbessern, nichts umbauen, nichts beschleunigen. Fällt ein Fehler in Go auf: Ticket anlegen und in Rust so nachbauen, wie Go rechnet.
- **Bitgleich** heißt: gleicher JSON-Baum, jede Zahl als `f64` mit denselben Bits. Golden werden nie geändert.

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
- Gleitkomma: dieselbe Reihenfolge und Klammerung der Rechenschritte wie in Go. `float64(…)`-Hüllen um Produkte in Go fallen weg, die Reihenfolge bleibt. Kein `mul_add`.
- Verboten in den Sim-Crates: `powf`, `powi`, `exp`, `ln`, `sin`, `cos`, `f64::max`, `f64::min`. Dafür `k3c_det::pow`, `exp`, `fmax`, `fmin` (auch für Gos eingebautes `min`/`max` auf Gleitkommazahlen).
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

## Nicht-Ziele

Eine Einführung in Rust. Regeln für die Workbench oder für Werkzeuge. Das Zustandsmodell im Einzelnen (RU8).

## Regeln und Einschränkungen

Der Leitfaden ändert sich nur über dieses Ticket (Revision erhöhen), nie nebenbei in einer Port-Session.

- **Bereich:** `docs/standards/rust-port.md`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

`sort.SliceStable(w.Troops, …)` → `w.troops.sort_by(…)`. `math.Max(a, b)` → `k3c_det::fmax(a, b)`. `w.Coins = append(w.Coins[:i], w.Coins[i+1:]...)` → `w.coins.remove(i)`. Ein Test braucht den Kampf, der erst in RU15 kommt → `#[ignore = "wartet auf RU15"]`.

## Ausnahme- und Fehlerfälle

Eine Regel passt auf eine Go-Stelle nicht → anhalten, Ticket mit Datei und Zeile, Session `blockiert`. Go und Leitfaden widersprechen sich → Go gilt, Ticket gegen den Leitfaden.

## Akzeptanzkriterien

- **AC-01** `docs/standards/rust-port.md` enthält die Abschnitte § 1 bis § 10 wörtlich (RU1.1).
- **AC-02** Jede Session-Datei eines Port-Sprints nennt den Leitfaden im Kontext.
- **AC-03** Nach G2 nennt der Bericht die Stellen, an denen der Leitfaden nicht reichte; sie sind als Revision eingearbeitet.

## Offene Fragen

1. `int` als `i64`: Go rechnet auf den Zielrechnern mit 64 Bit; reicht das als Festlegung (Vorschlag: ja)? 2. Fehler-Typ: eigenes `enum` je Crate statt einer Fehler-Crate (Vorschlag: ja, spart eine Abhängigkeit).

## Notizen

Grundlage: Zustands-Typen in `engine/sim/types.go` (Listen als `[]*T`, Verweise über IDs), `engine/internal/rules/map_range_test.go` (keine Durchläufe über Maps), `engine/internal/golden` (Vergleich als JSON-Baum), Sortier-Aufrufe und `math`-Aufrufe in `engine/sim`, gezählt am 2026-11-16.
