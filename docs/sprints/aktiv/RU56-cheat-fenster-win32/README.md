# RU56 · CLI · Cheat-Fenster: zweites Win32-Fenster neben dem Rust-Spiel

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-1131, B-1132, B-1133, B-1134, B-1135, B-1136
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19

## Ausgangslage

Das Rust-Spiel hat Cheat- und Debug-Menüs im Bild (`rust/crates/k3c-game/src/devtools/`, F9 Cheats, F10 Debug, RU29): Sie verdecken das Spiel, sind nur mit Pad und Tasten bedienbar und nicht frei anordenbar. Die Dev-Aktionen laufen über `DevSession::dev(player, &DevAction)` (`view/devkeys.rs`, `local/dev.rs`) und die F-Tasten (`game/devkeys.rs`). F4 ist frei. Der Workspace verbietet `unsafe` (`unsafe_code = forbid`), das Spiel läuft unter winit/wgpu (`k3c-platform`). Ralf will ein eigenes zweites Fenster neben dem Spiel mit normalen Win32-Controls (2026-11-19).

## Ziel

Ein zweites, verschiebbares und in der Größe änderbares Fenster `k3c-devwin.exe` im Dark-Look steuert das laufende Rust-Spiel: Welt, Insel und Stufe wählen, Tageszeit, Zeitraffer, Münzen und Ressourcen, Skills, Gegner, Boss, Welle, Anzeigen – mit demselben Funktionsumfang wie die In-Game-Menüs. Am Ende sichtbar: F4 im Spiel öffnet das Fenster, ein Klick löst z. B. "Nacht, Boss, 4×" aus.

## Beteiligte und Zielgruppen

Ralf und Tester, die Szenen gezielt aufbauen. Umsetzung: Claude (CLI). Fachprüfung `k3c-hud`, `k3c-reviewer` (Eingabe-, Pfad- und `unsafe`-Code). Freigabe der Abhängigkeit `windows`: 🧑 (gewünscht, 2026-11-19). Abnahme am PC (Aussehen, Bedienung): 🧑.

## Anforderungen

Siehe `B-1131`, `B-1132`, `B-1133`, `B-1134`, `B-1135 › Anforderungen`. Sprint-eigen:
- **Architektur (entschieden):** eigener Prozess, nicht Thread im Spiel. Gründe: `unsafe` bleibt aus dem Spiel (`k3c-game` bleibt `forbid`), Win32-Nachrichtenschleife unabhängig von winit, Absturz oder Hänger des Fensters stoppt das Spiel nicht, die Alloc-Ratsche im Bild-Pfad sieht nichts. Verbindung nur über eine lokale Named Pipe `\\.\pipe\k3c-devwin-<id>` (Fenster = Server, Spiel = Client über `std::fs`, ohne `unsafe`): kein Netz, kein Port, kein Firewall-Hinweis.
- **Zwei Crates:** `k3c-devproto` (Nachrichten, serde, kein `windows`, `forbid unsafe`) und `k3c-devwin` (Fenster, `windows`-Crate, `unsafe` nur in `src/win32/`).
- **Neue Abhängigkeit (von Ralf gewünscht, 2026-11-19):** `windows` (Microsoft), Version fest gepinnt, `default-features = false`, nur begründete Features; vermerkt in `rust/Cargo.toml` und `docs/standards/rust-port.md` (Abschnitt Abhängigkeiten). Kein GUI-Framework.
- Befehle laufen im Spiel über dieselben Cheat-Pfade wie die devtools-Menüs (`DevSession::dev`, `DevCmd`), mit der Spieler-Rolle Entwickler (RU52.7).
- Die In-Game-Menüs bleiben; ihre Entfernung ist das Nach-Ticket B-1136 (nach der Abnahme).
- RU42-Leitplanken: kein `unwrap`/`expect`/`panic` im Nicht-Test-Code, die Allokations-Obergrenze in `tests/alloc_je_bild.rs` bleibt unverändert grün; ohne Fenster laufen keine Threads und nichts kostet Zeit.

## Nicht-Ziele

