# GS4 · REG, SIM, PAP, CLI · Flaschenpost am Strand mit Geschichtszeile und Markierung

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** REG, SIM, PAP, CLI
- **Reife:** bereit
- **Tickets:** B-971, B-980
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Die Geschichte (GS1) blendet nur an Wendepunkten ein; Erkunden wird davon nicht belohnt. 🧑 2026-11-15 hat die Flaschenpost ausgewählt (B-971). Truhen sind gewöhnliche Level-Objekte, versteckte gibt es nicht; Einsiedler (B-972) und Reittier-Gatter (B-970) sind noch offen (SI1). B-971 ist zerlegt: Regel und Simulation bleiben in B-971, die Darstellung im Spiel ist B-980.

## Ziel

Alle paar Tage liegt bei Tagesanbruch eine Flaschenpost am Strand; wer sie aufhebt, sieht eine Zeile Kunstsprache, und Pfeil und Leuchten führen beide Spieler zu einer noch nicht gefundenen Truhe.

## Beteiligte und Zielgruppen

Spielende am TV (1–2 Monarchen). Claude schreibt Regel, Sim, Asset und Darstellung; 🧑 gibt frei und nimmt am TV ab. Die Häufigkeit stimmt später das Balancing ab (gesperrt).

## Anforderungen

- Regel, Daten, Simulation und Spielstand der Flaschenpost: siehe `B-971 › Anforderungen`.
- Flasche, Geschichtszeile, Pfeil und Leuchten im Spiel: siehe `B-980 › Anforderungen`.

## Nicht-Ziele

Schwäche des Endgegners als Hinweis, sammelbare Flaschen, Hinweis auf der Seekarte, eigene Flaschenpost-Truhen, Einsiedler und Gatter selbst (B-972, B-970), Ton, Balancing der Häufigkeit.

## Regeln und Einschränkungen

- Umgehängt auf Rust (B-1143, 2026-11-19): Sim in `rust/crates/k3c-sim`, Anzeige in `rust/crates/k3c-game` (Entscheidung 013). Die Go-Sessions GS4.2 und GS4.4 sind verworfen; GS4.6 und GS4.7 ersetzen sie. Der Sprint wartet auf RU37 und RU60.
- Reihenfolge REG → SIM → PAP → CLI → Review; ein Commit je Session auf `sprint/gs4`, ein PR.
- GS4.6 (Sim-Code, Spielstand, Golden) braucht Review vor dem Commit. Rust schreibt eigene Golden erst nach RU41; bis dahin ändert die Session `testdata/golden/**` nur mit Begründung im Commit. Keine Simulation, kein Balancing.
- Das Spielstand-Format ändert auch GS3; deshalb hängt GS4 von GS3 ab (Versionen steigen nacheinander).
- Asset nur über `assets_*`, Fachprüfung `k3c-grafik`; HUD prüft `k3c-hud`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `docs/rules/wirtschaft.md`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-data/**`, `data/bottle.json`, `testdata/golden/**`, `public/grafik/objekte/**`, `data/paper.json`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/world/**`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/view/**`, `rust/crates/k3c-game/src/fx/**`
- **Abhängig von:** GS1, GS3, RU37, RU60
- **Braucht 🧑:** Abnahme
- **Sperre:** golden

## Beispiele

Sengoku, Tag 5, Tagesanbruch: Am linken Inselende glitzert eine Flasche. Monarch 1 reitet darüber, eine Zeile Kunstsprache blendet ein, in beiden Hälften zeigt ein Pfeil nach rechts unten. Monarch 2 folgt ihm in Stufe 2 und findet die leuchtende Truhe; die Markierung erlischt.

## Ausnahme- und Fehlerfälle

- Keine ungefundene Truhe mehr → keine Flasche.
- Markierung läuft → keine neue Flasche.
- Insel ohne Tiefe 0 → keine Flaschenpost.
- Einzelheiten: `B-971`, `B-980 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** `docs/rules/wirtschaft.md` hat den Abschnitt Flaschenpost, Glossar ergänzt (B-971/AC-01).
- **AC-02** Flasche erscheint deterministisch bei `dawn`, wird aufgehoben oder von der Flut geholt (B-971/AC-02, AC-03, AC-08).
- **AC-03** Ziel, Markierung und Zustand für alle Spieler der Insel; Geschichtszeile reihum aus `story.bottles` (B-971/AC-04 bis AC-06).
- **AC-04** Spielstand erhält Flasche und Markierung, Version erhöht, Fixture; Golden gewollt neu, `check_run task:check:go` grün (B-971/AC-07, AC-09).
- **AC-05** Asset „Flaschenpost“ angelegt und zugeordnet (B-980/AC-01).
- **AC-06** Das Spiel zeichnet Flasche, Geschichtszeile, Pfeil und Leuchten in beiden Spielerhälften und löscht sie am Ziel (B-980/AC-02 bis AC-05).
- **AC-07** 🧑 am TV: Flasche aufheben und dem Pfeil folgen (B-980/AC-06).

## Offene Fragen

Alle mit Default entschieden, 🧑 bestätigt bei der Freigabe:

- Erste Flasche ab Tag 3, danach frühestens alle 4 Tage (vorläufig, Balancing später).
- Ziel ist eine noch nicht gefundene gewöhnliche Truhe; ohne Ziel keine Flasche; während einer Markierung keine neue Flasche (B-971).
- Einsiedler und Gatter sind keine Abhängigkeit, die Zielart ist erweiterbar (B-971).
- Pfeil zeigt bei Ziel in anderer Stufe schräg zum nächsten Übergang (B-980).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GS4.1 | `GS4.1-regeltext-flaschenpost.md` | Umsetzung | autonom | fertig |
| GS4.2 | `GS4.2-flaschenpost-sim.md` | Umsetzung | autonom | verworfen |
| GS4.3 | `GS4.3-asset-flaschenpost.md` | Umsetzung | autonom | fertig |
| GS4.4 | `GS4.4-flaschenpost-spiel.md` | Umsetzung | autonom | verworfen |
| GS4.5 | `GS4.5-review.md` | Review | autonom | offen |
| GS4.6 | `GS4.6-flaschenpost-sim-rust.md` | Umsetzung | autonom | offen |
| GS4.7 | `GS4.7-flaschenpost-spiel-rust.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
