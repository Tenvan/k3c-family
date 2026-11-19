# B-1133 · Cheat-Fenster: Spielseite mit Pipe-Client, Zustandsabzug und Befehlen über die Cheat-Pfade

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

Die Dev-Aktionen des Spiels laufen über `DevSession::dev(player, &DevAction)` (`rust/crates/k3c-game/src/view/devkeys.rs`, Umsetzung `local/dev.rs`), die F-Tasten über `game/devkeys.rs`; das Cheat-Menü F9 (`devtools/cheatlist.rs`) ruft nur diese Pfade. Eine Verbindung nach außen gibt es nicht.

## Ziel

Das Spiel kann sich mit dem Cheat-Fenster verbinden, schickt ihm seinen Zustand und führt seine Befehle über dieselben Cheat-Pfade aus wie F9, ohne `unsafe`, ohne neue Abhängigkeit außer `k3c-devproto`.

## Beteiligte und Zielgruppen

Umsetzung Claude (CLI). Fachprüfung `k3c-hud`, `k3c-reviewer` (Eingabe- und Pfad-Code).

## Anforderungen

- Neues Modul `rust/crates/k3c-game/src/devwin/` (`mod.rs`, `link.rs`, `state.rs`, `commands.rs` und je Bereich eine Datei `world.rs`, `eco.rs`, `players.rs`, `foes.rs`, `view.rs`, in denen die Befehle der Bereichs-Sessions landen; `commands.rs` leitet nach Bereich weiter).
- **Pipe-Client ohne `unsafe`:** `std::fs::OpenOptions` (lesen und schreiben) auf `\\.\pipe\k3c-devwin-<id>`; ein Lese-Thread (Zeilen, `k3c-devproto`) und ein Schreib-Thread mit begrenzter Warteschlange (`try_send`; `state` darf verworfen werden, `ack` nie). Keine Schleife im Bild-Pfad, kein Blockieren des Spiels. Auf anderen Plattformen ist das Modul ein leerer Rumpf.
- **Zustandsabzug** in `update` (nicht im Bild-Pfad), höchstens 4 Hz und bei Änderung; nur Lesen von `Frame`, `DevView` und Session. Der Katalog (`catalog`) geht einmal nach `hello`.
- **Befehle über die Cheat-Pfade:** `dev` → `From<DevCommand> for DevAction` → `DevSession::dev(player, …)`; `ui` → dieselben Funktionen wie die F-Tasten (`DevCmd`); `goto` → vorhandener Insel- und Stufenwechsel der Session; fehlt ein Pfad (z. B. direkter Welt- oder Stufenwechsel), antwortet das Spiel `ack` mit `ok:false` und `err` und es entsteht ein Ticket, nichts wird an der Sim vorbei erzwungen. Jeder Befehl wird mit `ack` beantwortet.
- **Rolle Entwickler:** Befehle laufen mit der Dev-Rolle der Session (RU52.7); `player` wählt den Zielspieler (0 bis Spielerzahl − 1, sonst `ok:false`).
- **Aus ist aus:** Ist das Fenster nicht gestartet (Standard), gibt es keinen Thread und keine Allokation; `tests/alloc_je_bild.rs` bleibt unverändert grün, die Obergrenze wird nicht gelockert.
- Pipe weg oder Zeile defekt: Verbindung gilt als getrennt, das Spiel läuft weiter, Wiederverbinden alle 2 s nur solange das Fenster gewollt ist; kein `unwrap`/`expect`/`panic`.

## Nicht-Ziele

Fenster und Controls (B-1132, B-1134), Start und Paket (B-1135), Änderung der Sim oder neuer Dev-Aktionen in `k3c-sim`.

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/src/devwin/**`, `rust/crates/k3c-game/src/lib.rs`, `rust/crates/k3c-game/Cargo.toml`, `rust/crates/k3c-game/tests/devwin_pipe.rs`
- **Abhängig von:** B-1131, RU52.7
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –
- Kein Eingriff in `devtools/**` (das F9-Menü bleibt unverändert), `k3c-sim`, `k3c-platform`. RU42-Leitplanken. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

Fenster schickt `dev` mit `action:"night"` → Spiel führt dieselbe Aktion aus wie der Eintrag "Nacht sofort" im F9-Menü, antwortet `ack ok`, der nächste `state` zeigt Phase Nacht.

## Ausnahme- und Fehlerfälle

Unbekannte Aktion → `ack` mit `ok:false` und der Fehlermeldung der Session. Spieler außerhalb des Bereichs → `ok:false`. Fenster-Prozess beendet → `state` wird nicht mehr gesendet, Spiel unberührt.

## Akzeptanzkriterien

- **AC-01** Integrationstest `devwin_pipe` (Plattform-Mock oder Datei-Paar statt Pipe): `hello` und `catalog` gehen hinaus, ein `dev`-Befehl erreicht `DevSession::dev` mit denselben Feldern wie der Eintrag im F9-Menü und wird mit `ack` beantwortet.
- **AC-02** Test: Zustandsabzug höchstens 4 Hz, `state` verworfen bei voller Warteschlange, `ack` nie.
- **AC-03** Test: getrennte Pipe und defekte Zeile führen weder zu Panic noch zu Blockieren; ohne Fenster laufen keine Threads.
- **AC-04** `cargo test -p k3c-game --test alloc_je_bild` (Obergrenze unverändert), `task rust:lint` grün.

## Offene Fragen

keine

## Notizen

Entschieden mit Default (autonom, Claude, 2026-11-19): Spiel ist Pipe-Client (std, kein `unsafe`), Fenster ist Server. Direkter Welt- oder Stufenwechsel nur über vorhandene Session-Pfade; fehlt er, Ticket statt Umgehung.
