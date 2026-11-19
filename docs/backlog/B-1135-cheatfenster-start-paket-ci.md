# B-1135 · Cheat-Fenster: Start per F4 und Schalter, Paket, Build und CI

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU56
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Das Cheat-Fenster (B-1132) ist eine zweite EXE; das Spiel startet sie nicht. `task build` und `buildall` packen nur `k3c-game.exe` nach `dist/k3c-game/` (RU35), die CI-Job `rust` prüft die EXE-Größe gegen 40 MB. F4 ist in `view/devkeys.rs` frei.

## Ziel

F4 oder der Startschalter `--devwin` öffnet das Cheat-Fenster neben dem Spiel; die EXE liegt im Paket, Build und CI kennen sie.

## Beteiligte und Zielgruppen

Umsetzung Claude (CLI, INF-Anteil Taskfile). Fachprüfung `k3c-hud`, `k3c-reviewer`. Abnahme 🧑.

## Anforderungen

- Neuer Eintrag `DevCmd::Window` auf **F4** in der Tabelle `view/devkeys.rs` (erscheint in der Tastenübersicht F1), Auswertung in `game/devkeys.rs`; im Aufnahme-Modus wirkungslos wie alle Dev-Tasten. Zweites Drücken von F4: Fenster in den Vordergrund (`focus`), nicht zweites Fenster.
- Startschalter `--devwin` und Umgebungsvariable `K3C_DEVWIN=1` öffnen das Fenster beim Start.
- Start des Prozesses mit `std::process::Command` (kein `unsafe`): EXE `k3c-devwin.exe` im Ordner der Spiel-EXE, ohne Konsolenfenster (`creation_flags(CREATE_NO_WINDOW)` aus `std::os::windows::process::CommandExt`), Argumente `--pipe k3c-devwin-<id> --exit-with-game <pid>`; Fehler (EXE fehlt) landen im Log mit Emoji und als Hinweis im Spiel, kein Panic. Nicht Windows: F4 meldet "nur Windows" im Log.
- Build: Task `devwin:exe` in `rust/Taskfile.yml`; `task build` und `buildall` bauen sie und legen `k3c-devwin.exe` neben `k3c-game.exe` nach `dist/k3c-game/`; CI-Job `rust` baut sie, prüft Vorhandensein und Größe (≤ 5 MB, Default) und führt `cargo test -p k3c-devproto -p k3c-devwin` aus. Hinweis auf die Lizenz des `windows`-Crates in B-1121 (Paket-Lizenzen).
- `CLAUDE.md` (Abschnitt Befehle) nennt F4 und `devwin:exe` in einer Zeile.

## Nicht-Ziele

Fensterinhalt (B-1132, B-1134), Entfernen der In-Game-Menüs (B-1136).

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/src/view/devkeys.rs`, `rust/crates/k3c-game/src/game/devkeys.rs`, `rust/crates/k3c-game/src/devwin/spawn.rs`, `rust/crates/k3c-game/src/game/flags.rs`, `rust/Taskfile.yml`, `Taskfile.yml`, `.github/workflows/ci.yml`, `CLAUDE.md`
- **Abhängig von:** B-1132, B-1133
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** taskfile
- Die Firewall-Regel: nur Named Pipe, kein Netz, kein Port; Prozess mit festem Pfad (Gedächtnis firewall-prompt-feste-exe).

## Beispiele

Spiel läuft, F4 gedrückt → Fenster erscheint neben dem Spiel, Statuszeile "verbunden"; Spiel beenden → Fenster schließt sich.

## Ausnahme- und Fehlerfälle

`k3c-devwin.exe` fehlt → Log-Zeile und Hinweis, Spiel läuft normal. F4 im Aufnahme-Modus → nichts.

## Akzeptanzkriterien

- **AC-01** Test: Tabelle enthält F4 mit `DevCmd::Window`, Tastenübersicht zeigt ihn, Aufnahme-Modus ignoriert ihn.
- **AC-02** Test: Start-Argumente (Pipe-Name, Eltern-PID, Pfad neben der Spiel-EXE) werden richtig gebaut; fehlende EXE ergibt Fehler ohne Panic.
- **AC-03** `task build` legt `k3c-devwin.exe` ins Paket; der CI-Job `rust` prüft sie.
- **AC-04** Am PC (🧑): F4 und `--devwin` öffnen das Fenster, Befehle wirken, kein Firewall-Hinweis.

## Offene Fragen

keine

## Notizen

Entschieden mit Default (autonom, Claude, 2026-11-19): Schwelle 5 MB für die Fenster-EXE; F4 statt eigener Menü-Eintrag.
