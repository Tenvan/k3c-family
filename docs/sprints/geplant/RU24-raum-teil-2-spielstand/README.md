# RU24 · SRV · Raum Teil 2 und Spielstand-Format

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** SRV
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach RU21 fehlen Stufenwechsel, Inselwechsel, Zeitraffer und der Rest des Raums.

## Ziel

Der Raum ist vollständig; Speichern und Laden ergibt denselben Welt-Hash. Am Ende sichtbar: alle Tests aus `engine/room` und `engine/store` portiert und grün.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 14 von 26, 5 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/room/` → `rust/crates/k3c-room/src/room/`; Dateinamen nach Leitfaden § 2.

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

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

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

Keine zum Port. **Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Die Go-Engine wächst nicht weiter). Diese Arbeit gehört nicht in den bitgleichen Port. Sie entsteht nach dem Golden-Abgleich (RU22) als neuer Rust-Code. Spec steht im Ticket:

- GS3.4 Spielstand-Liste liefert Route und Chronik (B-978)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
