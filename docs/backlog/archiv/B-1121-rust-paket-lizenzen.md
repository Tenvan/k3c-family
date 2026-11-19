# B-1121 · Das Rust-Paket enthält keine LIZENZEN.txt und gilt nicht als weitergebbar

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** RU57
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Das Go-Paket erzeugt `LIZENZEN.txt` mit `native/k3c-game/menu/licensefile`. Das Rust-Spiel (`rust/crates/k3c-game/src/menu/licenses.rs`) hat `license_text()` nur für die Menü-Seite, keine Ausgabe fürs Paket. `rust:game:build` (RU35) legt daher keine `LIZENZEN.txt` nach `dist/k3c-game/`.

## Ziel

Das Rust-Paket enthält `LIZENZEN.txt` (Spiel, Schrift, verwendete Rust-Crates) und ist weitergebbar.

## Beteiligte und Zielgruppen

Entwickler; 🧑 Ralf entscheidet über Weitergabe.

## Anforderungen

- `LIZENZEN.txt` beginnt mit `license_text()` des Rust-Spiels (Aufruf `k3c-game licenses`, Rust-Seite, Menü-Liste unverändert) und hängt je gelinktem Rust-Crate Name, Version, SPDX-Ausdruck und die Volltexte der LICENSE-, COPYING- und NOTICE-Dateien aus dem Cargo-Cache an (Vorbild `native/k3c-game/menu/licensefile`).
- Das Werkzeug ist ein kleines Go-Programm `tools/rustpaket` (Unterbefehl `lizenzen`, liest `cargo metadata`), **ohne neues Crate und ohne neues Go-Modul**; der Task `rust:game:dist` ruft es auf.
- Fehlt einem gelinkten Crate jede Lizenzdatei, bricht der Aufruf mit dem Crate-Namen ab (wie im Go-Werkzeug), außer das Crate steht in einer kleinen Ausnahmeliste mit SPDX-Text aus dem Werkzeug.
- `LICENSE` und `LICENSE-ASSETS` des Repos kommen ins Paket.

## Nicht-Ziele

Rust-Code des Spiels ändern ohne eigene Session; Release-Workflow (B-1122).

## Regeln und Einschränkungen

Entscheidung 008/011-Pfad, Rust-Leitplanken RU42.

- **Bereich:** `rust/Taskfile.yml`, `rust/crates/k3c-game/src/menu/licenses.rs`
- **Abhängig von:** RU35
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

`task rust:game:build` -> `dist/k3c-game/LIZENZEN.txt` mit denselben Werken wie die Menü-Seite.

## Ausnahme- und Fehlerfälle

nicht relevant: Werkzeugfehler lässt den Build rot werden.

## Akzeptanzkriterien

- **AC-01** `dist/k3c-game/LIZENZEN.txt` existiert nach `task rust:game:build` und enthält alle Werke der Menü-Liste (`license_text()`).
- **AC-02** `LIZENZEN.txt` nennt jedes gelinkte Rust-Crate (`cargo metadata`, Ziel Windows MSVC, normale Abhängigkeiten von `k3c-game`) mit Version und SPDX-Ausdruck und, soweit das Crate sie mitliefert, dem Lizenztext; ohne Datei stehen Quelle und Adresse des Standardtexts da, ohne SPDX-Ausdruck scheitert der Bau.
- **AC-03** `LICENSE` und `LICENSE-ASSETS` liegen im Paket.
- **AC-04** Das Go-Werkzeug hat Tests (Auswahl der gelinkten Crates, Ausgabe, Fehlerfall).

## Offene Fragen

Entschieden (autonom, Claude, 2026-11-19): kein cargo-about; `cargo metadata` genügt für die Crate-Liste. 🧑 widerspricht, falls er ein Fremdwerkzeug wünscht. Die Menü-Liste nennt noch Go-Bibliotheken (B-1139, RU37).

## Notizen

Aus RU35.1 (Vorentscheidung 3). Von RU37 nach RU57 verschoben (2026-11-19), weil RU37 Go-Rückbau ist. Umgesetzt in RU57.1: 132 Crates, davon 14 ohne Lizenzdatei im Crate (u. a. symphonia-* MPL-2.0, gilrs, dasp_sample, profiling, spirv); für sie nennt die Datei Quelle und Standardtext-Adresse. 🧑 entscheidet, ob das für die Weitergabe genügt (MPL-2.0 verlangt Lizenzhinweis und Quellen-Fundstelle für die Binärform).