Entfernen der In-Game-Menüs (B-1136); Look-Profil-Umschalter (nach RU55); Spielerzahl während des Spiels ändern (nur Anzeige, Folgeticket bei Bedarf); Fenster auf anderen Plattformen; Speichern der Fensterlage; Netzwerkzugriff.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Keine Änderung an Sim, Regeln, Protokoll des Spielstands und Golden. Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen, Sessions je ≤ ca. 150 Zeilen Änderung. Sim, Balance, Gen und `check:race` sind nicht Teil dieses Sprints (kein Sim-Eingriff, kein Abnahme-Ticket nötig). Jeder `unsafe`-Block hat einen `// SAFETY:`-Kommentar; ein Test hält `unsafe` aus dem restlichen Workspace heraus.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-devproto/**`, `rust/crates/k3c-devwin/**`, `rust/crates/k3c-game/src/devwin/**`, `rust/crates/k3c-game/src/view/devkeys.rs`, `rust/crates/k3c-game/src/game/devkeys.rs`, `rust/crates/k3c-game/src/game/flags.rs`, `rust/crates/k3c-game/src/lib.rs`, `rust/crates/k3c-game/Cargo.toml`, `rust/crates/k3c-game/tests/devwin_pipe.rs`, `rust/Cargo.toml`, `rust/Cargo.lock`, `rust/Taskfile.yml`, `Taskfile.yml`, `.github/workflows/ci.yml`, `CLAUDE.md`, `docs/standards/rust-port.md`
- **Abhängig von:** RU52.7, RU54
- **Braucht 🧑:** Abnahme
- **Sperre:** deps, taskfile

Hinweis Kollision: RU52.7 (`dev-session-rolle-im-view`, `devtools/**`, `view/session.rs`) liefert die Dev-Rolle, auf die RU56.5 aufsetzt. RU54 (Smoke-Befunde, `k3c-game/src/game/**`) berührt `game/devkeys.rs` und `game/flags.rs` aus RU56.11, daher läuft RU56 nach RU54. RU55 (Look, `world/**`, `view/cell.rs`) berührt keine dieser Dateien.

## Beispiele

F4 im laufenden Spiel → Fenster erscheint neben dem Spiel, Statuszeile "verbunden". Reiter Welt und Zeit: Welt, Insel, Stufe wählen, "Wechseln", "Nacht", "4×" → das Spiel springt dorthin und läuft mit vierfacher Geschwindigkeit. Reiter Gegner: "Boss" → Boss erscheint.

## Ausnahme- und Fehlerfälle

`k3c-devwin.exe` fehlt → Log-Zeile und Hinweis im Spiel, Spiel läuft normal. Fenster wird geschlossen oder stürzt ab → Spiel unberührt, F4 öffnet es neu. Spiel endet → Fenster schließt sich (`--exit-with-game`). Befehl ohne Dev-Aktion in der Sim (z. B. direkter Stufenwechsel) → `ack` mit `ok:false`, Ticket statt Umgehung. Dark-Theme nicht verfügbar → helle Standard-Controls mit gleichem Layout.

## Akzeptanzkriterien

- **AC-01** Protokoll `k3c-devproto`: Round-Trip aller Nachrichten, Fehlerfälle ohne Panic (B-1131/AC-01 bis AC-03).
- **AC-02** Fenster-Gerüst baut unter Windows, `windows` fest gepinnt und in `rust-port.md` vermerkt, kein `unsafe` außerhalb `k3c-devwin/src/win32/` (B-1132/AC-01, AC-03).
- **AC-03** Layout reflowt je Breite in 1, 2 oder 4 Spalten ohne Überlappung, Scroll-Bereich bei hohem Inhalt (B-1132/AC-02).
- **AC-04** Pipe-Server und Verbindungsanzeige: Nachrichten aus `k3c-devproto` werden empfangen und gesendet, Wiederverbindung funktioniert (B-1132/AC-01).
- **AC-05** Spielseite: Zustandsabzug, Befehle über `DevSession::dev`, `ack`, kein Blockieren, ohne Fenster keine Threads, Alloc-Test grün (B-1133/AC-01 bis AC-04).
- **AC-06** Welt und Zeit: Controls lösen die Cheat-Pfade aus (B-1134/AC-01).
- **AC-07** Wirtschaft (B-1134/AC-02).
- **AC-08** Spieler und Skills (B-1134/AC-03).
- **AC-09** Gegner und Welle (B-1134/AC-04).
- **AC-10** Anzeige (B-1134/AC-05).
- **AC-11** Start per F4 und `--devwin`, Spawn-Argumente, `task build` packt die EXE, CI prüft sie (B-1135/AC-01 bis AC-03).
- **AC-12** Abnahme am PC (🧑): Aussehen und Bedienung, Szene "Nacht, Boss, 4×" in unter einer Minute, kein Firewall-Hinweis (B-1132/AC-04, B-1134/AC-06, B-1135/AC-04).
- **AC-13** Review: `task check:diff`, `task check:go`, `cargo test --workspace` grün, Fachprüfung ohne schwere Befunde.

## Offene Fragen

Pad-Kombination View + Y (`ActionCheats`) nach der Entfernung des F9-Menüs (B-1136): entfernen oder auf das Fenster legen? Nicht blockierend, 🧑 entscheidet bei B-1136; Default entfernen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU56.1 | `RU56.1-protokoll-devproto.md` | Umsetzung | autonom | fertig |
| RU56.2 | `RU56.2-fenster-geruest.md` | Umsetzung | autonom | fertig |
| RU56.3 | `RU56.3-layout-reiter.md` | Umsetzung | autonom | fertig |
| RU56.4 | `RU56.4-pipe-server.md` | Umsetzung | autonom | offen |
| RU56.5 | `RU56.5-spielseite.md` | Umsetzung | autonom | offen |
| RU56.6 | `RU56.6-controls-welt-zeit.md` | Umsetzung | autonom | offen |
| RU56.7 | `RU56.7-controls-wirtschaft.md` | Umsetzung | autonom | offen |
| RU56.8 | `RU56.8-controls-spieler-skills.md` | Umsetzung | autonom | offen |
| RU56.9 | `RU56.9-controls-gegner-welle.md` | Umsetzung | autonom | offen |
| RU56.10 | `RU56.10-controls-anzeige.md` | Umsetzung | autonom | offen |
| RU56.11 | `RU56.11-start-paket-ci.md` | Umsetzung | autonom | offen |
| RU56.12 | `RU56.12-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

–
