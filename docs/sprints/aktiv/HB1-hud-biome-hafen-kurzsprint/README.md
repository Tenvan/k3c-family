# HB1 · CLI, SIM · Kurz-Sprint: HUD-Texte, dichte Biome, Landstreicher-Lager, Hafen als Start

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI, SIM
- **Reife:** bereit
- **Tickets:** B-1186, B-1187, B-1188
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (2026-11-21, Auftrag von 🧑)

## Ausgangslage

HUD-Texte an Baustellen und Gebäuden überladen das Bild, Händler und Stände fehlen, Biome sind nur im Hub-Kompass sichtbar, Landstreicher-Lager liegen frei, neue Spiele starten nicht am Hafen. Tickets B-1186, B-1187, B-1188.

## Ziel

Lesbares HUD, dicht bewachsene Biome mit Landstreicher-Lagern zwischen Bäumen und Taverne an der Lagerstelle, Start am bewaldeten Hafen. Am Ende sichtbar: Baustellen mit Icons und x/n, Gebäudename über dem Monarchen, Biome am Radar, Spielstart im Wald am Hafen.

## Beteiligte und Zielgruppen

Spieler; Second Worker setzt um; 🧑 Ralf nimmt am PC ab.

## Anforderungen

Siehe B-1186, B-1187, B-1188 › Anforderungen.

## Nicht-Ziele

Neue Einheiten, Balancing, neue Inseln.

## Regeln und Einschränkungen

Rust-Spiel, neue Mechaniken nur in Rust (Entscheidung 013). Go-Goldens nur bei gewollter Datenänderung neu (Begründung im Commit). Keine Rücksicht auf alte Spielstände.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-level/**`, `rust/crates/k3c-room/**`, `data/**`, `engine/level/**`, `testdata/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** golden

## Beispiele

Siehe Tickets.

## Ausnahme- und Fehlerfälle

Siehe Tickets.

## Akzeptanzkriterien

- **AC-01** B-1186/AC-01 bis AC-03.
- **AC-02** B-1187/AC-01 bis AC-04.
- **AC-03** B-1188/AC-01 bis AC-03.
- **AC-04** Review ohne schwere Befunde, Abnahme-Ticket angelegt.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| HB1.1 | `HB1.1-baustellen-texte-icons.md` | Umsetzung | autonom | fertig |
| HB1.2 | `HB1.2-gebaeudename-haendler.md` | Umsetzung | autonom | fertig |
| HB1.3 | `HB1.3-biome-dicht-radar.md` | Umsetzung | autonom | fertig |
| HB1.4 | `HB1.4-landstreicher-lager-taverne.md` | Umsetzung | autonom | fertig |
| HB1.5 | `HB1.5-hafen-start-bewuchs.md` | Umsetzung | autonom | fertig |
| HB1.6 | `HB1.6-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
