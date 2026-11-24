# WE8 · REG, SIM, CLI · Welten-Inhalte: Events Ninja-Überfall, Sandsturm, Nebel und Zuordnung in den Welten

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** REG, SIM, CLI
- **Reife:** bereit
- **Tickets:** B-991, B-992, B-995, B-996
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Nach WE5 bis WE7 gibt es Event-Overrides, Skins im Spiel und im Editor und die Assets der drei Welten. Es fehlen die Events Ninja-Überfall, Sandsturm und Nebel (B-960) und die Zuordnung in Sengoku, Nil und Britannien.

## Ziel

Die drei Welten spielen sich unterschiedlich. Am Ende sichtbar: Sengoku mit Ninja und Überfall, Nil mit Palmen und Sandsturm, Britannien mit Steinkreis und Nebel am PC.

## Beteiligte und Zielgruppen

Spieler (2 Spieler). 🧑 gibt frei (neue Spielregeln) und nimmt am PC ab.

## Anforderungen

Siehe `B-991 › Anforderungen`, `B-992 › Anforderungen`, `B-995 › Anforderungen`, `B-996 › Anforderungen`. Sprint-eigen: Die Standard-Welt und `testdata/golden/` bleiben unverändert.

## Nicht-Ziele

Taifun, Flut, Schneesturm (B-969), Ton der Events, Balancing, Simulationen.

## Regeln und Einschränkungen

Umgehängt auf Rust (B-1143, 2026-11-19): Sim in `rust/crates/k3c-sim`, Raum in `rust/crates/k3c-room`, Anzeige in `rust/crates/k3c-game` (Entscheidung 013). Die Go-Sessions WE8.2, WE8.3 und WE8.4 sind verworfen; WE8.7 und WE8.8 ersetzen sie. Der Sprint wartet auf RU37 und RU60.

Kette REG → SIM → CLI → Zuordnung, alle Sessions nacheinander. WE8.5 läuft im Hauptcheckout über `world_*`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `docs/rules/bosse.md`, `docs/protocol.md`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-room/**`, `data/events.json`, `rust/crates/k3c-game/src/**`, `data/worlds/sengoku.json`, `data/worlds/nil.json`, `data/worlds/britannien.json`
- **Abhängig von:** WE5, WE6, WE7, B-969, RU37, RU60
- **Braucht 🧑:** Abnahme
- **Sperre:** protokoll

## Beispiele

Sengoku, 5. Nacht → Ankündigung, zusätzliche schnelle Ninja. Nil, 6. Nacht → Sandsturm-Schleier, Bogner treffen später.

## Ausnahme- und Fehlerfälle

B-969 legt einen anderen Wetter-Rahmen fest → WE8.1 folgt B-969 und passt B-992 an (Revision), bevor WE8.3 startet. Asset aus WE7 fehlt → Skin weglassen, Folgeticket.

## Akzeptanzkriterien

- **AC-01** `docs/rules/bosse.md` § 2 beschreibt Überfall, Sandsturm und Nebel mit Daten-Feldern (B-991/AC-04, B-992/AC-04, Anteil Regeltext).
- **AC-02** Event `raid` in der Engine (B-991/AC-01 bis AC-04).
- **AC-03** Events `sandstorm` und `fog` in der Engine (B-992/AC-01 bis AC-04).
- **AC-04** Spiel kündigt die Events an und zeigt Schleier (B-995/AC-01, AC-02).
- **AC-05** Die drei Welten tragen Skins, Namen und ihr Event; `world_check` ohne Fehler (B-996/AC-01, AC-02).
- **AC-06** `testdata/golden/` unverändert, `task check:go` grün.

## Offene Fragen

Keine blockierende außer der Abhängigkeit von B-969 (Wetter-Rahmen). Defaults in den Tickets: Mechanik und Werte des Überfalls (B-991), Wirkung von Sandsturm und Nebel (B-992), Zuordnung Variante → Basisart und „Samurai-Lager“ (B-996). Sichtprüfung 🧑: B-996/AC-03.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WE8.1 | `WE8.1-regeln-events.md` | Umsetzung | autonom | offen |
| WE8.2 | `WE8.2-event-ueberfall.md` | Umsetzung | autonom | verworfen |
| WE8.3 | `WE8.3-events-sandsturm-nebel.md` | Umsetzung | autonom | verworfen |
| WE8.4 | `WE8.4-spiel-zeigt-events.md` | Umsetzung | autonom | verworfen |
| WE8.5 | `WE8.5-welten-zuordnen.md` | Umsetzung | autonom | offen |
| WE8.6 | `WE8.6-review.md` | Review | autonom | offen |
| WE8.7 | `WE8.7-events-sim-rust.md` | Umsetzung | autonom | offen |
| WE8.8 | `WE8.8-spiel-zeigt-events-rust.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
