# B-1137 · Das Rust-Paket wird beim Bau nicht auf Vollständigkeit geprüft und hat keinen Start-Hinweis

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
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

`rust:game:dist` kopiert EXE, data, Grafik, Ton, Schrift und Rig-Teile, prüft aber nichts außer `public/grafik`. Die CI prüft nur EXE-Größe, `data` und `public/grafik`. Ein Paket ohne Lizenzdatei, mit Go-Resten (`*.go`, `assets.txt`) oder fehlendem Ton fiele nicht auf. Einen Start-Hinweis für Empfänger gibt es nicht.

## Ziel

Der Bau prüft das Rust-Paket gegen eine feste Liste (Pflichtdateien, verbotene Dateien) und legt eine `README.txt` (Start, Ordner, Spielstände, Lizenzen) ins Paket; die CI nutzt dieselbe Prüfung.

## Beteiligte und Zielgruppen

Entwickler; Empfänger des Pakets (Mitspieler am Sofa).

## Anforderungen

- Go-Werkzeug `tools/rustpaket pruefen <paketordner>`: Pflicht sind `k3c-game.exe` (höchstens 40 MB), `README.txt`, `LICENSE`, `LICENSE-ASSETS`, `LIZENZEN.txt` (nicht leer, mindestens ein Crate-Abschnitt), `data/` mit JSON, `public/grafik/**/index.paper.json`, `public/audio/`, `public/schrift/LICENSE`, `native/k3c-rig/assets/`.
- Verboten sind `*.go`, `go.mod`, `assets.txt`, `*.rs`, `Cargo.*`, `*.pdb`, `*.md` unter `data/`, Ordner `saves/`, `reports/`, `.git`, `.work`, `target`.
- Task `rust:game:dist:check` (auch einzeln aufrufbar); `game:dist` ruft ihn am Ende.
- `README.txt` liegt als Vorlage im Repo (`rust/paket/README.txt`) und wird ins Paket kopiert.

## Nicht-Ziele

Installer, Signierung, Updates, Spielregeln, Rust-Code des Spiels.

## Regeln und Einschränkungen

Neue Abhängigkeit nur, wenn unvermeidbar: keine, Go-Standardbibliothek genügt.

- **Bereich:** `tools/rustpaket/**`, `rust/Taskfile.yml`, `rust/paket/README.txt`, `.github/workflows/ci.yml`
- **Abhängig von:** B-1121
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

`task rust:game:dist:check` -> `✅ Paket vollständig (n Dateien)`; mit einer `foo.go` im Paket: `❌ verbotene Datei: foo.go`, Exit 1.

## Ausnahme- und Fehlerfälle

Paketordner fehlt: klare Meldung, Exit 1.

## Akzeptanzkriterien

- **AC-01** `task rust:game:dist:check` prüft Pflicht- und verbotene Dateien und scheitert mit der Dateiliste.
- **AC-02** `README.txt` im Paket nennt Start, Spielstand-Ordner (`saves/` neben der EXE), Steuerung-Verweis und Lizenzdateien.
- **AC-03** Tests des Werkzeugs decken Pflicht fehlt, verbotene Datei, zu große EXE und das vollständige Paket ab.
- **AC-04** Der CI-Job `rust` ruft die Prüfung statt der handgeschriebenen pwsh-Schritte (EXE-Größe bleibt).

## Offene Fragen

keine (autonom, Claude, 2026-11-19).

## Notizen

Ergänzt B-1121 und B-1122 im Sprint RU57.
