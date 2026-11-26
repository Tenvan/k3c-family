# RU67 · DEV · assetkit nach Rust: Asset-Logik des Studios

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-25

## Ausgangslage

`tools/assetkit` (eigenes Go-Modul `k3c/tools/assetkit`, rund 4.000 Zeilen Go und 2.000 Zeilen Tests in 21 Testdateien) enthält die Asset-Logik des Studios (Paket `assets`: Slot-Register, Stand, Katalog, Credits, Web-Quellen, Recherche, Vorschläge, Import, Vorrat, Kandidaten) sowie die Unterpakete `ticket` (Ticket-Bausteine der Workbench-Planung), `windowstate` und `browser`. Entscheidung 014 ist angenommen, der Leitfaden `docs/standards/rust-werkzeuge.md` legt Workspace, Abhängigkeiten und Orakel fest; RU66 legt den Workspace `rust/tools/` samt `tools-rust:*`-Tasks an. Der Papierschnitt-Renderer `tools/paper` (TypeScript) bleibt.

## Ziel

Die Asset-Logik liegt als Rust-Crate mit portierten Tests; die Dateien unter `public/grafik` und die Golden bleiben unverändert (Port ohne Verhaltensänderung).

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft.

## Anforderungen

Port von `tools/assetkit` (alle Pakete: `assets`, `ticket`, `windowstate`, `browser`) nach dem Crate `rust/tools/crates/k3c-assetkit`, Verhalten wie Go, alle Go-Tests mitportiert, ohne Netzzugriff in Tests. Dateiformate (`data/sounds.json`, Recherche-, Vorschlags- und Vorrat-Dateien, `CREDITS.md`, Tickets) bleiben byte-gleich zur Go-Fassung. Der Papierschnitt-Renderer `tools/paper` und die Dateien unter `public/grafik` bleiben unverändert.

## Nicht-Ziele

Studio-Backend und MCP-Tools `assets_*` (RU70), Workbench-Planung (RU69), Entfernen des Go-Codes (RU71). Kein Umbau der Logik, keine neuen Funktionen, kein TLS-Testserver.

## Regeln und Einschränkungen

Port ohne Verhaltensänderung; Go ist das Orakel (Leitfaden `docs/standards/rust-werkzeuge.md`, Entscheidung 014). Datei ≤ 400, Funktion ≤ 60 Zeilen. Die Sessions laufen autonom auf dem Second Worker (`Lane: zweit`), ohne Rückfragen.

### Vorentscheidungen für alle Sessions (Claude, 2026-11-26, nach Leitfaden 🧑 2026-11-26)

