# RU62 · SIM · Welt-Features SI1 bis SI6 neu in Rust (Sim und Anzeige, Entwurf)

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** B-967, B-997, B-968, B-998, B-969, B-999, B-970, B-1000, B-972, B-1001, B-973, B-1002
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Sprints SI1 bis SI6 (Fracht, Doppel-Truhe, Wetter je Welt, Reittiere, Einsiedler, Fest) haben ihre Sim- und Anzeige-Sessions mit dem Go-Stopp verworfen; Regeltext und Assets (SI1.1, SI1.3 usw.) stehen teils. Die Tickets blieben eingeplant im geschlossenen Sprint. B-1143 hat sie hier zusammengeführt und auf Rust umgehängt.

## Ziel

Die sechs Welt-Features laufen im Rust-Spiel: Sim in `rust/crates/k3c-sim`, Anzeige in `rust/crates/k3c-game`.

**Am Ende sichtbar:** Fracht an Deck, Doppel-Truhe mit Koop-Aktion, Taifun und Flut mit Vorboten, Welt-Reittiere, Einsiedler mit Angebot, Fest nach dem Sieg mit Musik.

## Beteiligte und Zielgruppen

Second Worker setzt um (Lane zweit); 🧑 Ralf gibt die Regeltexte frei und nimmt am PC ab; der Hauptrechner schreibt die Sessions vor dem Aktivieren (Reife Entwurf).

## Anforderungen

Reife Entwurf: Je Welt-Feature zwei bis drei Sessions (Regeltext REG, Sim SIM, Assets PAP, Anzeige CLI), vor dem Aktivieren als Dateien geschrieben (Stichpunkte, Kriterien in Klammern):

- RU62.1 Fracht auf dem Schiff (B-967 Sim, B-997 Anzeige).
- RU62.2 Doppel-Truhe und Koop-Aktionen (B-968 Sim, B-998 Anzeige).
- RU62.3 Wetter je Welt (B-969 Sim, B-999 Vorboten, Taifun, Flut mit Ton).
- RU62.4 Reittiere je Welt (B-970 Sim, B-1000 Anzeige).
- RU62.5 Einsiedler mit Angebot und Bauwerk (B-972 Sim, B-1001 Anzeige).
- RU62.6 Fest nach dem Sieg (B-973 Sim, B-1002 Anzeige und Festmusik).
- Review und Abnahme-Ticket.

Bereits vorhandene Regeltexte und Assets aus den SI-Sprints werden gelesen und weiterverwendet (SI1 bis SI6 unter `docs/sprints/erledigt/`).

## Nicht-Ziele

Neue Mechaniken außer den genannten Tickets. Sim- und Balance-Läufe (gesperrt). Änderungen an der Go-Engine.

## Regeln und Einschränkungen

Neue Mechanik: Logik und Test in `rust/crates/k3c-sim`, das Spiel zeichnet nur (Entscheidung 013). Das Spielstand-Format ändert sich mit mehreren Tickets (Sperre `spielstand`): Versionen steigen nacheinander, Fixture, keine Rücksicht auf alte Spielstände. Rust schreibt eigene Golden erst nach RU41.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `docs/rules/**`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-room/**`, `rust/crates/k3c-data/**`, `rust/crates/k3c-game/src/**`, `public/grafik/**`, `public/audio/**`, `data/**`, `testdata/golden/**`
- **Abhängig von:** RU37, RU60, B-961
- **Braucht 🧑:** Freigabe, Abnahme
- **Sperre:** golden, spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** B-967 und B-997 sind umgesetzt und belegt.
- **AC-02** B-968 und B-998 sind umgesetzt und belegt.
- **AC-03** B-969 und B-999 sind umgesetzt und belegt.
- **AC-04** B-970 und B-1000 sind umgesetzt und belegt.
- **AC-05** B-972 und B-1001 sind umgesetzt und belegt.
- **AC-06** B-973 und B-1002 sind umgesetzt und belegt.

## Offene Fragen

Regeltexte brauchen die Freigabe von 🧑 Ralf je Ticket (Feld `Braucht`). Zuschnitt der Sessions beim Aktivieren (Hauptrechner). Sonst keine.

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
