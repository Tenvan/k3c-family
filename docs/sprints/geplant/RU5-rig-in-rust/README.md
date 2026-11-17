# RU5 · CLI · Rig in Rust: Rechnung und Zeichnen der Rig-Figuren

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`native/k3c-rig` (3080 Zeilen Go, Pakete `rig` und `paint`) setzt Figuren aus Teilen zusammen; das Spiel bindet beide Pakete ein.

## Ziel

Rig-Rechnung und Dreiecksnetze der Figuren in Rust, bildgleich zum Go-Rig. Am Ende sichtbar: Vergleichsbilder Go gegen Rust je Figur und Pose.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 3 von 26, 5 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-rig/rig/` → `rust/crates/k3c-rig/src/rig/`, `native/k3c-rig/paint/` → `rust/crates/k3c-rig/src/paint/`; Dateinamen nach Leitfaden § 2.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-rig/rig/`: band (186, T), cloak (120), curve (41), figure (151) | 498 |
| 2 | `native/k3c-rig/rig/`: frames (83), gait (209), keyframes (67) | 359 |
| 3 | `native/k3c-rig/rig/`: load (244), matrix (49), order (73), ride (40) | 406 |
| 4 | `native/k3c-rig/paint/`: actor (258), assets (167), painter (148, T) | 573 |
| 5 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 1836 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

### Hinweise für die Umsetzung

- Matrizen und Kurven rechnen mit `f64` in derselben Reihenfolge wie Go; `sin` und `cos` dürfen hier aus der Standardbibliothek kommen (Zeichnen ist nicht Teil der Sim), die Toleranz steht im Kriterium.
- `paint` zeichnet über `ebiten.DrawTriangles`; in Rust geht das Dreiecksnetz an `k3c_platform::Canvas::mesh`. `mesh` liefert RU4.2: Session 4 (`paint`) bekommt beim Ausarbeiten `Abhängig von: RU4.2`; Sessions 1–3 (`rig`) brauchen RU4 nicht.
- Die Brücke im Spiel (`native/k3c-game/art/rigs.go` → `rust/crates/k3c-game/src/art/rigs.rs`) portiert RU7.6 nach RU5, nicht dieser Sprint (Bereich bleibt `rust/crates/k3c-rig/**`).

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-rig/**`
- **Abhängig von:** RU1, RU2
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Alle Rig-Dateien unter `native/k3c-rig/assets` laden; Knochen-Transformationen stimmen je Pose mit Go-Werten überein (Toleranz 1e-9).
- **AC-02** Die Figur wird als Dreiecksnetz über die Plattform-Schicht gezeichnet, mit Plätzen für Rüstung und Waffe.
- **AC-03** Vergleichsbilder Rust gegen Go weichen je Figur und Pose höchstens um die in `RIG.md` genannte Schwelle ab.

## Offene Fragen

Keine zum Port. **Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Das Go-Spiel wächst nicht weiter). Diese Arbeit entsteht direkt im Rust-Rig, angefangener Go-Code wird neu geschrieben statt konvertiert. Spec steht in der Session:

- NA1.6 Nativer Rig-Export der Bosse auf Stand der Specs

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
