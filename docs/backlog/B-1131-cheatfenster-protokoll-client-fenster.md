# B-1131 · Cheat-Fenster: Protokoll zwischen Spiel und Fenster als eigenes Crate k3c-devproto

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

Das Rust-Spiel hat Cheat- und Debug-Menüs im Bild (`rust/crates/k3c-game/src/devtools/`, F9 und F10, RU29). Sie verdecken das Spiel, sind nur mit Pad oder Tasten bedienbar und lassen sich nicht frei anordnen. Ralf will stattdessen ein eigenes, verschiebbares Fenster neben dem Spiel (Sprint RU56). Dafür braucht es zuerst eine feste Sprache zwischen beiden Prozessen, die ohne Win32 und ohne das Spiel testbar ist.

## Ziel

Ein kleines Crate `k3c-devproto` beschreibt alle Nachrichten zwischen Spiel und Cheat-Fenster als Typen mit Zeilen-JSON-Kodierung; Spiel und Fenster hängen beide daran, keiner kennt den anderen.

## Beteiligte und Zielgruppen

Umsetzung Claude (CLI, Rust). Nutzer des Fensters später Ralf und Tester. Fachprüfung `k3c-hud` (Dev-Menüs).

## Anforderungen

- Neues Crate `rust/crates/k3c-devproto` (kein `windows`, kein Netz, nur `serde` und `serde_json` aus dem Workspace, `unsafe_code = forbid` wie der Workspace).
- **Transport (festgelegt):** Named Pipe `\\.\pipe\k3c-devwin-<id>`, Byte-Strom, eine JSON-Nachricht je Zeile (UTF-8, `\n`), höchstens 64 KiB je Zeile. Das Fenster ist der Server, das Spiel der Client. Das Crate kennt nur Zeilen (`encode_line`, `decode_line`), nicht die Pipe.
- Version `v: 1`; beide Seiten beginnen mit `hello` (`{"t":"hello","v":1}`, vom Spiel zusätzlich `game` mit Name und Version). Gleiche Hauptversion, sonst `ProtoError::Version`.
- **Fenster → Spiel** (jeder Befehl trägt eine `id`, das Spiel antwortet mit `ack`): `dev` (`player`, `action` mit den Feldern der DevAction: `action`, `amount`, `resource`, `factor`, `phase`, `grade`, `on`, `level`, `boss`, `event`, `kind`, `unit`), `ui` (`key`: `debug`, `stats`, `river`, `help`, `pause`, `step`, `speed`, `save`, `load`, `shot`, `fullscreen`; `value` optional), `goto` (`world`, `island`, `stage`), `focus`, `bye`.
- **Spiel → Fenster:** `catalog` (einmal nach `hello`: Welten mit Inseln und Stufen, Bosse, Events, Skills, Ressourcen, Truppenarten, Phasen, Grade, Zeitrafferstufen), `state` (Welt, Insel, Stufe, Phase, Zeitraffer, Halt, Spieler mit Gold und Vorrat, Debug-Seite, Leistungsanzeige, Fluss-Fassung, Takte/s), `ack` (`id`, `ok`, `err`), `bye`.
- Unbekannte Felder werden ignoriert (Vorwärtskompatibilität, `#[serde(default)]`); eine unbekannte Art `t` ergibt `Msg::Unknown`, nie einen Panic.
- `DevCommand` im Crate spiegelt `DevAction` des Spiels (das Spiel hängt von `k3c-devproto` ab, nicht umgekehrt); die Umwandlung `From<DevCommand> for DevAction` steht im Spiel (B-1133).

## Nicht-Ziele

Pipe, Fenster, Spielseite (B-1132, B-1133). Authentifizierung (lokale Pipe, nur gleicher Benutzer).

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-devproto/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –
- RU42-Leitplanken: kein `unwrap`/`expect`/`panic` im Produktivcode. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

`{"t":"dev","id":7,"player":0,"action":{"action":"night"}}` → Spiel setzt die Nacht, antwortet `{"t":"ack","id":7,"ok":true}`.

## Ausnahme- und Fehlerfälle

Zeile > 64 KiB oder kein gültiges JSON → `ProtoError::Zeile` (Verbindung bleibt bestehen, Zeile wird verworfen). Fehlende Pflichtfelder → `ProtoError::Feld`. Falsche Version → `ProtoError::Version`.

## Akzeptanzkriterien

- **AC-01** Round-Trip-Test je Nachricht (`encode_line` → `decode_line` gleich), mindestens `hello`, `dev`, `ui`, `goto`, `catalog`, `state`, `ack`.
- **AC-02** Tests: unbekanntes Feld wird ignoriert, unbekanntes `t` ergibt `Unknown`, Zeile über 64 KiB und Versionskonflikt ergeben Fehler ohne Panic.
- **AC-03** `cargo test -p k3c-devproto`, `task rust:lint` grün; das Crate hängt nur an `serde`/`serde_json`.

## Offene Fragen

keine

## Notizen

Entschieden mit Default (autonom, Claude, 2026-11-19): Zeilen-JSON statt Binärformat (lesbar im Log, serde schon im Stack); Named Pipe statt Loopback-TCP (kein Firewall-Prompt, kein Port).
