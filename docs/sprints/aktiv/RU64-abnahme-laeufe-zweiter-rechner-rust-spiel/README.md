# RU64 · CLI, INF · Abnahme-Läufe auf dem zweiten Rechner: Rust-Spiel (Bild, Paket, Look, Frame-Zeit)

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI, INF
- **Reife:** bereit
- **Tickets:** B-1120, B-1123, B-1126, B-1129, B-1130, B-1141
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19

## Ausgangslage

Mehrere Rust-Sprints (RU32, RU35, RU53, RU54, RU55, RU57) sind ohne Aufnahme-, Bild-, Gen-, Race- und Messläufe abgeschlossen worden, weil der Hauptrechner gesperrt ist. Die Abnahme-Tickets tragen Lane zweit. RU51 deckt die älteren Sim-Abnahmen ab; hier folgen die Bild-, Paket- und Look-Abnahmen. Die Go-Referenz liegt seit RU58 im Repo (Tag `go-referenz-final`, `testdata/rust/e2e/go/`, `testdata/rust/go-referenz/`, `docs/messung/go-referenz.json`); das Go-Spiel baut nicht mehr, Vergleiche laufen gegen diese Dateien.

## Ziel

Die Abnahme-Tickets B-1120, B-1123, B-1126, B-1129, B-1130 und der Rechner-Teil von B-1141 sind auf dem Second Worker gefahren, belegt und für Ralf zusammengefasst; Rotes steht im Ergebnis, nichts wird nebenbei behoben.

Am Ende sichtbar: je Ticket eine Ergebnisdatei `docs/messung/abnahme-<ticket>.md` (B-1120 zusätzlich die Tabelle in `rust-bildvergleich.md`, B-1130 den Bericht `look-k2c.md`) mit Befehl, Messwert, Dauer und dem Abschnitt „Für den Hauptrechner“.

## Beteiligte und Zielgruppen

Second Worker fährt die Sessions; 🧑 liest die Ergebnisse und führt manuelle Abnahmen (Pad, TV, Mini-PC, Doppelklick, Ton, Weite 4/4,5/5 M) selbst.

## Anforderungen

Siehe die Ticket-Specs B-1120, B-1123, B-1126, B-1129, B-1130, B-1141 › Anforderungen. Sprint-eigen: Läufe nur über `check_run`, Cargo/Go-Tests blockierend gewartet, Messen statt raten (Wiederholungen, Median, Streuung, schlechtester Frame). Vergleiche gegen Go ausschließlich gegen die Referenz-Dateien. Keine Code-Änderungen, kein Golden-Neuschreiben, keine Tickets anlegen.

## Nicht-Ziele

Fixes roter Läufe (kommen als Ticket vom Hauptrechner), Optimierung (RU43), Gate-Messung B-1074 (nur auf Ralfs Freigabe), manuelle Abnahmen am Pad oder TV, Messungen am Mini-PC, Doppelklick-Start und Ton (B-1141, Ralf).

## Regeln und Einschränkungen

Sim-Sperre gilt nur für den Hauptrechner (🧑 2026-11-18); `sim_test` läuft nur, wenn 🧑 `K3C_SIM_ENTSPERRT=1` freigegeben hat, sonst steht im Ergebnis „nicht gefahren: Sperre“. Schreiben nur im Repo (`.work/`), Aufnahmen danach löschen, Bilder nicht einchecken. `check:race` läuft einmal in RU64.2; RU64.3 und RU64.6 zitieren das Ergebnis, solange der Stand (Commit) gleich ist. Reihenfolge: RU64.1 vor RU64.4 (B-1129 AC-03 Abstimmung), RU64.2 vor RU64.7 (Paket), RU64.5 vor RU64.6 (gemeinsame Ergebnisdatei).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile:

- **Bereich:** `docs/messung/abnahme-b-1120.md`, `docs/messung/rust-bildvergleich.md`, `docs/messung/abnahme-b-1123.md`, `docs/messung/abnahme-b-1126.md`, `docs/messung/abnahme-b-1129.md`, `docs/messung/abnahme-b-1130.md`, `docs/messung/look-k2c.md`, `docs/messung/abnahme-b-1141.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

RU64.1 fährt die Rust-Aufnahmen (14 Stationen, Läufe a und b) und `e2e_compare` gegen `testdata/rust/e2e/go/a/` und trägt je Station Mittel, Anteil und Ergebnis in `docs/messung/rust-bildvergleich.md` und `docs/messung/abnahme-b-1120.md` ein.

## Ausnahme- und Fehlerfälle

Ein Lauf ist rot: Ausgabe und Vermutung („ungeprüft“) ins Ergebnis unter „Für den Hauptrechner“, Session trotzdem `fertig`, Ticket bleibt offen. Ein Lauf braucht Gerät, Mini-PC oder Fenster am Pad: in die Prüfliste für Ralf. Fehlt die gerenderte Grafik (`task gen:paper`), erst rendern; `K3C_STILL_REQUIRE=1` macht stilles Überspringen zum Fehler.

## Akzeptanzkriterien

- **AC-01** B-1120: 14 Rust-Stationen (a und b) aufgenommen und gegen die Referenz verglichen, Tabelle in `rust-bildvergleich.md`, `testdata/` unverändert.
- **AC-02** B-1123: Gen, Race und `task buildall` samt Rust-Paket belegt; Sim und Balance belegt oder mit Grund (Sperre) verschoben.
- **AC-03** B-1126: `e2e_flow` ohne Ausnahmeliste und `cargo test --workspace` grün oder rot belegt.
- **AC-04** B-1129: Farbwerte und Pause im Fenster gegen `testdata/rust/go-referenz/` gemessen.
- **AC-05** B-1130: Szenen-Aufnahmen, Pixelverhältnisse, Bildvergleich im Profil `go`, schlechtester Frame, `alloc_je_bild` belegt in `look-k2c.md` und `abnahme-b-1130.md`.
- **AC-06** B-1141: Paket im fremden Ordner aus der Konsole gestartet, Befund in `abnahme-b-1141.md`; Ralf-Teile in der Prüfliste.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU64.1 | `RU64.1-abnahme-b1120-bildvergleich-rust.md` | Umsetzung | autonom | fertig |
| RU64.2 | `RU64.2-abnahme-b1123-paket-ci-schwere-laeufe.md` | Umsetzung | autonom | fertig |
| RU64.3 | `RU64.3-abnahme-b1126-skills-inselwahl.md` | Umsetzung | autonom | offen |
| RU64.4 | `RU64.4-abnahme-b1129-farbe-pause-fenster.md` | Umsetzung | autonom | offen |
| RU64.5 | `RU64.5-abnahme-b1130-szenen-pixelmessung.md` | Umsetzung | autonom | offen |
| RU64.6 | `RU64.6-abnahme-b1130-frame-zeit-alloc.md` | Umsetzung | autonom | offen |
| RU64.7 | `RU64.7-abnahme-b1141-paket-zweiter-rechner.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
