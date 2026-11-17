# RU13 · CLI · Welt und Akteure zeichnen

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

`world` (1255 Zeilen) zeichnet Boden, Hintergrund und Tageszeit, `actors` (1447 Zeilen) Monarch, Truppen und Gegner aus dem Sim-Zustand.

## Ziel

Ein Sim-Zustand wird im Rust-Spiel als Welt mit Figuren gezeichnet. Am Ende sichtbar: Standbild aus einem Go-Zustand, Bildvergleich gegen das Go-Spiel.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 5 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/world/` → `rust/crates/k3c-game/src/world/`, `native/k3c-game/actors/` → `rust/crates/k3c-game/src/actors/`; Dateinamen nach Leitfaden § 2.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/world/`: backdrop (196), camera (142, T), cull (54, T), ground (56), layers (98, T) | 546 |
| 2 | `native/k3c-game/world/`: light (150, T), lightdraw (128), river (134, T), water (191, T) | 603 |
| 3 | `native/k3c-game/world/`: world (106); `native/k3c-game/actors/`: actors (92), anim (168, T) | 366 |
| 4 | `native/k3c-game/actors/`: catalog (216, T), combatmarks (123, T), draw (130, T), figures (326, T) | 795 |
| 5 | `native/k3c-game/actors/`: levels (72, T), markdraw (153), marks (114, T), pose (53) | 392 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2702 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-game/src/world/**`, `rust/crates/k3c-game/src/actors/**`
- **Abhängig von:** RU7, RU8
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Welt und Tageszeit eines geladenen Zustands sehen aus wie im Go-Spiel (Bildvergleich an fünf Ständen).
- **AC-02** Monarch, Truppen und Gegner stehen und laufen an den Positionen des Zustands; die Paket-Tests sind portiert.

## Offene Fragen

Keine zum Port. **Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Das Go-Spiel wächst nicht weiter). Diese Arbeiten entstehen direkt im Rust-Client, sobald die Sim-Seite steht. Spec steht im Ticket:

- SI4.4 Gatter und Reittier je Monarch im Spiel (B-1000)
- WE6.1 Bau und Endboss in jedem Biom, Ersatz-Asset der Basisart (B-993)

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
