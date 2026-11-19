# B-1138 · Das Rust-Spiel zeigt im Menü immer Version unbekannt, weil der Bau K3C_VERSION nicht setzt

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU57
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`menu/version.rs` kennt `set_version`, aber kein Bau und kein `main` ruft sie auf (nur Tests). Das Go-Spiel setzt `menu.Version` per Linker-Flag aus `git describe`. Das Rust-Paket zeigt deshalb im Hauptmenü und in `LIZENZEN.txt` „Version unbekannt“.

## Ziel

Der Rust-Bau setzt `K3C_VERSION` aus `git describe` und `main` ruft `set_version(option_env!("K3C_VERSION"))`; Entwicklungs-Bauten ohne Variable bleiben „unbekannt“.

## Beteiligte und Zielgruppen

Entwickler, Empfänger des Pakets.

## Anforderungen

- `rust:game:exe` setzt `K3C_VERSION` aus `git describe --tags --always --dirty` (Vorbild `native:game:version`).
- `main.rs` ruft beim Start `set_version`, wenn die Variable beim Bau gesetzt war.
- Test: Ohne Variable bleibt `is_unset()` wahr.

## Nicht-Ziele

Winres-Versionsinfo der EXE (eigenes Ticket, falls gewünscht).

## Regeln und Einschränkungen

Bildvergleichs-Goldens setzen die Version selbst (`image_tests.rs`), bleiben unberührt.

- **Bereich:** `rust/Taskfile.yml`, `rust/crates/k3c-game/src/main.rs`
- **Abhängig von:** B-1121
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

`task rust:game:build` -> Hauptmenü unten „Version v0.24.1-12-gabc“.

## Ausnahme- und Fehlerfälle

Kein git (Quellarchiv): Variable leer, Menü zeigt „unbekannt“.

## Akzeptanzkriterien

- **AC-01** Ein per `task rust:game:build` gebautes Paket zeigt die Version aus `git describe` (`LIZENZEN.txt` Kopfzeile nennt sie).
- **AC-02** Ohne `K3C_VERSION` bleibt die Anzeige „unbekannt“ (Test).

## Offene Fragen

keine.

## Notizen

Aus der Paket-Analyse RU57, 2026-11-19.
