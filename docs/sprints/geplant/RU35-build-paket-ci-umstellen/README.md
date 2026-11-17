# RU35 · INF · Build, Paket und CI auf das Rust-Spiel umstellen

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** INF
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

`task build` und `task buildall` bauen das Go-Spiel nach `dist/k3c-game/`; die CI baut und testet es unter Windows.

## Ziel

`task build` liefert das Rust-Spiel als Paket, das Go-Spiel bleibt als `task native:game:build` baubar. Am Ende sichtbar: `dist/k3c-game/k3c-game.exe` ist die Rust-EXE mit Grafik und allen Welten.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 18 von 26, 4 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

1. `rust:game:exe`, `rust:game:build` (Paket wie das Go-Paket nach `dist/k3c-rust/`), `rust:game:run`, `rust:game:dev` (AC-01)
2. `task build` und `buildall` auf Rust: Rust-Paket nach `dist/k3c-game/`, Go-Paket nach `dist/k3c-game-go/` (AC-02)
3. CI: Job `rust` baut das Paket und prüft EXE-Größe, `data` und Grafik (AC-03)
4. Review

### Vorentscheidungen

- Kopieren weiter mit `native/k3c-game/cmd/sync`; der Umzug des Werkzeugs vor dem Löschen von `native/` gehört zu RU37.
- Das Rust-Paket enthält ganz `public/grafik` (kein `--assets`-Filter im Rust-Spiel) und `native/k3c-rig/assets` (relativer Rig-Pfad des Rust-Spiels).
- `rust:game:dev` startet den Debug-Bau ohne Neustart bei Änderungen (`engine/devrun` kennt nur Go).

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Kein Rust- oder Go-Code ändert sich; nur Taskfiles, CI und der Abschnitt Befehle in `CLAUDE.md`.

- **Bereich:** `Taskfile.yml`, `rust/Taskfile.yml`, `native/Taskfile.yml`, `.github/workflows/ci.yml`, `CLAUDE.md`
- **Abhängig von:** RU32
- **Braucht 🧑:** –
- **Sperre:** taskfile

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Das Rust-Spiel kennt `build`, `run` und `dev` wie jedes EXE-Projekt.
- **AC-02** `task buildall` legt das Rust-Spiel samt Grafik und `data/` nach `dist/k3c-game/`; das Go-Spiel baut weiter über seinen eigenen Task.
- **AC-03** Die CI baut und testet das Rust-Spiel unter Windows; die Go-Jobs laufen bis RU37 weiter.

## Offene Fragen

Keine offenen.

- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Das Go-Paket zieht nach `dist/k3c-game-go/`, damit `dist/k3c-game/` das Rust-Spiel ist und beide gleichzeitig baubar bleiben.
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): `LIZENZEN.txt` kommt aus dem Rust-Spiel, wenn es eine Lizenz-Ausgabe hat; sonst Ticket in RU35 und das Paket gilt bis dahin als nicht weitergebbar.
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): `data/embed.go` aus dem Bereich genommen (das Rust-Spiel liest `data/` aus dem Paket), `CLAUDE.md` (Abschnitt Befehle) aufgenommen.
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Die CI prüft die EXE-Größe gegen die G1-Schwelle 40 MB.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU35.1 | `RU35.1-rust-game-build-run-dev.md` | Umsetzung | autonom | offen |
| RU35.2 | `RU35.2-build-und-buildall-auf-rust.md` | Umsetzung | autonom | offen |
| RU35.3 | `RU35.3-ci-rust-spiel.md` | Umsetzung | autonom | offen |
| RU35.4 | `RU35.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
