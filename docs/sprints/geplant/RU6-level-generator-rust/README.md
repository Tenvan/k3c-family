# RU6 · SIM · Level-Generator in Rust, Level-Golden bitgleich

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

`engine/level` (2206 Zeilen, 2019 Zeilen Tests) erzeugt aus einem Seed ein Level; sechs Level-Golden und `nature.json` prüfen jedes Feld.

## Ziel

Der Level-Generator läuft in Rust und trifft alle Level-Golden bitgleich; damit ist der Weg „Golden als Orakel“ belegt. Am Ende sichtbar: `task rust:test -- -p k3c-level` grün mit allen Level-Golden.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 9 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/level/` → `rust/crates/k3c-level/src/`; Dateinamen nach Leitfaden § 2.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `engine/level/`: backdrop (34, T), biome (172), guards (96, T) | 302 |
| 2 | `engine/level/`: level (355, T), lines (57), nature (218, T) | 630 |
| 3 | `engine/level/`: nature_check (173), planned (269, T) | 442 |
| 4 | `engine/level/`: planned_gen (121, T), planned_totals (88), ruins (158, T) | 367 |
| 5 | `engine/level/`: sites (275, T), validate (190) | 465 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2206 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

### Hinweise für die Umsetzung

- Der Generator würfelt nur über `k3c-rng`; jede Würfel-Stelle bleibt an derselben Stelle der Reihenfolge, auch wenn das Ergebnis verworfen wird.
- Die Level-Golden vergleicht `engine/level/*_test.go` über `engine/internal/golden`; denselben Vergleich liefert `k3c_det::golden` (RU3).

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-level/**`
- **Abhängig von:** RU3, RU39
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** `level-cave`, `-crystal`, `-forest`, `-ironhold`, `-mine`, `-planned` und `nature.json` werden von Rust bitgleich (gleicher JSON-Baum, jede Zahl mit denselben Bits) erzeugt, ohne Golden-Neuschrieb.
- **AC-02** Die Tests aus `engine/level` (17 Dateien) sind sinngleich portiert und grün.
- **AC-03** Das Review nennt Sessions und Zeilen je Session (Plan: 450 Zeilen Code je Session); bei mehr als dem 1,5-Fachen des Plans empfiehlt es, vor RU10 neu zu schätzen.

## Offene Fragen

keine

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
