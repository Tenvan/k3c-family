# RU58 · INF, CLI, DEV · Go-Referenz sichern, Go-Spiel aus Build und CI nehmen, Lizenzliste des Rust-Spiels

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** INF, CLI, DEV
- **Reife:** bereit
- **Tickets:** B-1124, B-1139
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19 (Ralf: ausdrücklich)

## Ausgangslage

**Vorrang vor RU51/RU52 (Ralf 2026-11-19).** Ralf hat am 2026-11-19 ausdrücklich entschieden: Alle Go-Builds, die nicht mehr gebraucht werden, werden entfernt, einzige Ausnahme ist die Workbench (`tools/k3c-workbench`, Go, samt allem, was sie im Prozess braucht). Alle drei Schritte laufen jetzt, nicht erst nach G4 (RU36): (1) Go-Spiel aus Build, CI und Release nehmen (dieser Sprint, B-1124), (2) Go-Spiel entfernen (RU37), (3) Go-Engine entfernen, soweit die Workbench sie nicht braucht (RU59). Ralf 2026-11-19: alle drei Schritte sofort; die Abhängigkeiten von RU36, RU33 und RU34 entfallen bewusst für diese Sprints.

Das Go-Spiel ist aber das Orakel der offenen Abnahme-Tickets B-1120, B-1129, B-1130 (Bildvergleich Go gegen Rust, Aufnahmen mit der Go-EXE, `e2e_images`, `e2e_compare`, `measure.ps1 -Engine game`). Darum sichert dieser Sprint zuerst die Referenz (Tag `go-referenz-final`, Referenzaufnahmen und Messwerte), bevor irgendetwas gelöscht wird.

Weiterer Befund (B-1139): `k3c-game.exe licenses` und das Lizenz-Menü des Rust-Spiels nennen Go-Bibliotheken (Ebitengine, Oto, go-text …) statt der Rust-Crates.

Stand `origin/main` (c76b80005): `task buildall` und `task build` bauen schon nur das Rust-Spiel (RU35.2); es bleiben der CI-Job `k3c-game` ("Natives Spiel"), der Go-Bezug in `release.yml` und im CI-Job `rust` (`go-version-file: native/k3c-game/go.mod`), `native:game:build` im Namensraum `native`, und der rote Frontend-Test der Workbench.

## Ziel

Die Go-Referenz ist gesichert (Tag, 14 Referenzbilder, Messwerte, Farbwerte), das Go-Spiel steckt in keinem Build, keiner CI und keinem Release mehr, `main` ist grün (B-1124, ohne den Go-arm64-Job, den RU37 entfernt), und die Lizenzliste des Rust-Spiels nennt Rust-Crates.

Am Ende sichtbar: `git show go-referenz-final` baut das Go-Spiel; `testdata/rust/e2e/go/` und `docs/messung/go-referenz.md` liegen im Repo; `gh run list --branch main` zeigt keinen Job `Natives Spiel`; `dist/k3c-game/k3c-game.exe licenses` nennt wgpu, fontdue, cpal usw.

## Beteiligte und Zielgruppen

Second Worker setzt alle Sessions um (Lane `zweit`; GPU und Fenster für die Aufnahmen). Ralf (🧑) entscheidet nichts mehr vor dem Löschen, prüft aber B-1139 im Spiel (Abnahme).

## Anforderungen

