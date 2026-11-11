# NG1 · CLI, INF · Natives Spiel: Fundament mit Modul, Vertrag, Simulation im Prozess und Spielschleife

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** CLI, INF
- **Reife:** bereit
- **Tickets:** B-792, B-793, B-794
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Prototypen `native/k3c-ebiten` (Level, Fluss, `fern` fest, `--art rig|sheet`) und `native/k3c-rig` (Rig-Format, Pakete `rig`, `paint`) aus NV2 und NV3. Ein natives Spiel mit Simulation, Menüs und Spielständen gibt es nicht. Einzelheiten: B-792, B-793, B-794.

## Ziel

Das Modul `native/k3c-game` steht mit Vertrag, Simulation im Prozess und Spielschleife, sodass NG2–NG8 parallel je ein Paket füllen können. Am Ende sichtbar: `task native:game` zeigt 1–2 Zellen mit Platzhaltern an den Positionen der echten Simulation und großer FPS-Anzeige; `task check:diff` prüft das Modul.

## Beteiligte und Zielgruppen

Claude (Umsetzung; NG2–NG8 bauen darauf), 🧑 Ralf (Freigabe, Entscheidungen zu Ort, Abhängigkeiten, Schrift, Speicherort).

## Anforderungen

`B-792 › Anforderungen`, `B-793 › Anforderungen`, `B-794 › Anforderungen`. Sprint-eigen: Der Vertrag in `view` ist nach dem Review eingefroren; NG2–NG8 ändern ihn nicht, sondern legen Tickets an.

## Nicht-Ziele

Fertige Darstellung der Bereiche (NG2–NG8); Shader (NG11, B-790); Rust entfernen (NG10, B-779); Änderungen unter `engine/`; Online-Spiel und Browser.

## Regeln und Einschränkungen

Entscheidung 005 und 008, Rig-Format `native/k3c-rig/RIG.md`, Komplexitäts-Budget. Keine Simulations-, Balancing- und Lasttest-Läufe (Sperre 2026-11-11); Go-Tests, die einige hundert Takte über `engine/room` laufen, sind Unit-Tests. Alle `go.mod`-Änderungen des Projekts für Text und Ton geschehen hier (Sperre `deps`), damit NG2–NG8 ohne Sperre parallel laufen.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/**`, `native/Taskfile.yml`, `tools/checkdiff/**`, `.github/workflows/ci.yml`
- **Abhängig von:** NV3
- **Braucht 🧑:** Freigabe, Entscheidung
- **Sperre:** deps, taskfile

## Beispiele

`B-792 › Beispiele`, `B-793 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-792 › Ausnahme- und Fehlerfälle`, `B-793 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Modul baut ohne cgo, Ebitengine gepinnt, `replace` auf `k3c` und `k3c-rig` (`B-792/AC-01`).
- **AC-02** Vertrag `view` mit Platzhaltern je Fachpaket und Test der Importregel (`B-792/AC-02`).
- **AC-03** Simulation im Prozess: gleicher Welt-Hash wie `engine/room` nach 300 Takten mit 2 Spielern (`B-793/AC-01`).
- **AC-04** Speichern, Liste, Laden, Löschen gegen Temp-Ordner (`B-793/AC-02`).
- **AC-05** Nichts unter `engine/` und `testdata/golden/` geändert (`B-793/AC-03`).
- **AC-06** Grafik-Bibliothek `art` mit Sheets je Level, Rig-Bibliothek, Schrift, Meldung bei fehlender Grafik (`B-792/AC-03`).
- **AC-07** Fenster mit 1–2 Zellen, Platzhaltern und FPS-Anzeige, Lage offline getestet (`B-792/AC-04`).
- **AC-08** `check:diff` prüft das Modul (`B-794/AC-01`).
- **AC-09** CI-Schritt unter Windows (`B-794/AC-02`).
- **AC-10** Die Begriffe „Natives Spiel“, „Frame“ (nativ), „Zelle“ und „Zeichner“ stehen im Glossar (letzter Commit des Sprints).

## Offene Fragen

Gesammelt aus den Tickets, je mit Vorschlag; entscheidet 🧑 vor der Freigabe:

- Ort `native/k3c-game` als eigenes Modul statt Ausbau des Prototyps oder `cmd/k3c-native` (`B-792 › Offene Fragen`). Vorschlag: eigenes Modul.
- Zusätzliche Module, die Ebitengine für Text und Ton nachzieht (`B-792`). Vorschlag: zulassen, keine weiteren.
- Schrift (`B-792`). Vorschlag: freie OFL-Schrift über `assets_*`.
- Speicherort der Spielstände (`B-793`). Vorschlag: `saves/` neben der EXE, Schalter `--saves`.
- Takt 30 Hz ohne Zwischenbilder (`B-793`). Vorschlag: erst ohne, Ticket bei sichtbarem Ruckeln.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG1.1 | `NG1.1-modul-vertrag.md` | Umsetzung | autonom | offen |
| NG1.2 | `NG1.2-simulation-im-prozess.md` | Umsetzung | autonom | offen |
| NG1.3 | `NG1.3-grafik-bibliothek.md` | Umsetzung | autonom | offen |
| NG1.4 | `NG1.4-spielschleife-zellen.md` | Umsetzung | autonom | offen |
| NG1.5 | `NG1.5-pruefkette.md` | Umsetzung | autonom | offen |
| NG1.6 | `NG1.6-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