- **V1 Crate:** `rust/tools/crates/k3c-assetkit` (Paket `k3c-assetkit`, Bibliothek `k3c_assetkit`, `edition = "2024"`, `publish = false`) im Workspace `rust/tools/` (RU66.1); Eintrag in `members`, falls dort kein Glob `crates/*` steht; führt RU66 `[workspace.dependencies]`, dort eintragen und mit `workspace = true` nutzen. Keine Abhängigkeit auf `k3c-kit` oder Spiel-Crates.
- **V2 Module = Go-Dateien:** je Go-Datei ein Modul gleichen Namens (`vorrat_adopt.go` → `src/vorrat_adopt.rs`), Unterpakete als `src/ticket/` (`mod.rs` = `ticket.go`, `draft.rs`, `files.rs`, `rename.rs`), `src/windowstate.rs`, `src/browser.rs`. Öffentliche Go-Namen → `pub` in snake_case (`SetChoicesIf` → `set_choices_if`), Typnamen bleiben. `lib.rs` re-exportiert die öffentlichen Namen des Go-Pakets `assets` auf der Wurzel (`k3c_assetkit::load` wie `assets.Load`); `ticket`, `windowstate`, `browser` bleiben eigene Module.
- **V3 Tests:** je Go-Testdatei `<name>_test.go` eine Datei `src/<name>_tests.rs` (im Modul per `#[cfg(test)] #[path = "<name>_tests.rs"] mod tests;` oder in `lib.rs`), Testnamen wie Go in snake_case. Gemeinsame Helfer in `src/testutil.rs` (`#[cfg(test)]`). Temp-Ordner `testutil::tempdir()` = `tempfile::tempdir_in(<Repo>/.work/tmp)` (anlegen; `K3C_TMP` gewinnt), Repo-Wurzel = `env!("CARGO_MANIFEST_DIR")` vier Ebenen hoch. Testdaten aus `tools/assetkit/testdata/` byte-gleich nach `rust/tools/crates/k3c-assetkit/testdata/`.
- **V4 Fehler:** `pub enum Error { Stale, Io(std::io::Error), Msg(String) }` in `src/error.rs`, `pub type Result<T>`. `Stale` zeigt den Text von `ErrStale`, `Msg` den Go-Text wörtlich, `Io` den Systemtext. `fmt.Errorf("…: %w")` → `Msg(format!(…))`, außer wo Go mit `errors.Is` unterscheidet (`ErrStale`, `fs.ErrNotExist`): dann Variante erhalten. Tests prüfen wie Go per `contains` auf den Go-Text.
- **V5 JSON wie Go:** `src/gojson.rs`: `marshal_indent(v, escape_html)` = `serde_json::to_vec` → Go-`Compact` → Go-`Indent` (zwei Leerzeichen, Logik aus Go `encoding/json/indent.go`) → bei `escape_html` `<`, `>`, `&`, U+2028, U+2029 als `<` usw. (Go `json.MarshalIndent`). Go-Encoder mit `SetEscapeHTML(false)` + `SetIndent("", "  ")` = `marshal_indent(v, false)` + `\n`. `json.RawMessage` → `Box<RawValue>` (Feature `raw_value`; nie `preserve_order` oder `arbitrary_precision`, die den ganzen Workspace umstellen). `float64` mit `serialize_with = "gojson::float"` (ganze Zahlen ohne `.0`). `omitempty` → `skip_serializing_if`; JSON-Namen und Feldreihenfolge exakt aus den Go-Tags; Maps als `BTreeMap`.
- **V6 HTTP:** `reqwest` 0.12 als normale Abhängigkeit (`default-features = false`, Feature `native-tls`: Schannel, kein C-Compiler), abweichend vom Leitfaden („nur dev“), weil assetkit schon in Go selbst über https lädt. Funktionen mit `context.Context` werden `async fn` (Tokio), Fristen über `timeout` wie in Go. Wo Go einen `*http.Client` nimmt, nimmt Rust `&reqwest::Client`, gebaut von `net::client()` (Proxy aus der Umgebung) oder `net::test_client()` (ohne Proxy) mit `redirect::Policy::none()`; Weiterleitungen folgt `net::send_following` und prüft jeden Sprung wie Go-`CheckRedirect`. `Searcher` hat einen eigenen Client mit Standard-Weiterleitung.
- **V7 Testserver:** `axum` 0.8 + `tokio` (dev), `testutil::serve(router)` liefert `http://127.0.0.1:<port>`. Statt `httptest.NewTLSServer` gilt nur unter `#[cfg(test)]` eine Naht: `net::testing::allow(origin)` (Ersatz für `allowTestServer`/`allowTestHost`), danach akzeptieren https-Zwang und Host-Prüfungen genau diesen Ursprung. Ohne `cfg(test)` ist sie `false`. Der TLS-Handschlag selbst bleibt ungetestet (Grund im Ergebnis).
- **V8 Globale Zustände:** Go-`sync.Mutex` auf Paketebene → `static …: std::sync::Mutex<()>`, nie über `.await` gehalten. `DownloadHosts`, `DownloadPorts`, `VorratHosts` → `pub const`-Slices. Rust-Tests laufen parallel; Tests mit wirklich geteiltem Zustand nehmen `testutil::serial()`.
- **V9 URL, Zeit, Hash, Text:** `url` 2 (Parse-Fehler gilt wie „kein https/kein Host“ mit Go-Text), `net::query_escape` wie Go `url.QueryEscape`. `time.Time` → `chrono::DateTime<FixedOffset>`; `Format("2006-01-02")` → `%Y-%m-%d`, `UTC().Format(time.RFC3339)` → `to_rfc3339_opts(SecondsFormat::Secs, true)`. `sha2`/`sha1` 0.10, Hex selbst. `html.UnescapeString` → `html-escape`, `regexp` → `regex`, `filepath.WalkDir` → `walkdir` mit `sort_by_file_name()`. `image.DecodeConfig` (nur PNG) → `pngsize::size` (Kopf lesen, kein Bild-Crate).
- **V10 Logging:** die zwei `slog.Warn` (`import.go`, `vorrat_adopt.go`) über die Log-Fassade von `k3c-kit` (RU66: `log` oder `tracing`; sonst `log` 0.4), Text mit Emoji wie Go.
- **V11 Funktionstypen und Nebenläufigkeit:** `Progress` → `Arc<dyn Fn(i64) + Send + Sync>` (nil = `None`), `SizeFunc` → `Arc<dyn Fn(String) -> Pin<Box<dyn Future<Output = Result<i64>> + Send>> + Send + Sync>`. Goroutinen → `tokio::task::JoinSet` bzw. in Tests `std::thread::scope`; Ergebnisse in Go-Reihenfolge.
- **V12 Abhängigkeiten:** `serde`, `serde_json` (`raw_value`), `chrono`, `sha2`, `sha1`, `regex`, `url`, `walkdir`, `html-escape`, `reqwest`, `tokio` (`rt`, `sync`, `time`, `macros`), `log`, `windows` (nur `cfg(windows)`); dev: `tempfile`, `axum`, `tokio` (`rt-multi-thread`, `net`). Versionen wie im Lockfile vorhanden, sonst aktuell. `sha1`, `html-escape`, `log` (MIT/Apache, reines Rust, über 1 Mio. Downloads) im Ergebnis melden. Lange Go-Dateien bei Bedarf in `<name>_<teil>.rs` teilen.
- **V13 Kein neues Verhalten:** Go-Fehler nicht beheben, sondern unter `### Für den Hauptrechner` melden (offensichtlicher Absturz darf behoben werden). Go-Code, `tools/paper`, `public/grafik`, `data/` bleiben unverändert; bei Abweichung gilt Go.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/tools/crates/k3c-assetkit/**`, `rust/tools/Cargo.toml`, `rust/tools/Cargo.lock`
- **Abhängig von:** RU66
- **Braucht 🧑:** –
- **Sperre:** deps

## Beispiele

Dieselbe Asset-Spec ergibt in Rust denselben Katalogeintrag und dieselben Prüfmeldungen wie in Go; das echte `data/sounds.json` geladen und gespeichert ändert kein Byte; eine Weiterleitung auf `https://192.168.0.5/x.png` wird wie in Go abgelehnt, ohne dass eine Datei entsteht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go (Abweichung beheben oder Ticket).

## Akzeptanzkriterien

- **AC-01** `assetkit` liegt als Crate `rust/tools/crates/k3c-assetkit` vor, die portierten Tests sind grün (`task tools-rust:test`) und der Lint ist grün (`task tools-rust:lint`).
- **AC-02** Kein Go-Code geändert, `public/grafik` und `data/` unverändert; `task check:diff` grün.
- **AC-03** Jede der 21 Go-Testdateien unter `tools/assetkit` hat eine Rust-Entsprechung; nicht portierbare Tests stehen mit Grund in den Session-Ergebnissen.

## Offene Fragen

keine (Leitfaden, 🧑 2026-11-26)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU67.1 | `RU67.1-crate-register-ticket.md` | Umsetzung | autonom | fertig |
| RU67.2 | `RU67.2-katalog-quellen-recherche.md` | Umsetzung | autonom | fertig |
| RU67.3 | `RU67.3-import-vorrat.md` | Umsetzung | autonom | offen |
| RU67.4 | `RU67.4-kandidaten-vorschlaege.md` | Umsetzung | autonom | offen |
| RU67.5 | `RU67.5-review.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