- `B-1124 › Anforderungen` (AC-01 Job `Natives Spiel` entfernt, AC-03 Job `k3c-workbench` grün; AC-02 Go-arm64 erledigt RU37.2 durch Entfernen des Jobs).
- `B-1139 › Anforderungen` (Liste aus `cargo metadata`, Texte DE/EN in der Struktur unverändert). Abweichung von B-1139: Die Session läuft jetzt in diesem Sprint (vor dem Löschen); die Go-Goldens der Lizenz-Seite (`testdata/rust/bild/menu/licenses.go.png`) werden aufgegeben und durch eine Rust-eigene Baseline ersetzt (Ralf 2026-11-19: ausdrücklich).
- `B-1120`, `B-1129`, `B-1130`: Deren Go-Seite (Aufnahmen und Messwerte mit der Go-EXE) entsteht einmal in RU58.2 und RU58.3 und liegt danach im Repo; die Rust-Seite der Tickets vergleicht gegen diese Referenz.
- Sprint-eigen: Die Referenz muss ohne Go-EXE nutzbar sein; wer die Go-EXE doch braucht, baut sie aus dem Tag (`docs/standards/go-referenz.md`).

## Nicht-Ziele

Go-Spiel oder `engine/` löschen (RU37, RU59). Rust-Fehler beheben, die der Bildvergleich findet (eigene Tickets im jeweiligen RU-Sprint, vom Hauptrechner angelegt). Sim-, Balance- und Lasttest-Läufe über die Aufnahmen hinaus. Schwellen des Bildvergleichs anheben.

## Regeln und Einschränkungen

**Vorrang:** Alle Sessions dieses Sprints haben `Lane: zweit` und stehen im Projekt RSU vor RU51 und RU52 (Ralf 2026-11-19). Der Second Worker nimmt sie vor jeder anderen Session (Projekt-Tabelle: `docs/projekte/RSU-rust-umstellung.md`, Reihenfolge der Sprints).

Golden werden nie zugunsten von Rust neu geschrieben; einzige Ausnahme in diesem Sprint: die Lizenz-Seite (Text ändert sich absichtlich, Ralf 2026-11-19). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Schreiben nur im Repo (`.work/`, danach löschen). Der Second Worker darf Aufnahme-, Mess- und Gen-Läufe fahren (Beschluss 2026-11-18), nur in Sessions mit `Lane: zweit`. Neue Crates nur mit Freigabe 🧑 (B-1139 nutzt `cargo metadata` nur lesend). Keine Tickets anlegen: Befunde unter `### Für den Hauptrechner`.

Reihenfolge: RU58.1 (Tag) → RU58.2 → RU58.3 (Referenz) → RU58.4, RU58.5 (Schritt 1) → RU58.6 (B-1139) → RU58.7 Review. Das Löschen beginnt erst in RU37, nachdem RU58 gemergt ist.

- **Bereich:** `docs/standards/go-referenz.md`, `docs/messung/go-referenz.md`, `docs/messung/go-referenz.json`, `testdata/rust/e2e/go/**`, `testdata/rust/go-referenz/**`, `rust/crates/k3c-game/tests/e2e_compare.rs`, `rust/crates/k3c-game/tests/e2e_images.rs`, `rust/crates/k3c-game/tests/common/**`, `.github/workflows/ci.yml`, `.github/workflows/release.yml`, `Taskfile.yml`, `native/Taskfile.yml`, `rust/Taskfile.yml`, `tools/k3c-workbench/frontend/**`, `rust/crates/k3c-game/src/menu/licenses.rs`, `rust/crates/k3c-game/src/menu/image_tests.rs`, `tools/rustpaket/**`, `testdata/rust/bild/menu/**`
- **Abhängig von:** – (Ralf 2026-11-19: alle drei Schritte sofort; RU36, RU33, RU34 sind bewusst keine Abhängigkeit)
- **Braucht 🧑:** Abnahme (nur B-1139: Lizenz-Seite im Spiel)
- **Sperre:** taskfile

## Beispiele

Referenz: `go test ./e2e -run TestFlowImages` im Tag-Checkout schreibt 14 Bilder; sie liegen als `testdata/rust/e2e/go/a/shot-NN.png` im Repo, `e2e_compare` liest sie ohne Go-EXE.
CI: Auf `main` fehlt der Job `Natives Spiel`; der Job `rust` nimmt `go.mod` der Wurzel für `tools/sync` und `tools/rustpaket`.
Lizenz-Seite: "Engine: wgpu, naga, winit …" statt "Ebitengine".

