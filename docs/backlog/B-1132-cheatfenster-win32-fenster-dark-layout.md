# B-1132 · Cheat-Fenster: Win32-Fenster mit Dark-Look, Layout und Pipe-Server als Crate k3c-devwin

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

Es gibt kein Fenster außerhalb des Spiels. Das Rust-Spiel läuft unter winit/wgpu (`k3c-platform`); der Workspace verbietet `unsafe` (`unsafe_code = forbid`). Win32-Controls brauchen `unsafe`, deshalb gehört das Fenster in ein eigenes Programm.

## Ziel

`k3c-devwin.exe` öffnet ein verschiebbares, in der Größe änderbares Fenster mit normalen Win32-Controls im Dark-Look, ordentlicher Schrift und Abständen, das sich über die Pipe mit dem Spiel verbindet und dessen Zustand anzeigt.

## Beteiligte und Zielgruppen

Umsetzung Claude (CLI). Nutzer: Ralf und Tester am PC. Fachprüfung `k3c-hud`. Abnahme 🧑 (Aussehen).

## Anforderungen

- **Prozess statt Thread (festgelegt):** eigenes Programm. Gründe: `unsafe` bleibt aus dem Spiel (`k3c-game` bleibt `forbid`); eigene Nachrichtenschleife ohne Berührung mit winit; ein Absturz oder Hänger des Fensters stoppt das Spiel nicht; die Alloc-Ratsche im Bild-Pfad sieht das Fenster nicht. Kosten: eine zweite EXE im Paket (B-1135).
- Neues Crate `rust/crates/k3c-devwin` (Binary `k3c-devwin`), nur Windows (`#[cfg(windows)]`; sonst ein `main`, das "nur Windows" meldet und endet). Die reine Logik (Layout, Zustand) liegt in `lib`-Modulen ohne Win32 und ist überall testbar.
- **Neue Abhängigkeit (von Ralf gewünscht, 2026-11-19):** Crate `windows` von Microsoft, Version fest (`=`, die aktuelle stabile Version, mindestens zwei Wochen alt, in der Session aus crates.io bestimmt), `default-features = false`, nur die nötigen Features (Kandidaten: `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Graphics_Dwm`, `Win32_UI_WindowsAndMessaging`, `Win32_UI_Controls`, `Win32_UI_HiDpi`, `Win32_UI_Input_KeyboardAndMouse`, `Win32_System_LibraryLoader`, `Win32_System_Pipes`, `Win32_Storage_FileSystem`, `Win32_System_IO`, `Win32_Security`; jedes Feature in der Session begründet). Eintrag in `rust/Cargo.toml` (Workspace-Abhängigkeiten, Kommentar mit Datum und Freigabe) und im Abschnitt Abhängigkeiten von `docs/standards/rust-port.md`.
- **unsafe:** `k3c-devwin` bekommt einen eigenen `[lints]`-Block (die clippy-Lints des Workspace, `unsafe_code = "deny"`) statt `workspace = true`; `unsafe` nur im Modul `src/win32/` (`#![allow(unsafe_code)]`, je Block ein `// SAFETY:`-Kommentar). Ein Test prüft, dass kein `unsafe` außerhalb von `k3c-devwin/src/win32/` im Rust-Workspace steht.
- **Fenster:** `WS_OVERLAPPEDWINDOW` (verschiebbar, Größe änderbar, Maximieren), Mindestgröße 420 × 480 dip, Startgröße 520 × 720 dip, Titel "K3C Cheat-Fenster"; Per-Monitor-DPI (v2), alle Maße in dip und skaliert; Position und Größe werden nicht gespeichert (Default).
- **Dark-Look (Default; Aussehen der Controls ungeprüft, daher Abnahme 🧑):** `DwmSetWindowAttribute(DWMWA_USE_IMMERSIVE_DARK_MODE)` für die Titelleiste; Hintergrund `#1E1F22`, Text `#DCDDDE`, Akzent `#4F8CFF`; Edit, Combobox, Listbox, Scrollbars und Tab über `SetWindowTheme("DarkMode_Explorer")` bzw. `DarkMode_CFD` und `WM_CTLCOLOR*`; Schaltflächen und Umschalter als `BS_OWNERDRAW` (flach, Hover- und Gedrückt-Zustand), weil die Standardschaltfläche kein Dark-Theme hat. Schrift Segoe UI aus `NONCLIENTMETRICS` (9 pt, DPI-skaliert), Raster 8 dip, Gruppen mit Titel.
- **Layout:** reine, getestete Funktion in `src/layout.rs` (`layout(breite, höhe, knoten) -> Rechtecke`): Gruppen fließen in so viele Spalten, wie die Breite zulässt (Schaltfläche mindestens 120 dip breit); ist der Inhalt höher als das Fenster, scrollt die Seite senkrecht (Mausrad, Scrollbar). Fünf Reiter (`SysTabControl32`): Welt und Zeit, Wirtschaft, Spieler und Skills, Gegner und Welle, Anzeige; die Seiten liegen in `src/pages/` je eine Datei, die Registry in `pages/mod.rs`.
- **Pipe-Server:** erzeugt `\\.\pipe\k3c-devwin-<id>` (Name aus `--pipe <name>`), `PIPE_ACCESS_DUPLEX`, Byte-Modus, `PIPE_REJECT_REMOTE_CLIENTS`, eine Instanz; Lese-Thread schickt Nachrichten per `PostMessage` an die Schleife; Verbindungsanzeige in der Statuszeile (wartet, verbunden, getrennt); Wiederverbindung, wenn das Spiel neu startet. Mit `--exit-with-game <pid>` schließt sich das Fenster, wenn das Spiel endet.
- Nachrichten: nur die aus `k3c-devproto` (B-1131).

