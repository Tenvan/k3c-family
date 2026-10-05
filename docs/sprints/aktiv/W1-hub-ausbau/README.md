# W1 · SIM · Hub-Ausbau und Mauerstufen

- **Status:** aktiv
- **Domäne:** SIM
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-112
- **Start-Commit:** ef16c9a
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, Revision 1; mit Änderungen aus der Spec-Prüfung

## Ausgangslage

Nach W0 (B-206) hat jede Stufe feste Hub-Plätze mit Hub-Stufe aus den Daten und je Seite fünf Mauerlinien mit Mauer-, Turm- und Tor-Platz; `World.HubLevel` (Start 1) steuert bisher nur die Linien- und Tor-Regel, Hub-Gebäude sind noch nicht gesperrt (Q59). Mauer und Turm haben nur die Holzstufe, es gibt weder Hub-Ausbau noch Reparatur (B-112). Voraussetzung sind der Insel-Vorrat aus dem Lager (SP13) und W0.

## Ziel

Hub-Stufen 1 bis 5 und Mauer-/Turm-Stufen 1 bis 5 sind in der Simulation spielbar, Reparatur zwischen den Wellen eingeschlossen.

Am Ende sichtbar: `task check:go` grün, Tests für Ausbau, Zerstörung und Reparatur, aktualisierte Golden-Daten.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen) und Bauern; Werte pflegt REG; 🧑 gibt die Spec frei.

## Anforderungen

B-112 › Anforderungen.

## Nicht-Ziele

Gebäude-Wirkungen (W3), Protokoll (W5), Anzeige (W6, B-117).

## Regeln und Einschränkungen

Werte nur in `data/`, Logik und Tests in `engine/sim/`; deterministisch (`engine/rng`), mit 2+ Spielern gleichzeitig. Golden-Daten nach dem Golden-Ablauf (B-137) aktualisieren, Spielstand-Änderungen nach der Migrationsregel (B-137). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`. Der Sprint bleibt in der Domäne SIM. Quelle: `docs/rules/materialien-gebaeude.md`. Begriffe (§ 4): **Reparatur** (kostenlos, Anteil der Bauzeit) gilt nur für beschädigte Gebäude; ein **zerstörtes** Gebäude verliert die Stufe, der Platz ist wieder `unpaid` und wird gegen Bezahlung neu gebaut. „Bis die Mauer k−1 repariert ist“ (Q58) heißt bei zerstörter Mauer: bis sie neu gebaut ist.

## Beispiele

Hub-Stufe 1, Vorrat 100 Stein, 50 Gold gezahlt → Material abgebucht → Bauer baut → Hub-Stufe 2 → Steinmauer baubar.

## Ausnahme- und Fehlerfälle

Material fehlt → Ausbau wartet. Ausbau während einer Welle → erlaubt, der Bauer baut nach der Gefahr.

## Akzeptanzkriterien

- **AC-01** Hub-Ausbau auf Stufe 2 bis 5 bucht Gold und Material laut `hub.json` ab; Gebäude höherer Stufen sind vor dem Ausbau nicht bezahlbar (Test) (B-112/AC-01, B-112/AC-03).
- **AC-02** Mauer und Turm werden am selben Platz auf Stufe 2 bis 5 ausgebaut, Kosten und HP laut Daten, Stufe 3 braucht Hub-Stufe 3 (Test) (B-112/AC-02).
- **AC-03** Zerstörung setzt die Stufe von Mauer und Turm zurück, die Hub-Stufe bleibt; Reparatur stellt HP zwischen den Wellen ohne Gold und Material her (Test) (B-112/AC-04, B-112/AC-05).
- **AC-04** Spielstand speichert Hub- und Gebäude-Stufen, Golden-Daten sind aktualisiert, `task check:go` grün (B-112/AC-06).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W1.1 | `W1.1-hub-ausbau.md` | Umsetzung | autonom | fertig |
| W1.2 | `W1.2-mauer-turm-stufen.md` | Umsetzung | autonom | fertig |
| W1.3 | `W1.3-zerstoerung-reparatur-spielstand.md` | Umsetzung | autonom | fertig |
| W1.4 | `W1.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
