# RU57 · INF · Rust-Paket dist/k3c-game auslieferbar (Lizenzen, Prüfung, Pfade)

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** INF
- **Reife:** bereit
- **Tickets:** B-1121, B-1122, B-1137, B-1138
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19

## Ausgangslage

`task build` liefert seit RU35 das Rust-Spiel nach `dist/k3c-game/`, aber ohne `LIZENZEN.txt` (B-1121), ohne Prüfung der Paketliste, ohne Start-Hinweis und ohne Versionsangabe. `native/measure.ps1`, `docs/messung/rust-ruckler.md` und `release.yml` zeigen teils noch auf alte Pfade (B-1122). Das Paket gilt deshalb als nicht weitergebbar.

## Ziel

`task build` bzw. `task rust:game:build` erzeugt in `dist/k3c-game/` ein vollständiges, startfähiges, weitergebbares Paket (EXE, data, Grafik, Ton, Schrift, Rig-Teile, LICENSE, LICENSE-ASSETS, `LIZENZEN.txt` mit Rust-Crate-Lizenzen, `README.txt`), ohne Go-Reste und Entwickler-Dateien; eine automatische Prüfung hält das fest; der Release-Workflow zippt dieses Paket.

## Beteiligte und Zielgruppen

Claude (Hauptrechner) setzt um und reviewt; 🧑 Ralf prüft das Paket auf einem anderen PC.

## Anforderungen

Keine neue Abhängigkeit (kein Crate, kein neues Go-Modul): Das Werkzeug `tools/rustpaket` liegt im Wurzel-Modul und nutzt nur die Go-Standardbibliothek und `cargo metadata`. Dateien <= 400 Zeilen, Funktionen <= 60 Zeilen. Die Menü-Liste und deren Bildvergleich bleiben unverändert (B-1139 in RU37).

## Nicht-Ziele

Installer, Signierung, Update-Mechanismus, Go-Rückbau (RU37), Menü-Lizenzliste (B-1139), Spielregeln, Protokoll, Spielstand-Format. Nicht in Dateien von RU54 (`view`, `hud`, Bildpfad) und der Look-/Cheat-Fenster-Sprints (RU55, RU56).

## Regeln und Einschränkungen

Sim, Balancing, Messläufe, `check:race` und Gen-Abnahme gesperrt; `gen:paper` nur als Teil des Paketbaus. `rust/Taskfile.yml` und `.github/workflows/*` hält nur diese Spur (RU35 ist durch); `rust/crates/k3c-game/src/main.rs` berühren nur RU57.1 und RU57.2 (RU54 arbeitet in `view/hud/Bildpfad`, kein Overlap).

Planungsangaben für die Arbeitswellen, je eine Zeile:

- **Bereich:** `tools/rustpaket/**`, `rust/Taskfile.yml`, `rust/paket/**`, `rust/crates/k3c-game/src/main.rs`, `native/measure.ps1`, `scripts/**`, `.github/workflows/release.yml`, `.github/workflows/ci.yml`, `docs/messung/rust-ruckler.md`, `docs/messung/rust-abnahme.md`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** taskfile

## Beispiele

`task build` -> `dist/k3c-game/` enthält `k3c-game.exe`, `README.txt`, `LICENSE`, `LICENSE-ASSETS`, `LIZENZEN.txt`, `data/`, `public/grafik|audio|schrift/`, `native/k3c-rig/assets/`; der Ordner lässt sich auf einen anderen PC kopieren und startet dort.

## Ausnahme- und Fehlerfälle

Ein Crate ohne Lizenzdatei oder ein verbotenes Paketfile lässt den Bau scheitern. Fehlt `cargo` oder `go`, meldet der Task es klar.

## Akzeptanzkriterien

- **AC-01** `LIZENZEN.txt` im Paket nennt die Menü-Werke und alle gelinkten Rust-Crates samt Lizenztexten (B-1121).
- **AC-02** `task rust:game:dist:check` prüft Pflicht- und verbotene Dateien, ist Teil von `game:dist` und der CI (B-1137).
- **AC-03** `README.txt`, `LICENSE`, `LICENSE-ASSETS` liegen im Paket; das Menü zeigt die Version aus `git describe` (B-1137, B-1138).
- **AC-04** `measure.ps1`, `ops:backup`, `rust-ruckler.md` und `release.yml` kennen die Pfade; der Release zippt `dist/k3c-game/` (B-1122).
- **AC-05** Das gebaute Paket startet aus `dist/k3c-game` und aus einem fremden Arbeitsordner (Aufnahme-Modus), Station 70–75 in `rust-abnahme.md` ist aktualisiert.
- **AC-06** Review ohne offene schwere Befunde, `task check` und `task check:go` grün.

## Offene Fragen

keine (Defaults autonom entschieden, 2026-11-19): kein cargo-about, Werkzeug in Go im Wurzel-Modul; die Menü-Lizenzliste mit Go-Namen bleibt bis B-1139 (RU37) und wird in `LIZENZEN.txt` durch die echte Crate-Liste ergänzt; 🧑 entscheidet über die Weitergabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU57.1 | `RU57.1-lizenzen-txt-im-paket.md` | Umsetzung | autonom | fertig |
| RU57.2 | `RU57.2-paketpruefung-readme-version.md` | Umsetzung | autonom | fertig |
| RU57.3 | `RU57.3-pfade-release-sichtpruefung.md` | Umsetzung | autonom | fertig |
| RU57.4 | `RU57.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
