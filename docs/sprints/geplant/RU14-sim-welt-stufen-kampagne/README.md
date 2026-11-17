# RU14 · SIM · Sim: Welt-Schritt, Kampagne, Schiff und Vorschau

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`world*.go` (1396 Zeilen) rechnet den Schritt einer Stufe; dazu Kampagne, Schiff, Vorschau und Orte (rund 330 Zeilen).

## Ziel

Der Schritt einer Stufe und der Ablauf einer Kampagne laufen in Rust. Am Ende sichtbar: portierte Tests grün.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 11 von 26, 5 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/sim/` → `rust/crates/k3c-sim/src/{world,campaign}/`; Dateinamen nach Leitfaden § 2.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `engine/sim/`: campaign (117, T), preview (112, T), ship (76, T), world (231) | 536 |
| 2 | `engine/sim/`: world_cycle (122, T), world_lava (46, T), world_lines (108, T), world_state_mirror (71, T) | 347 |
| 3 | `engine/sim/`: world_travel (74), world_values (289, T), world_values_locked (51) | 414 |
| 4 | `engine/sim/`: world_values_merge (267), world_values_room (84, T), world_vein (53, T) | 404 |
| 5 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 1701 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-sim/src/world/**`, `rust/crates/k3c-sim/src/campaign/**`
- **Abhängig von:** RU10
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Reihenfolge der Teilschritte ist dieselbe wie in Go; Teilschritte anderer Gruppen sind über feste Aufrufpunkte eingehängt.
- **AC-02** Die Go-Tests zu Kampagne, Schiff und Vorschau sind portiert und grün.

## Offene Fragen

Keine zum Port. **Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Die Go-Engine wächst nicht weiter). Diese Arbeiten gehören nicht in den bitgleichen Port. Sie entstehen nach dem Golden-Abgleich (RU22) als neuer Rust-Code, angefangener Go-Code wird neu geschrieben statt konvertiert. Spec steht im Ticket:

- SI1.2 Laderaum, Ladeplätze und Fracht in der Engine (B-967)
- GS3.1 Chronik abgeschlossener Inseln im Spielstand (B-966)

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