## Ausnahme- und Fehlerfälle

Ist die Referenz über etwa 8 MiB groß, liegt sie nicht im Repo: Dann bleiben nur Tag und Messwerte, und RU58.2 trägt im Ergebnis die Begründung (Größe messen, nicht schätzen). Lässt sich die Go-EXE auf dem Second Worker nicht bauen oder aufnehmen, hält die Session an (`blockiert`), nichts wird gelöscht. Rust-Abweichungen beim Vergleich sind Befunde für den Hauptrechner, kein Umbau in diesem Sprint.

## Akzeptanzkriterien

- **AC-01** Der annotierte Tag `go-referenz-final` ist auf `origin` und zeigt auf einen baubaren Stand; `docs/standards/go-referenz.md` beschreibt den Bau aus dem Tag.
- **AC-02** Referenzaufnahmen (14 Stationen, Läufe a und b pixelgleich) und Messwerte (Frame, Start, Speicher je Szene `tag` und `nacht`, Farbwerte, Go-Pause) liegen gesichert im Repo (oder begründet nur im Tag); `e2e_compare` und `e2e_images` brauchen keine Go-EXE mehr.
- **AC-03** (`B-1124/AC-01`) Der CI-Job `Natives Spiel` und alle Go-Spiel-Bezüge in `release.yml`, im CI-Job `rust` und in `task check:all` sind entfernt; `native:game:build` ist in keinem Sammelziel.
- **AC-04** (`B-1124/AC-03`) Der Job `k3c-workbench` ist grün (Vitest `⎇`).
- **AC-05** (`B-1139`) `licenses` und das Lizenz-Menü nennen nur Rust-Abhängigkeiten, die mit den direkten Abhängigkeiten der Spiel-Crates übereinstimmen; die Baseline der Lizenz-Seite ist neu aufgenommen und begründet.

## Offene Fragen

keine. Mit Default entschieden (autonom, Claude, 2026-11-19; Ralf kann widersprechen): Tag heißt `go-referenz-final` (wie Ralf nannte, nicht `go-final` aus dem alten RU37); B-1139 läuft vor dem Löschen; die Referenzbilder liegen bevorzugt im Repo (`testdata/rust/e2e/go/`), nur bei Überschreiten von 8 MiB nicht.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU58.1 | `RU58.1-tag-go-referenz-final.md` | Umsetzung | autonom | fertig |
| RU58.2 | `RU58.2-go-referenzbilder-e2e.md` | Umsetzung | autonom | fertig |
| RU58.3 | `RU58.3-go-referenz-messwerte-farben-pause.md` | Umsetzung | autonom | fertig |
| RU58.4 | `RU58.4-go-spiel-aus-build-ci-release.md` | Umsetzung | autonom | fertig |
| RU58.5 | `RU58.5-workbench-vitest-ci-rot.md` | Umsetzung | autonom | fertig |
| RU58.6 | `RU58.6-lizenzliste-rust-crates.md` | Umsetzung | autonom | fertig |
| RU58.7 | `RU58.7-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-19 Review RU58.7: AC-01 (RU58.1, Tag auf origin), AC-02 (RU58.2/.3, Referenz 3,0 MiB Bilder + 56 KiB Farben/Pause, `e2e_compare` ohne Go-EXE), AC-03 (RU58.4, Job entfernt; `go-version-file` bleibt für `cmd/sync` im Modul `native/k3c-game` bis RU37.1), AC-04 (RU58.5, Vitest 575 grün; CI-Lauf Rust und Workbench grün), AC-05 (RU58.6, Liste mit Cargo-Test, neue Baseline). Keine neuen Tickets; Sim/Balance/Gen: kein Lauf nötig, kein Abnahme-Ticket für diesen Sprint nötig. Nicht gemergt: Sprint-Branch `sprint/ru58` liegt auf origin, Merge macht der Hauptrechner.
