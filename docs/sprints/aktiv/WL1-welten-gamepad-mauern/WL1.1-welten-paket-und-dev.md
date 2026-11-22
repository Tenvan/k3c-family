# WL1.1 · Welten im dist-Paket, Repo-Vorrang mit Dev-Symbol

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wl1/1-welten-paket-und-dev
- **Abhängig von:** –
- **Tickets:** B-1210
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Das Paket `dist/k3c-game` enthält alle Welten und die Paketprüfung verlangt sie; startet `k3c-game.exe` aus `<repo>/dist/k3c-game`, gelten die Welten aus `<repo>/data/worlds`, in der Welt-Auswahl kenntlich an einem kleinen Dev-Symbol (B-1210 AC-01 bis AC-03).

## Kontext

- `rust/Taskfile.yml` › `game:dist` kopiert schon den ganzen Ordner `data/` (samt `worlds/`) nach `dist/k3c-game/data`; es fehlt nur die Prüfung in `tools/rustpaket/pruefen.go` (Pflichtordner `data/worlds` mit `*.json`).
- `rust/crates/k3c-game/src/local/worlds.rs`: `worlds_fs`/`pick_worlds_dir` wählen den Welten-Ordner (`K3C_WORLDS_DIR`, sonst `data/worlds` neben der EXE, sonst im Arbeitsordner); `worlds_source` macht daraus die Quelle des Managers. Weitere Welten zeigt der Manager nur im Dev-Mode.
- Repo-Erkennung über den Exe-Pfad: zwei Ebenen über dem EXE-Ordner liegt `Taskfile.yml` (und `.git`), EXE-Ordner heißt `k3c-game` unter `dist`. Keine Umgebungsvariable. Repo-Welten unlesbar: Log-Hinweis, Rückfall auf die Paket-Welten.
- Welt-Auswahl: `rust/crates/k3c-game/src/menu/worldpick.rs` (Seite), `menu/hero.rs` (Weltbild), `menu/draw.rs` (`Hero`). Der Eintrag `WorldInfo` steht in `view/session.rs` und bekommt das Kennzeichen `repo`.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/local/**`
- `rust/crates/k3c-game/src/menu/**`
- `rust/crates/k3c-game/src/view/session.rs`
- `rust/crates/k3c-game/src/game/testutil.rs`
- `rust/crates/k3c-game/src/hud/testutil.rs`
- `rust/Taskfile.yml`
- `tools/rustpaket/**`

## Nicht-Ziele

Andere Daten als Welten aus dem Repo lesen; Workbench; neue Grafik-Assets oder `gen:paper`; Dev-Mode-Regel der Weltliste ändern.

## Schritte

1. Test der Paketprüfung mit Welten (`tools/rustpaket/pruefen_test.go`), dann Pflichtordner `data/worlds` in `pruefen.go`.
2. Test der Pfadwahl Repo vs. Paket in `local/worlds_tests.rs`, dann `pick_worlds` mit Repo-Erkennung und Rückfall in `local/worlds.rs`.
3. `WorldInfo.repo` setzen (`local/saves.rs`), Test in `menu/worldpick_tests.rs`, Dev-Symbol im Weltbild (`menu/hero.rs`).
4. cargo test, fmt, clippy, go test `tools/rustpaket`, `task check:diff`.

## Fertig, wenn

- [ ] AC-01: `go test ./tools/rustpaket` grün, ein Paket ohne `data/worlds/*.json` meldet einen Mangel.
- [ ] AC-02: `cargo test -p k3c-game worlds` grün: EXE in `<repo>/dist/k3c-game` liest `<repo>/data/worlds`, sonst das Paket; unlesbare Repo-Welten fallen auf das Paket zurück.
- [ ] AC-03: Test, dass die Welt-Auswahl bei Repo-Welten das Dev-Kennzeichen setzt; Sicht am PC durch 🧑.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
