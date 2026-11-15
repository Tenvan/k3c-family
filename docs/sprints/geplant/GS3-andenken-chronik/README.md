# GS3 · SIM, PAP, CLI, SRV · Andenken und Chronik abgeschlossener Inseln auf der Seekarte

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** SIM, PAP, CLI, SRV
- **Reife:** bereit
- **Tickets:** B-966, B-978
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Die Seekarte zeigt abgeschlossene Inseln nur als Zustand (B-962, gestaltet in GS2). Der Spielstand kennt die abgeschlossenen Insel-IDs, aber weder Tage noch Spieler (`engine/sim/island_save.go`, Version 9). 🧑 2026-11-15 will Andenken und eine Chronik (B-966). B-966 ist zerlegt: Chronik, Trophäen und Anzeige im Spiel bleiben in B-966, die Vorschau in der Spielstand-Auswahl ist B-978.

## Ziel

Jede abgeschlossene Insel trägt auf der Seekarte Trophäe, Abschlusszeile und zwei Zahlen; dieselbe Karte erscheint als Vorschau in der Spielstand-Auswahl.

## Beteiligte und Zielgruppen

Spielende am TV (1–2 Monarchen). Claude setzt um und legt Assets über `assets_*` an; 🧑 gibt frei und nimmt am TV ab.

## Anforderungen

- Chronik im Spielstand, Trophäen-Assets, Andenken auf der Seekarte im Spiel: siehe `B-966 › Anforderungen`.
- Seekarte als Vorschau in der Spielstand-Auswahl: siehe `B-978 › Anforderungen`.

## Nicht-Ziele

Abbild der Burg, ganze Geschichte zum Nachlesen, eigener Menüpunkt „Chronik“, Namen oder Farben der Spieler, eigene Trophäen je Endboss.

## Regeln und Einschränkungen

- Reihenfolge SIM → PAP → CLI → SRV → CLI → Review; ein Commit je Session auf `sprint/gs3`, ein PR.
- GS3.1 ändert das Spielstand-Format (Version erhöhen, Fixture; keine Rücksicht auf alte Stände) und läuft im eigenen Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`); Review vor dem Commit.
- Assets nur über `assets_*`, Fachprüfung `k3c-grafik`; HUD und Menü prüft `k3c-hud`.

- **Bereich:** `docs/rules/welten.md`, `engine/sim/**`, `engine/store/**`, `public/grafik/objekte/**`, `data/paper.json`, `native/k3c-game/hud/**`, `native/k3c-game/view/**`, `native/k3c-game/local/**`, `native/k3c-game/menu/**`
- **Abhängig von:** GS1, GS2
- **Braucht 🧑:** Abnahme
- **Sperre:** spielstand

## Beispiele

Sengoku, Insel 1 nach 14 Tagen zu zweit abgeschlossen: Auf der Seekarte an Insel 2 steht auf der Reisfeld-Ebene die Maske mit Fahne, mit dem Zeiger darauf erscheinen die Abschlusszeile und „14 Tage · 2 Monarchen“. In der Spielstand-Auswahl zeigt der markierte Stand dieselbe Karte.

## Ausnahme- und Fehlerfälle

- Stand ohne Chronik → Trophäe ohne Zahlen.
- Insel ohne Abschlusszeile → nur Zahlen.
- Nicht ladbarer Stand → keine Vorschau.
- Einzelheiten: `B-966`, `B-978 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Die Engine hält beim Ablegen Insel, Tage und Monarchen fest; Speichern und Laden erhalten die Chronik, Version erhöht, Fixture; Regeltext und Glossar ergänzt (B-966/AC-01 bis AC-03).
- **AC-02** Trophäen-Assets der vier Welten angelegt und zugeordnet (B-966/AC-04).
- **AC-03** Die Seekarte zeigt Trophäe, Abschlusszeile und Zahlen für abgeschlossene Inseln, für beide Spieler gleich (B-966/AC-05, AC-06).
- **AC-04** Die Liste der Spielstände liefert Route, aktuelle Insel, abgeschlossene Inseln und Chronik (B-978/AC-01).
- **AC-05** Die Spielstand-Auswahl zeigt die Vorschau des markierten Stands; Laden, Löschen und B unverändert (B-978/AC-02, AC-03).
- **AC-06** 🧑 am TV: Andenken auf der Seekarte und Vorschau im Menü geprüft (B-966/AC-07, B-978/AC-04).

## Offene Fragen

Alle mit Default entschieden, 🧑 bestätigt bei der Freigabe:

- Ein Trophäen-Motiv je Welt: Standard Kristallsplitter, Sengoku Maske, Nil Skarabäus, Britannien Horn (B-966).
- „Wer dabei war“ = Anzahl der Monarchen; „Tage“ = Tage auf dieser Insel bis zum Ablegen (B-966).
- Vorschau rechts neben der Liste (B-978).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GS3.1 | `GS3.1-chronik-spielstand.md` | Umsetzung | autonom | offen |
| GS3.2 | `GS3.2-assets-trophaeen.md` | Umsetzung | autonom | offen |
| GS3.3 | `GS3.3-andenken-seekarte.md` | Umsetzung | autonom | offen |
| GS3.4 | `GS3.4-spielstand-liste-route.md` | Umsetzung | autonom | offen |
| GS3.5 | `GS3.5-vorschau-spielstand-auswahl.md` | Umsetzung | autonom | offen |
| GS3.6 | `GS3.6-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