## Nicht-Ziele

Befehle je Bereich (B-1134), Spielseite (B-1133), Start aus dem Spiel (B-1135). Ein GUI-Framework (egui, iced, Slint): ausdrücklich nicht gewollt. Speichern der Fensterlage.

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-devwin/**`, `rust/Cargo.toml`, `rust/Cargo.lock`, `docs/standards/rust-port.md`
- **Abhängig von:** B-1131
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** deps
- RU42-Leitplanken (kein `unwrap`/`expect`/`panic` im Produktivcode, Fehler weiterreichen), Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Drei Sessions (RU56.2 Gerüst, RU56.3 Layout, RU56.4 Pipe-Server), je ≤ ca. 150 Zeilen Änderung.

## Beispiele

Fenster auf 1920 × 1080 breit gezogen: Schaltflächen stehen in vier Spalten; auf 420 dip schmal gezogen: eine Spalte mit Scrollbar.

## Ausnahme- und Fehlerfälle

Pipe-Name belegt oder Anlegen scheitert → Fehlermeldung im Fenster und Exit-Code ≠ 0, kein Panic. Dark-Theme nicht verfügbar (ältere Windows-Version) → helle Standard-Controls, ordentliches Layout bleibt. Spiel nicht verbunden → alle Controls ausgegraut.

## Akzeptanzkriterien

- **AC-01** `cargo build -p k3c-devwin` baut unter Windows; `cargo test -p k3c-devwin` (Layout, Reiter-Registry, Test "kein unsafe außerhalb win32/") und `task rust:lint` grün.
- **AC-02** Layout-Test: gleiche Gruppen ergeben bei 420, 720 und 1600 dip Breite 1, 2 und 4 Spalten ohne Überlappung; zu hoher Inhalt liefert einen Scroll-Bereich.
- **AC-03** `windows` ist fest gepinnt, nur mit den begründeten Features eingetragen und in `docs/standards/rust-port.md` vermerkt.
- **AC-04** Am PC (🧑): Fenster lässt sich verschieben und in der Größe ändern, Dark-Look mit lesbarer Schrift und gleichmäßigen Abständen, Statuszeile zeigt die Verbindung.

## Offene Fragen

keine

## Notizen

Entschieden mit Default (autonom, Claude, 2026-11-19; 🧑 kann widersprechen): eigener Prozess, zwei Crates (`k3c-devproto` ohne `windows` bleibt `forbid unsafe`, `k3c-devwin` mit begrenztem `unsafe`), keine Fensterlage speichern. Bekanntes Risiko (ungeprüft): Das Dark-Theme einzelner Standard-Controls (Combobox, Edit) hängt von der Windows-Version ab; die Abnahme AC-04 entscheidet, ob weitere Controls owner-draw werden.
