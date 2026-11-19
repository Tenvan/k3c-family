# RU61 · CLI · Rust-Spiel: Nachträge und Wünsche aus den Go-Sprints (CH1, BI1, BI2, NA1, WG2, FB4, SR1)

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-885, B-889, B-890, B-891, B-910, B-911, B-938, B-944, B-947, B-948, B-949, B-1006
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Sprints CH1, BI1, BI2, NA1, WG2, FB4 und NG3 sind geschlossen, ihre Tickets (Anzeige, Eingabe, Look, Rig) blieben offen und zeigten auf das Go-Spiel, das mit RU37 entfällt. B-1143 hat sie hier zusammengeführt und auf `rust/crates/k3c-game` umgehängt.

## Ziel

Die Nachträge und Wünsche aus den Go-Sprints sind im Rust-Spiel umgesetzt oder als erledigt belegt.

**Am Ende sichtbar:** HUD-Icons mit Zahl statt Namen, Tasten-Icons, Weltauswahl im großen Bild, Debug-Werte (F10), Xbox-Testseite, Lader im Spielstil; Mauern ohne Lücken, Bäume in passender Größe; Biom-Eigenschaften je Welt.

## Beteiligte und Zielgruppen

Second Worker setzt um (Lane zweit); 🧑 Ralf nimmt am PC ab; der Hauptrechner schreibt die Sessions vor dem Aktivieren (Reife Entwurf).

## Anforderungen

Reife Entwurf: Sessions werden vor dem Aktivieren als Dateien geschrieben (Stichpunkte, Kriterien in Klammern):

- RU61.1 Look-Nachträge: Hintergrundmauern ohne Lücken (B-947), Bäume in passender Größe (B-948), Lader im Spielstil (B-949); zuerst prüfen, ob RU55 sie schon löst.
- RU61.2 HUD und Eingabe: Ressourcen als Icon mit Zahl (B-890), Tasten-Icons als Papierschnitt (B-911), Xbox-Tasten mit Testseite (B-944), gehaltenes A am Steg (B-1006, erst mit Test prüfen).
- RU61.3 Menü und Debug: Weltauswahl im großen Bild (B-938), Debug-Werte F10 und Logs (B-889, nach RU56).
- RU61.4 Biom-Eigenschaften je Welt in Level, Sim und Anzeige (B-891) und Abnahme der Biom-Darstellung (B-910).
- RU61.5 Rig-Export der Bosse an den Stand nach MG10 anpassen (B-885, nach RU37.1).
- Review und Abnahme-Ticket.

## Nicht-Ziele

Neue Mechaniken außer B-891 (Entscheidung 013: nur in Rust). Sim- und Balance-Läufe. Das Go-Spiel.

## Regeln und Einschränkungen

Das Spiel ist `rust/crates/k3c-game`; die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-game/src/**`, `rust/crates/k3c-game/tests/**`, `rust/crates/k3c-platform/src/**`, `rust/crates/k3c-level/**`, `rust/crates/k3c-sim/**`, `public/grafik/**`, `public/rig/**`, `data/paper.json`, `data/biomes/**`, `tools/k3c-workbench/internal/world/**`, `tools/k3c-workbench/frontend/src/world/**`
- **Abhängig von:** RU37, RU55, RU56, RU60
- **Braucht 🧑:** Abnahme
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** B-947, B-948, B-949 sind im Rust-Spiel belegt oder umgesetzt.
- **AC-02** B-890, B-911, B-944, B-1006 sind im Rust-Spiel belegt oder umgesetzt.
- **AC-03** B-938 und B-889 sind im Rust-Spiel umgesetzt.
- **AC-04** B-891 und B-910 sind umgesetzt und belegt.
- **AC-05** B-885 ist umgesetzt.

## Offene Fragen

Reihenfolge und Zuschnitt der Sessions beim Aktivieren (Hauptrechner). Sonst keine.

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
