# RU24 · SRV · Raum Teil 2 und Spielstand-Format

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** SRV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Nach RU21 fehlen Stufenwechsel, Inselwechsel, Zeitraffer und der Rest des Raums.

## Ziel

Der Raum ist vollständig; Speichern und Laden ergibt denselben Welt-Hash. Am Ende sichtbar: alle Tests aus `engine/room` und `engine/store` portiert und grün.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 14 von 26, 5 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018) und das Zustandsmodell `docs/standards/rust-sim-zustand.md`. Modell-Empfehlung: `sonnet`, Session 2 `opus` (Speicher-Schreiber, Spielstand-Format).

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/room/` → `rust/crates/k3c-room/src/room/`; Dateinamen nach Leitfaden § 2. Die Sessions laufen als Kette auf dem Branch `sprint/ru24`. Gegen das Repo geprüft (2026-11-17): alle Dateien und Zeilenzahlen stimmen. Tests ohne eigene Quelldatei gehören zur passenden Session (`dev_castlefall`, `dev_grade`, `dev_timescale`, `island`, `island_etappe`, `events`, `depths` zu Session 1; `autosave`, `save`, `gameover_save` zu Session 2; `preview_chunkcount`, `preview_planned`, `preview_ruin` zu Session 3; `worlds_values` zu Session 4). `stages_bench_test.go`: nur `TestStufenEreignisBudgetJeGeraet`, die Benchmark-Funktion entfällt (RU39 misst).

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `engine/room/`: stages (115, T), island_switch (34, T), pause (79, T), dev (229, T) | 457 |
| 2 | `engine/room/`: dev_combat (45, T), fullsave (126, T), saver (162, T) | 333 |
| 3 | `engine/room/`: closeroom (38), deletesave (19, T), skills (64, T), preview (155, T), preview_candidate (43, T), preview_chunk (94, T) | 413 |
| 4 | `engine/room/`: worlds (107, T), metrics (199, T), observer (94) | 400 |
| 5 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 1603 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Protokoll (`docs/protocol.md`) und Spielstand-Format (Version 9) bleiben unverändert; keine neuen Fixtures unter `testdata/`. Keine neue Abhängigkeit außer vorhandenen Workspace-Crates.

- **Bereich:** `rust/crates/k3c-room/**`
- **Abhängig von:** RU21, RU22
- **Braucht 🧑:** –
- **Sperre:** spielstand

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Stufen- und Inselwechsel verhalten sich wie in Go.
- **AC-02** Zeitraffer und Pause ändern das Ergebnis nicht.
- **AC-03** Nach Speichern und Laden ist der Welt-Hash gleich; das Format bleibt Version 9.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Spielstand-Format:** unverändert Version 9; Rust liest die Versionen 1 bis 9 (Migration in RU10) und schreibt Version 9 wie Go (B-1014: Port ohne Formatänderung). Die Memory-Regel „keine Rücksicht auf alte Spielstände“ bleibt ohne Wirkung. Ein neues Format wäre eine eigene Entscheidung von 🧑 nach RU22.
- **Welt-Hash (AC-03):** Vergleich des JSON-Baums aller Stufen vor dem Speichern und nach dem Laden über `k3c_det::golden`; kein neuer Hash und keine neue Golden-Datei.
- **Hintergrund-Speichern:** `trait SaveWriter` mit `InlineWriter` (synchron, gleiche Zustandsmaschine wie Gos Schreiber); ein Thread-Schreiber gehört in den Aufrufer (RU27). **Risiko (vermutet, ungeprüft):** Mit `InlineWriter` blockiert ein Spielstand den Tick; RU39 misst, vorher soll das Spiel den Thread-Schreiber nutzen.
- **Benchmark:** `BenchmarkStufenZweiSpieler` wird nicht portiert (RU39 misst).
- **Zeitraffer und Pause:** AC-02 wird als Vergleich der JSON-Bäume nach n Schritten mit und ohne Zeitraffer geprüft.
- **`SkillError`:** wird `Fehler::BadRequest { reason }`.
- **Abweichung vom Schnitt:** keine.

**Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Die Go-Engine wächst nicht weiter). Diese Arbeit gehört nicht in den bitgleichen Port. Sie entsteht nach dem Golden-Abgleich (RU22) als neuer Rust-Code. Spec steht im Ticket:

- GS3.4 Spielstand-Liste liefert Route und Chronik (B-978)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU24.1 | `RU24.1-room-stages-switch-pause-dev.md` | Umsetzung | autonom | offen |
| RU24.2 | `RU24.2-room-devcombat-fullsave-saver.md` | Umsetzung | autonom | offen |
| RU24.3 | `RU24.3-room-closeroom-skills-preview.md` | Umsetzung | autonom | offen |
| RU24.4 | `RU24.4-room-worlds-metrics-observer.md` | Umsetzung | autonom | offen |
| RU24.5 | `RU24.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
