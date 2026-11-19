# B-1121 · Das Rust-Paket enthält keine LIZENZEN.txt und gilt nicht als weitergebbar

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU37
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Das Go-Paket erzeugt `LIZENZEN.txt` mit `native/k3c-game/menu/licensefile`. Das Rust-Spiel (`rust/crates/k3c-game/src/menu/licenses.rs`) hat `license_text()` nur für die Menü-Seite, keine Ausgabe fürs Paket. `rust:game:build` (RU35) legt daher keine `LIZENZEN.txt` nach `dist/k3c-game/`.

## Ziel

Das Rust-Paket enthält `LIZENZEN.txt` (Spiel, Schrift, verwendete Rust-Crates) und ist weitergebbar.

## Beteiligte und Zielgruppen

Entwickler; 🧑 Ralf entscheidet über Weitergabe.

## Anforderungen

- Dieselbe Liste wie die Lizenz-Seite im Menü; Crate-Lizenzen aus `Cargo.lock` (z. B. cargo-about oder Handliste; neues Werkzeug nur mit Freigabe 🧑).

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

- **AC-01** `dist/k3c-game/LIZENZEN.txt` existiert nach `task rust:game:build` und nennt alle Werke der Menü-Seite.

## Offene Fragen

Welches Werkzeug erzeugt die Crate-Lizenzen (neues Werkzeug nur mit Freigabe 🧑).

## Notizen

Aus RU35.1 (Vorentscheidung 3). Bis dahin gilt das Rust-Paket als nicht weitergebbar.
