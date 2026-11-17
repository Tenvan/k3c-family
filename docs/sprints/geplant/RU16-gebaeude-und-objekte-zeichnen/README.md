# RU16 · CLI · Gebäude und Objekte zeichnen

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

`props` (2521 Zeilen, 18 Importe der Sim) zeichnet Gebäude, Baustellen, Münzen und Objekte.

## Ziel

Gebäude und Objekte eines Zustands erscheinen im Rust-Spiel. Am Ende sichtbar: Bildvergleich einer ausgebauten Basis.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 6 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/props/` → `rust/crates/k3c-game/src/props/`; Dateinamen nach Leitfaden § 2.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/props/`: assets (45, T), buildings (243, T), coin_fountain (23, T), data (126), decor (118, T) | 555 |
| 2 | `native/k3c-game/props/`: decor_glow (130), listen (206, T), nature (215, T) | 551 |
| 3 | `native/k3c-game/props/`: paint (214), plan (350, T) | 564 |
| 4 | `native/k3c-game/props/`: plan_combat (130, T), plan_objects (88) | 218 |
| 5 | `native/k3c-game/props/`: plan_sites (314), props (52), sites (267, T) | 633 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2521 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-game/src/props/**`
- **Abhängig von:** RU13
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Gebäude und Baustellen sehen an fünf Ständen aus wie im Go-Spiel.
- **AC-02** Objekte und Natur stehen an den Positionen des Zustands; die Paket-Tests sind portiert.

## Offene Fragen

Keine zum Port. **Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Das Go-Spiel wächst nicht weiter). Diese Arbeiten entstehen direkt im Rust-Client, sobald die Sim-Seite steht. Spec steht im Ticket:

- SI1.4 Ladeplätze und Fracht an Deck (B-997)
- SI2.4 Doppel-Truhe im Spiel (B-998)
- SI5.4 Hütte, Figur und Bauwerk im Spiel (B-1001)
- WE6.2 Skins der Welt für Gegner, Objekte, Gebäude und Schiff (B-994)

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
