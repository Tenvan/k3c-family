# GEB11 · REG · Hub wächst kleinteilig vom Lagerfeuer bis zur Burg

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** REG
- **Reife:** Entwurf
- **Tickets:** B-458
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Hub hat fünf Stufen (GEB4: Burg und Hub-Stufen 1–5). 🧑 wünscht (2026-10-17), dass er kleinteiliger wächst wie in Kingdom Two Crowns: Lagerfeuer mit Grill, dann ein Haus und so weiter bis zur Burg, je Stufe mit freigeschalteten Gebäuden und Schilden (B-458). GEB9 (Ausbaustufen der Hub-Gebäude) klärt in GEB9.1 die Abgrenzung zu B-458.

## Ziel

Der Hub wächst in mehr, kleineren Stufen sichtbar vom Lagerfeuer bis zur Burg. Am Ende sichtbar: Wachstumsreihe in der Vorschau und im Spiel, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Spielende am TV; 🧑 legt Stufenfolge und Freischaltungen fest; Agent in REG, SIM, DEV und CLI nacheinander.

## Anforderungen

B-458 › Akzeptanzkriterien (Spec noch Entwurf). Sprint-eigen: Regel- und Server-Anteil (B-458/AC-04) läuft hier als eigene Sessions statt als neues Ticket, falls der Workshop `data/` oder `engine/` betrifft.

## Nicht-Ziele

Ausbaustufen der übrigen Hub-Gebäude (GEB9); Werte-Runde zu Kosten der Stufen (Projekt BAL); Migration alter Spielstände (B-288).

## Regeln und Einschränkungen

Feature-Kette REG → SIM → DEV → CLI. **Abhängigkeit:** nach GEB9 (GEB9.1 legt die Abgrenzung fest). Neue Felder in `data/` legt SIM an, Werte REG; Golden nur mit Begründung. Grafik nach `docs/standards/gebaeude-grafik.md` § 6. Deterministisch, 2 Spieler.

## Beispiele

Neue Insel → Lagerfeuer mit Grill; erste Stufe bezahlt → Haus, Bauplatz Werkstatt frei; letzte Stufe → Burg mit Schildern.

## Ausnahme- und Fehlerfälle

Workshop entscheidet „nur Grafik“ → SIM-Session entfällt (`verworfen` mit Grund). Stufe ohne Grafik → Grafik der vorigen Stufe.

## Akzeptanzkriterien

- **AC-01** Stufenfolge und Freischaltungen je Stufe sind als Entscheidung von 🧑 festgelegt und in `docs/rules/` nachgezogen (B-458/AC-01).
- **AC-02** Simulation und Daten kennen die neue Stufenfolge, falls der Workshop Regeln ändert; `task check:go` und `task balance` grün (B-458/AC-04).
- **AC-03** Hub-Grafik je Stufe als Daten-JSON und PNG, die Vorschau zeigt die Wachstumsreihe (B-458/AC-02).
- **AC-04** Das Spiel zeichnet den Hub nach der Stufe; 🧑 nimmt Vorschau und Spiel ab (B-458/AC-03).

## Offene Fragen

🧑: Anzahl Stufen je Level, Reihenfolge der freigeschalteten Gebäude und Schilde, Abgrenzung zu GEB4 und GEB9 (B-458 › Offene Fragen). Reine Grafik oder auch neue Regeln?

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GEB11.1 (REG, Mensch) Workshop: Stufenfolge, Freischaltungen, Abgrenzung (AC-01).
- GEB11.2 (SIM) Neue Stufenfolge in Daten und Simulation, Golden (AC-02; entfällt bei „nur Grafik“).
- GEB11.3 (DEV) Hub-Grafik je Stufe, Wachstumsreihe in der Vorschau (AC-03).
- GEB11.4 (CLI) Spiel zeichnet den Hub nach Stufe (AC-04).
- GEB11.5 (CLI) Review (Code-Sprint): alle Kriterien prüfen.
- GEB11.6 (CLI, Mensch) Abnahme in Vorschau und Spiel (AC-04).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
