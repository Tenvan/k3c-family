# W3 · SIM · Gebäude-Wirkungen

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-116
- **Start-Commit:** –
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, Revision 1; mit Änderungen aus der Spec-Prüfung

## Ausgangslage

Die Gebäude des Regelwerks haben feste Bauplätze, aber keine oder nur teilweise Wirkung (B-116). Hub-Stufen aus W1 sind Voraussetzung.

## Ziel

Tor, Kaserne, Taverne, Heilplatz und Zaubertum wirken laut Regelwerk, Schmiede und Rüstkammer sind baubar.

Am Ende sichtbar: Tests je Gebäude grün, Werte aus den Daten, aktualisierte Golden-Daten.

## Beteiligte und Zielgruppen

Spieler und Bauern; Werte pflegt REG; 🧑 gibt die Spec frei.

## Anforderungen

B-116 › Anforderungen.

## Nicht-Ziele

Wirkung von Schmiede und Rüstkammer (W4, B-122), Schwerter in der Werkstatt (Krieger B-014, W4), Anzeige (W6), Protokoll (W5).

## Regeln und Einschränkungen

Werte nur in `data/`, Logik und Tests in `engine/sim/`; deterministisch (`engine/rng`), mit 2+ Spielern gleichzeitig. Golden-Daten nach dem Golden-Ablauf (B-137) aktualisieren, Spielstand-Änderungen nach der Migrationsregel (B-137). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`. Der Sprint bleibt in der Domäne SIM. Quelle: `docs/rules/materialien-gebaeude.md` §§ 3.2 und 4.

## Beispiele

Tor gebaut → Gegner werden blockiert, eigene Truppen passieren.

## Ausnahme- und Fehlerfälle

Gebäude zerstört → Wirkung entfällt bis zum Wiederaufbau.

## Akzeptanzkriterien

- **AC-01** Je Gebäude wirkt es wie beschrieben: Tor, Kaserne-Limit, Taverne, Heilplatz, Zaubertum-Schaden (Test je Gebäude) (B-116/AC-01).
- **AC-02** Kosten, HP und Platz kommen aus den Daten, nicht aus dem Code (Test) (B-116/AC-02).
- **AC-03** Schmiede und Rüstkammer lassen sich bauen und zerstören, ihre Wirkung ist als offen markiert (Test) (B-116/AC-03).
- **AC-04** Golden-Daten aktualisiert, `task check:go` grün (B-116/AC-04).

## Offene Fragen

- Wirkung von Schmiede und Rüstkammer (B-116, Regelwerk III, B-122); W3 baut sie nur mit Vermerk „Wirkung offen“.
- Taverne: `dawn` fällt nach Q65 auf den **Beginn des Morgengrauens**; das Zyklus-Ticket aus Q65 fehlt noch (die dort genannte Nummer B-213 ist schon vergeben). Offen, ob W3 bis dahin am heutigen `dawn` (Übergang Nacht → Tag) auslöst.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W3.1 | `W3.1-tor-kaserne-taverne.md` | Umsetzung | autonom | offen |
| W3.2 | `W3.2-heilplatz-zaubertum-schmiede-ruestkammer.md` | Umsetzung | autonom | offen |
| W3.3 | `W3.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
