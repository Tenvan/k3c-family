# K3 · SIM · Events Vollmond, Blutmond und Händler-Überfall

- **Status:** erledigt
- **Projekt:** KMP
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** B-131, B-373
- **Start-Commit:** 1ac1824
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-09, Chat, durch 🧑, Revision 2 (HP 100, speichern, Session K3.2a)

## Ausgangslage

Der Nacht-Rhythmus kennt keine Events (B-131). Alpha-Wolf (K1) und Händler (W4) sind Voraussetzung.

## Ziel

Vollmond, Blutmond und Händler-Überfall lösen nach Rhythmus aus und wirken auf Wellen, Schaden und Drops.

Am Ende sichtbar: Tests je Event grün, aktualisierte Golden-Daten.

## Beteiligte und Zielgruppen

Spieler und REG für Werte; 🧑 gibt die Spec frei.

## Anforderungen

B-131 › Anforderungen.

## Nicht-Ziele

Protokoll (K4), Anzeige (K5), Balancing (BR2).

## Regeln und Einschränkungen

Werte nur in `data/`, Logik und Tests in `engine/sim/`; deterministisch (`engine/rng`), mit 2+ Spielern gleichzeitig. Golden-Daten nach dem Golden-Ablauf (B-137) aktualisieren, Spielstand-Änderungen nach der Migrationsregel (B-137). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`. Der Sprint bleibt in der Domäne SIM. Quelle: `docs/rules/bosse.md` § 2.

## Beispiele

7. Nacht → Vollmond mit verstärkter Wolfswelle und Alpha-Wolf.

## Ausnahme- und Fehlerfälle

Händler-Überfall ohne Händler-Besuch → kein Überfall.

## Akzeptanzkriterien

- **AC-01** Auslöser nach Rhythmus (7., 13., jeder 4. Besuch), Wirkung auf Wellen, Schaden und Drops laut Daten (Test) (B-131/AC-01).
- **AC-02** Händler-Überfall nur bei Besuch, Belohnung bei Schutz (Test) (B-131/AC-02).
- **AC-03** Golden-Daten aktualisiert, `task check:go` grün (B-131/AC-03).
- **AC-04** Der Händler ist eine angreifbare Figur mit 100 HP, die bei 0 abreist (Test) (B-373/AC-01).
- **AC-05** Besuchszähler je Insel und anwesender Händler überstehen Speichern und Laden, alte Stände laden (Test) (B-373/AC-02).

## Offene Fragen

Revision 2 (2026-10-09): K3.2 war blockiert, weil der Händler aus W4.2 weder angreifbar ist noch Besuche zählt (B-373). 🧑 hat entschieden: HP 100, anwesender Händler wird gespeichert, neue Session K3.2a vor K3.2.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| K3.1 | `K3.1-vollmond-blutmond.md` | Umsetzung | autonom | fertig |
| K3.2a | `K3.2a-haendler-figur-besuche.md` | Umsetzung | autonom | fertig |
| K3.2 | `K3.2-haendler-ueberfall-golden.md` | Umsetzung | autonom | fertig |
| K3.3 | `K3.3-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-10-09, Review K3.3: AC-01 K3.1 (a–g) und K3.2 (`JederVierteBesuch`), AC-02 K3.2 (b–f), AC-03 Golden geprüft ohne Diff (Events nur auf Inseln, kein rng-Aufruf ohne Insel, `rng.json` unverändert), AC-04 und AC-05 K3.2a › Ergebnis.
`task check` (MCP) und `task check:go` (Shell, B-372) grün. Keine schweren Befunde; Spielstand v6 mit Fixture und Ladetest, alte Stände laden mit Zähler 0.
Leicht, als Ticket: B-379 (Vollmond-Belohnung nur in Stufe 0), B-380 (Blutmond ohne Boss-Flächenangriffe), B-381 (`merchant.raid`/HP im Protokoll, zu K4).
Version: v0.15.0 vorgeschlagen (Minor: Vollmond, Blutmond und Händler-Überfall wirken im Spiel; v0.16.0, falls K2 vorher getaggt wird); gesetzt erst nach Bestätigung durch 🧑.
