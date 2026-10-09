# K3 · SIM · Events Vollmond, Blutmond und Händler-Überfall

- **Status:** geplant
- **Projekt:** KMP
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** B-131
- **Start-Commit:** –
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑, Revision 1

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

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| K3.1 | `K3.1-vollmond-blutmond.md` | Umsetzung | autonom | offen |
| K3.2 | `K3.2-haendler-ueberfall-golden.md` | Umsetzung | autonom | offen |
| K3.3 | `K3.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
